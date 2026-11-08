use std::{
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use md5::{Digest as _, Md5};
use sha2::Sha256;

use crate::{task_failure::TaskFailure, tool_commands::run_tool};

const EICAR_TEST_TEXT: &[u8] =
    br"X5O!P%@AP[4\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*";
const FALSE_POSITIVE_DOCUMENT: &[u8] = b"Quarterly payroll macro template\nDepartment: Finance\nRows: 42\nThis document only formats the payroll table.\n";
const TOOLBAR_OFFER_MARKER: &[u8] = b"LAZARET-FIXTURE-TOOLBAR-OFFER";
const TOOLBAR_INSTALLER: &[u8] = b"Setup for the Search Toolbar\nLAZARET-FIXTURE-TOOLBAR-OFFER\nInstall the browser toolbar with sponsored results.\n";
const MISSED_DROPPER: &[u8] = b"LAZARET-FIXTURE-DROPPER-2026\nThis sample drops nothing; only the fixture signature of release two names it.\n";
const URLHAUS_FIXTURE_ADDRESS: &[u8] = b"http://malware.fixture.test/payload";
const ARCHIVED_SAMPLE_NAME: &str = "invoice.pdf.exe";
const ARCHIVE_SAMPLE_NAME: &str = "holiday photos.zip";
const CERTIFICATE_LIFETIME_DAYS: &str = "3650";

const FAKE_INTERNET_HOSTS: [&str; 7] = [
    "mb-api.abuse.ch",
    "hashlookup.circl.lu",
    "www.virustotal.com",
    "www.clamav.net",
    "clamav-site.s3.amazonaws.com",
    "urlhaus-api.abuse.ch",
    "signatures.fixture.test",
];

struct FixtureSample {
    file_name: &'static str,
    file_content: &'static [u8],
}

const FIXTURE_SAMPLES: [FixtureSample; 4] = [
    FixtureSample {
        file_name: ARCHIVED_SAMPLE_NAME,
        file_content: EICAR_TEST_TEXT,
    },
    FixtureSample {
        file_name: "payroll-macro.docm",
        file_content: FALSE_POSITIVE_DOCUMENT,
    },
    FixtureSample {
        file_name: "toolbar-setup.exe",
        file_content: TOOLBAR_INSTALLER,
    },
    FixtureSample {
        file_name: "dropper-2026.bin",
        file_content: MISSED_DROPPER,
    },
];

struct CorpusDirectories {
    samples: PathBuf,
    signatures: PathBuf,
    transport_security: PathBuf,
}

impl CorpusDirectories {
    fn create_under(output_directory: &Path) -> Result<Self, TaskFailure> {
        let corpus_directories = Self {
            samples: output_directory.join("samples"),
            signatures: output_directory.join("signatures"),
            transport_security: output_directory.join("tls"),
        };
        for release_directory in ["v1", "v2", "urlhaus"] {
            fs::create_dir_all(corpus_directories.signatures.join(release_directory))?;
        }
        fs::create_dir_all(&corpus_directories.samples)?;
        fs::create_dir_all(&corpus_directories.transport_security)?;
        Ok(corpus_directories)
    }
}

fn hash_signature_line(file_content: &[u8], signature_name: &str) -> String {
    format!(
        "{}:{}:{signature_name}\n",
        hex::encode(Md5::digest(file_content)),
        file_content.len()
    )
}

fn body_signature_line(marker_bytes: &[u8], signature_name: &str) -> String {
    format!("{signature_name}:0:*:{}\n", hex::encode(marker_bytes))
}

fn sha256_hex(file_content: &[u8]) -> String {
    hex::encode(Sha256::digest(file_content))
}

fn write_samples(samples_directory: &Path) -> Result<(), TaskFailure> {
    FIXTURE_SAMPLES.iter().try_for_each(|fixture_sample| {
        fs::write(
            samples_directory.join(fixture_sample.file_name),
            fixture_sample.file_content,
        )
    })?;
    let archive_staging = tempfile::tempdir()?;
    fs::write(
        archive_staging.path().join(ARCHIVED_SAMPLE_NAME),
        EICAR_TEST_TEXT,
    )?;
    run_tool(
        "zip",
        Command::new("zip")
            .current_dir(archive_staging.path())
            .arg("-X")
            .arg("-q")
            .arg(samples_directory.join(ARCHIVE_SAMPLE_NAME))
            .arg(ARCHIVED_SAMPLE_NAME),
    )?;
    let mut checksum_lines = String::new();
    let mut sample_names = fs::read_dir(samples_directory)?
        .map(|directory_entry| directory_entry.map(|sample_entry| sample_entry.file_name()))
        .collect::<Result<Vec<_>, _>>()?;
    sample_names.sort();
    for sample_name in sample_names {
        let sample_content = fs::read(samples_directory.join(&sample_name))?;
        let _infallible = writeln!(
            checksum_lines,
            "{}  {}",
            sha256_hex(&sample_content),
            sample_name.to_string_lossy()
        );
    }
    Ok(fs::write(
        samples_directory.join("SHA256SUMS"),
        checksum_lines,
    )?)
}

fn write_signatures(signatures_directory: &Path) -> Result<(), TaskFailure> {
    let eicar_line = hash_signature_line(EICAR_TEST_TEXT, "Win.Fixture.Agent-1");
    let false_positive_line =
        hash_signature_line(FALSE_POSITIVE_DOCUMENT, "Doc.Fixture.FalsePositive-1");
    let missed_malware_line = hash_signature_line(MISSED_DROPPER, "Win.Fixture.Missed-1");
    fs::write(
        signatures_directory.join("v1/lazaret-fixture.hdb"),
        format!("{eicar_line}{false_positive_line}"),
    )?;
    fs::write(
        signatures_directory.join("v2/lazaret-fixture.hdb"),
        format!("{eicar_line}{missed_malware_line}"),
    )?;
    fs::write(
        signatures_directory.join("lazaret-fixture.ndb"),
        body_signature_line(TOOLBAR_OFFER_MARKER, "PUA.Win.Fixture.Toolbar-1"),
    )?;
    let urlhaus_signatures = body_signature_line(URLHAUS_FIXTURE_ADDRESS, "Urlhaus.Fixture.Url-1");
    fs::write(
        signatures_directory.join("urlhaus/urlhaus_clamav.ndb.sha256"),
        sha256_hex(urlhaus_signatures.as_bytes()),
    )?;
    Ok(fs::write(
        signatures_directory.join("urlhaus/urlhaus_clamav.ndb"),
        urlhaus_signatures,
    )?)
}

fn write_transport_security(security_directory: &Path) -> Result<(), TaskFailure> {
    let signing_staging = tempfile::tempdir()?;
    let authority_key = signing_staging.path().join("fixture-ca.key");
    let authority_certificate = security_directory.join("fixture-ca.crt");
    let server_key = security_directory.join("fake-internet.key");
    let server_request = signing_staging.path().join("fake-internet.csr");
    let server_extensions = signing_staging.path().join("fake-internet.ext");
    run_tool(
        "openssl",
        Command::new("openssl")
            .args(["req", "-x509", "-newkey", "rsa:2048", "-nodes"])
            .args(["-days", CERTIFICATE_LIFETIME_DAYS])
            .args(["-subj", "/CN=Lazaret Fixture CA"])
            .args(["-addext", "basicConstraints=critical,CA:TRUE"])
            .args(["-addext", "keyUsage=critical,keyCertSign,cRLSign"])
            .arg("-keyout")
            .arg(&authority_key)
            .arg("-out")
            .arg(&authority_certificate),
    )?;
    run_tool(
        "openssl",
        Command::new("openssl")
            .args(["req", "-newkey", "rsa:2048", "-nodes"])
            .args(["-subj", "/CN=fake-internet"])
            .arg("-keyout")
            .arg(&server_key)
            .arg("-out")
            .arg(&server_request),
    )?;
    let alternative_names = FAKE_INTERNET_HOSTS
        .iter()
        .map(|host_name| format!("DNS:{host_name}"))
        .collect::<Vec<_>>()
        .join(",");
    fs::write(
        &server_extensions,
        [
            format!("subjectAltName={alternative_names}"),
            "basicConstraints=CA:FALSE".to_owned(),
            "extendedKeyUsage=serverAuth".to_owned(),
        ]
        .join("\n")
            + "\n",
    )?;
    run_tool(
        "openssl",
        Command::new("openssl")
            .args(["x509", "-req"])
            .args(["-days", CERTIFICATE_LIFETIME_DAYS])
            .arg("-in")
            .arg(&server_request)
            .arg("-CA")
            .arg(&authority_certificate)
            .arg("-CAkey")
            .arg(&authority_key)
            .arg("-CAcreateserial")
            .arg("-CAserial")
            .arg(signing_staging.path().join("fixture-ca.srl"))
            .arg("-extfile")
            .arg(&server_extensions)
            .arg("-out")
            .arg(security_directory.join("fake-internet.crt")),
    )
}

pub fn build_vm_fixture_corpus(output_directory: &Path) -> Result<(), TaskFailure> {
    let corpus_directories = CorpusDirectories::create_under(output_directory)?;
    write_samples(&corpus_directories.samples)?;
    write_signatures(&corpus_directories.signatures)?;
    write_transport_security(&corpus_directories.transport_security)?;
    Ok(fs::write(
        output_directory.join("fake-internet-hosts"),
        format!("127.0.0.1 {}\n", FAKE_INTERNET_HOSTS.join(" ")),
    )?)
}
