use std::{fs, io, os::unix::fs::PermissionsExt, path::Path};

const SAMPLES_DIRECTORY: &str = "/etc/lazaret-test/samples";
const SAMPLE_PERMISSIONS: u32 = 0o640;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureSample {
    EicarInvoice,
    HolidayArchive,
    PayrollDocument,
    ToolbarInstaller,
}

impl FixtureSample {
    pub const fn file_name(self) -> &'static str {
        match self {
            Self::EicarInvoice => "invoice.pdf.exe",
            Self::HolidayArchive => "holiday photos.zip",
            Self::PayrollDocument => "payroll-macro.docm",
            Self::ToolbarInstaller => "toolbar-setup.exe",
        }
    }

    pub fn source_path(self) -> String {
        format!("{SAMPLES_DIRECTORY}/{}", self.file_name())
    }

    pub fn content_sha256(self) -> String {
        let checksum_path = format!("{SAMPLES_DIRECTORY}/SHA256SUMS");
        let checksum_listing = fs::read_to_string(&checksum_path)
            .unwrap_or_else(|read_failure| panic!("{checksum_path} is unreadable: {read_failure}"));
        checksum_listing
            .lines()
            .find_map(|checksum_line| {
                let (content_sha256, listed_name) = checksum_line.split_once("  ")?;
                (listed_name == self.file_name()).then(|| content_sha256.to_owned())
            })
            .unwrap_or_else(|| panic!("{checksum_path} has no checksum for {}", self.file_name()))
    }

    pub fn place_at(self, destination_path: impl AsRef<Path>) {
        let destination_path = destination_path.as_ref();
        copy_with_sample_permissions(&self.source_path(), destination_path).unwrap_or_else(
            |placement_failure| {
                panic!(
                    "placing {} at {} failed: {placement_failure}",
                    self.file_name(),
                    destination_path.display()
                )
            },
        );
    }
}

fn copy_with_sample_permissions(source_path: &str, destination_path: &Path) -> io::Result<()> {
    if let Some(destination_directory) = destination_path.parent() {
        fs::create_dir_all(destination_directory)?;
    }
    fs::copy(source_path, destination_path)?;
    fs::set_permissions(
        destination_path,
        fs::Permissions::from_mode(SAMPLE_PERMISSIONS),
    )
}
