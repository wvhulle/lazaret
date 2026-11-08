use std::{
    fs, io,
    io::{Read, Write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use clamd_configuration::{
    protection_rules::ProtectionReduction,
    trust_entry_names::{TrustEntryRefusal, validated_sha256, validated_signature_name},
};

pub const TRUSTED_FILES_NAME: &str = "lazaret-local.sfp";
pub const IGNORED_SIGNATURES_NAME: &str = "lazaret-local.ign2";
const TRUSTED_FILE_LABEL: &str = "Lazaret.TrustedFile";
const PINNED_ENTRY_MD5_LENGTH: usize = 32;
const WORLD_READABLE_MODE: u32 = 0o644;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TrustRequest {
    TrustFile { sha256_hex: String, file_size: u64 },
    ForgetTrustedFile { sha256_hex: String },
    StopIgnoringSignature { signature_name: String },
}

#[derive(Debug, thiserror::Error)]
pub enum TrustRefusal {
    #[error(transparent)]
    EntryRefused(#[from] TrustEntryRefusal),
    #[error("an empty file can't be trusted")]
    EmptyFile,
    #[error("the trust list could not be updated: {0}")]
    FilesUnavailable(#[from] io::Error),
}

pub fn validated_request(trust_request: TrustRequest) -> Result<TrustRequest, TrustRefusal> {
    Ok(match trust_request {
        TrustRequest::TrustFile {
            sha256_hex,
            file_size,
        } => {
            if file_size == 0 {
                return Err(TrustRefusal::EmptyFile);
            }
            TrustRequest::TrustFile {
                sha256_hex: validated_sha256(&sha256_hex)?,
                file_size,
            }
        }
        TrustRequest::ForgetTrustedFile { sha256_hex } => TrustRequest::ForgetTrustedFile {
            sha256_hex: validated_sha256(&sha256_hex)?,
        },
        TrustRequest::StopIgnoringSignature { signature_name } => {
            TrustRequest::StopIgnoringSignature {
                signature_name: validated_signature_name(&signature_name)?,
            }
        }
    })
}

pub fn request_summary(trust_request: &TrustRequest) -> String {
    match trust_request {
        TrustRequest::TrustFile {
            sha256_hex,
            file_size,
        } => format!("trust the file with SHA-256 {sha256_hex} ({file_size} bytes)"),
        TrustRequest::ForgetTrustedFile { sha256_hex } => {
            format!("stop trusting the file with SHA-256 {sha256_hex}")
        }
        TrustRequest::StopIgnoringSignature { signature_name } => {
            format!("report detections named {signature_name} again")
        }
    }
}

pub const fn request_action_name(trust_request: &TrustRequest) -> &'static str {
    match trust_request {
        TrustRequest::TrustFile { .. } => "trust-file",
        TrustRequest::ForgetTrustedFile { .. } => "forget-trusted-file",
        TrustRequest::StopIgnoringSignature { .. } => "stop-ignoring-signature",
    }
}

pub const fn request_protection_reduction(trust_request: &TrustRequest) -> ProtectionReduction {
    match trust_request {
        TrustRequest::TrustFile { .. } => ProtectionReduction::DetectionTrusted,
        TrustRequest::ForgetTrustedFile { .. } | TrustRequest::StopIgnoringSignature { .. } => {
            ProtectionReduction::NoReduction
        }
    }
}

fn list_lines(list_text: &str) -> impl Iterator<Item = &str> {
    list_text
        .lines()
        .map(str::trim)
        .filter(|list_line| !list_line.is_empty() && !list_line.starts_with('#'))
}

fn trusted_hash_of_line(list_line: &str) -> &str {
    list_line.split(':').next().unwrap_or_default()
}

fn is_trusted_file_line(list_line: &str) -> bool {
    let mut line_fields = list_line.split(':');
    matches!(
        (line_fields.next(), line_fields.next(), line_fields.next(), line_fields.next()),
        (Some(hash_field), Some(size_field), Some(label_field), None)
            if validated_sha256(hash_field).is_ok()
                && size_field.parse::<u64>().is_ok()
                && validated_signature_name(label_field).is_ok()
    )
}

fn is_entry_md5(md5_text: &str) -> bool {
    md5_text.len() == PINNED_ENTRY_MD5_LENGTH
        && md5_text
            .bytes()
            .all(|md5_byte| md5_byte.is_ascii_hexdigit())
}

fn ignored_name_of_line(list_line: &str) -> Option<&str> {
    let (name_field, pinned_md5) = match list_line.split_once(':') {
        Some((name_field, pinned_md5)) => (name_field, Some(pinned_md5)),
        None => (list_line, None),
    };
    let name_is_valid = validated_signature_name(name_field)
        .is_ok_and(|signature_name| signature_name == name_field);
    (name_is_valid && pinned_md5.is_none_or(is_entry_md5)).then_some(name_field)
}

fn line_fits_list(list_line: &str, trust_request: &TrustRequest) -> bool {
    match trust_request {
        TrustRequest::TrustFile { .. } | TrustRequest::ForgetTrustedFile { .. } => {
            is_trusted_file_line(list_line)
        }
        TrustRequest::StopIgnoringSignature { .. } => ignored_name_of_line(list_line).is_some(),
    }
}

fn lines_after_request(list_text: &str, trust_request: &TrustRequest) -> String {
    let kept_lines = list_lines(list_text)
        .filter(|list_line| line_fits_list(list_line, trust_request))
        .filter(|list_line| match trust_request {
            TrustRequest::TrustFile { sha256_hex, .. }
            | TrustRequest::ForgetTrustedFile { sha256_hex } => {
                !trusted_hash_of_line(list_line).eq_ignore_ascii_case(sha256_hex)
            }
            TrustRequest::StopIgnoringSignature { signature_name } => {
                ignored_name_of_line(list_line) != Some(signature_name.as_str())
            }
        });
    let added_line = match trust_request {
        TrustRequest::TrustFile {
            sha256_hex,
            file_size,
        } => vec![format!("{sha256_hex}:{file_size}:{TRUSTED_FILE_LABEL}")],
        TrustRequest::ForgetTrustedFile { .. } | TrustRequest::StopIgnoringSignature { .. } => {
            Vec::new()
        }
    };
    kept_lines
        .map(str::to_owned)
        .chain(added_line)
        .map(|list_line| format!("{list_line}\n"))
        .collect()
}

pub fn list_file_for_request(database_directory: &Path, trust_request: &TrustRequest) -> PathBuf {
    database_directory.join(match trust_request {
        TrustRequest::TrustFile { .. } | TrustRequest::ForgetTrustedFile { .. } => {
            TRUSTED_FILES_NAME
        }
        TrustRequest::StopIgnoringSignature { .. } => IGNORED_SIGNATURES_NAME,
    })
}

fn read_list_or_empty(list_path: &Path) -> io::Result<String> {
    let opened_list = rustix::fs::open(
        list_path,
        rustix::fs::OFlags::RDONLY | rustix::fs::OFlags::NOFOLLOW | rustix::fs::OFlags::CLOEXEC,
        rustix::fs::Mode::empty(),
    );
    match opened_list {
        Ok(list_descriptor) => {
            let mut list_text = String::new();
            fs::File::from(list_descriptor).read_to_string(&mut list_text)?;
            Ok(list_text)
        }
        Err(rustix::io::Errno::NOENT) => Ok(String::new()),
        Err(rustix::io::Errno::LOOP) => Err(io::Error::other(format!(
            "{} is a symbolic link, which Lazaret never follows",
            list_path.display()
        ))),
        Err(open_failure) => Err(open_failure.into()),
    }
}

fn replace_list_file(list_path: &Path, list_text: &str) -> io::Result<()> {
    let list_directory = list_path
        .parent()
        .ok_or_else(|| io::Error::other("the trust list has no directory"))?;
    let mut staged_list = tempfile::Builder::new()
        .prefix(".lazaret-trust-")
        .suffix(".staged")
        .tempfile_in(list_directory)?;
    staged_list.write_all(list_text.as_bytes())?;
    staged_list.as_file().sync_all()?;
    fs::set_permissions(
        staged_list.path(),
        fs::Permissions::from_mode(WORLD_READABLE_MODE),
    )?;
    staged_list
        .persist(list_path)
        .map_err(|persist_failure| persist_failure.error)?;
    Ok(())
}

pub fn apply_trust_request(
    database_directory: &Path,
    trust_request: &TrustRequest,
) -> Result<(), TrustRefusal> {
    let list_path = list_file_for_request(database_directory, trust_request);
    let updated_list = lines_after_request(&read_list_or_empty(&list_path)?, trust_request);
    if updated_list.is_empty() {
        return match fs::remove_file(&list_path) {
            Ok(()) => Ok(()),
            Err(remove_failure) if remove_failure.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(remove_failure) => Err(remove_failure.into()),
        };
    }
    replace_list_file(&list_path, &updated_list)?;
    Ok(())
}

#[cfg(test)]
mod detection_trust_files_tests {
    use super::*;

    const EICAR_SHA256: &str = "275A021BBFB6489E54D471899F7DB9D1663FC695EC2FE2A2C4538AABF651FD0F";

    fn trust_eicar() -> TrustRequest {
        TrustRequest::TrustFile {
            sha256_hex: EICAR_SHA256.to_owned(),
            file_size: 68,
        }
    }

    #[test]
    fn hashes_are_normalized_and_malformed_ones_refused() {
        assert_eq!(
            validated_sha256(EICAR_SHA256).unwrap(),
            EICAR_SHA256.to_ascii_lowercase()
        );
        assert!(validated_sha256("abc").is_err());
        assert!(validated_sha256(&format!("{}\nx", &EICAR_SHA256[..63])).is_err());
    }

    #[test]
    fn signature_names_cannot_inject_lines_or_paths() {
        assert_eq!(
            validated_signature_name("Win.Trojan.Agent-123.UNOFFICIAL").unwrap(),
            "Win.Trojan.Agent-123"
        );
        [
            "Win.Trojan\nEicar",
            "../etc",
            "a b c",
            "-rf",
            "x",
            "Win..Trojan",
        ]
        .into_iter()
        .for_each(|unsafe_name| assert!(validated_signature_name(unsafe_name).is_err()));
    }

    #[test]
    fn engine_heuristics_must_be_trusted_by_hash() {
        assert!(matches!(
            validated_signature_name("Heuristics.Encrypted.Zip"),
            Err(TrustEntryRefusal::HeuristicSignature(_))
        ));
    }

    #[test]
    fn empty_files_are_never_trusted() {
        assert!(matches!(
            validated_request(TrustRequest::TrustFile {
                sha256_hex: EICAR_SHA256.to_owned(),
                file_size: 0,
            }),
            Err(TrustRefusal::EmptyFile)
        ));
    }

    #[test]
    fn trusting_twice_keeps_one_line_and_forgetting_removes_the_file() {
        let database_directory = tempfile::tempdir().unwrap();
        let trust_request = validated_request(trust_eicar()).unwrap();
        apply_trust_request(database_directory.path(), &trust_request).unwrap();
        apply_trust_request(database_directory.path(), &trust_request).unwrap();
        let list_path = database_directory.path().join(TRUSTED_FILES_NAME);
        assert_eq!(
            fs::read_to_string(&list_path).unwrap(),
            format!(
                "{}:68:{TRUSTED_FILE_LABEL}\n",
                EICAR_SHA256.to_ascii_lowercase()
            )
        );
        assert_eq!(
            fs::metadata(&list_path).unwrap().permissions().mode() & 0o777,
            WORLD_READABLE_MODE
        );
        apply_trust_request(
            database_directory.path(),
            &TrustRequest::ForgetTrustedFile {
                sha256_hex: EICAR_SHA256.to_ascii_lowercase(),
            },
        )
        .unwrap();
        assert!(!list_path.exists());
    }

    #[test]
    fn stop_ignoring_keeps_other_signatures() {
        let database_directory = tempfile::tempdir().unwrap();
        fs::write(
            database_directory.path().join(IGNORED_SIGNATURES_NAME),
            "Win.Test.First\nWin.Test.Second\n",
        )
        .unwrap();
        apply_trust_request(
            database_directory.path(),
            &TrustRequest::StopIgnoringSignature {
                signature_name: "Win.Test.First".to_owned(),
            },
        )
        .unwrap();
        assert_eq!(
            fs::read_to_string(database_directory.path().join(IGNORED_SIGNATURES_NAME)).unwrap(),
            "Win.Test.Second\n"
        );
    }

    #[test]
    fn signatures_can_no_longer_be_ignored_for_every_user() {
        let database_directory = tempfile::tempdir().unwrap();
        apply_trust_request(
            database_directory.path(),
            &TrustRequest::StopIgnoringSignature {
                signature_name: "Win.Test.Absent".to_owned(),
            },
        )
        .unwrap();
        assert!(
            !database_directory
                .path()
                .join(IGNORED_SIGNATURES_NAME)
                .exists()
        );
    }

    #[test]
    fn a_planted_symlink_in_place_of_the_list_is_neither_read_nor_written() {
        let database_directory = tempfile::tempdir().unwrap();
        let victim_file = database_directory.path().join("victim");
        fs::write(&victim_file, "secret:line\n").unwrap();
        std::os::unix::fs::symlink(
            &victim_file,
            database_directory.path().join(TRUSTED_FILES_NAME),
        )
        .unwrap();
        assert!(
            apply_trust_request(
                database_directory.path(),
                &validated_request(trust_eicar()).unwrap(),
            )
            .is_err()
        );
        assert_eq!(fs::read_to_string(&victim_file).unwrap(), "secret:line\n");
    }

    #[test]
    fn foreign_lines_in_the_list_are_dropped() {
        let database_directory = tempfile::tempdir().unwrap();
        fs::write(
            database_directory.path().join(IGNORED_SIGNATURES_NAME),
            "Win.Test.Kept\nWin.Test.Removed\nroot:x:0:0:root:/root:/bin/sh\n",
        )
        .unwrap();
        apply_trust_request(
            database_directory.path(),
            &TrustRequest::StopIgnoringSignature {
                signature_name: "Win.Test.Removed".to_owned(),
            },
        )
        .unwrap();
        assert_eq!(
            fs::read_to_string(database_directory.path().join(IGNORED_SIGNATURES_NAME)).unwrap(),
            "Win.Test.Kept\n"
        );
    }

    #[test]
    fn pinned_ignore_entries_are_kept_and_removed_by_their_name() {
        let database_directory = tempfile::tempdir().unwrap();
        let ignore_list = database_directory.path().join(IGNORED_SIGNATURES_NAME);
        fs::write(
            &ignore_list,
            "Win.Test.Pinned:44d88612fea8a8f36de82e1278abb02f\nWin.Test.Kept\nWin.Test.BadPin:\
             not-an-md5\n",
        )
        .unwrap();
        apply_trust_request(
            database_directory.path(),
            &TrustRequest::StopIgnoringSignature {
                signature_name: "Win.Test.Pinned".to_owned(),
            },
        )
        .unwrap();
        assert_eq!(fs::read_to_string(&ignore_list).unwrap(), "Win.Test.Kept\n");
    }
}
