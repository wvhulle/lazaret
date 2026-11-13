use std::{fs, path::PathBuf, time::Duration};

use rusqlite::{Connection, OpenFlags, OptionalExtension, params};

use crate::vm_journeys::harness::system_commands::{holds_within, poll_until};

const BUSY_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubmissionService {
    MalwareBazaar,
}

impl SubmissionService {
    const fn stored_text(self) -> &'static str {
        match self {
            Self::MalwareBazaar => "malware_bazaar",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubmissionKind {
    MalwareSample,
}

impl SubmissionKind {
    const fn stored_text(self) -> &'static str {
        match self {
            Self::MalwareSample => "malware_sample",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubmissionStatus {
    UploadAccepted,
}

impl SubmissionStatus {
    const fn stored_text(self) -> &'static str {
        match self {
            Self::UploadAccepted => "upload_accepted",
        }
    }
}

pub struct SubmissionKey<'hash> {
    pub content_sha256: &'hash str,
    pub submission_service: SubmissionService,
    pub submission_kind: SubmissionKind,
}

#[derive(Clone, Debug)]
pub struct StoredDetection {
    pub signature_name: String,
    pub detection_sources: String,
    pub resolution_kind: String,
}

pub struct LazaretDatabase {
    database_path: PathBuf,
}

fn read_outcome<Read>(read_description: &str, read_result: rusqlite::Result<Read>) -> Read {
    read_result.unwrap_or_else(|read_failure| {
        panic!("reading {read_description} from the Lazaret database failed: {read_failure}")
    })
}

impl LazaretDatabase {
    pub fn for_home(home_directory: &str) -> Self {
        Self {
            database_path: PathBuf::from(home_directory).join(".local/share/lazaret/lazaret.db"),
        }
    }

    fn read_connection(&self) -> rusqlite::Result<Connection> {
        let database_connection = Connection::open_with_flags(
            &self.database_path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        database_connection.busy_timeout(BUSY_TIMEOUT)?;
        Ok(database_connection)
    }

    fn signature_row(&self, path_fragment: &str) -> rusqlite::Result<Option<String>> {
        self.read_connection()?
            .query_row(
                "SELECT signature_name FROM quarantine JOIN quarantine_signatures USING \
                 (entry_id) WHERE instr(CAST(original_path AS TEXT), ?1) > 0",
                params![path_fragment],
                |signature_row| signature_row.get(0),
            )
            .optional()
    }

    pub fn quarantined_signature(&self, path_fragment: &str) -> Option<String> {
        read_outcome("a quarantined signature", self.signature_row(path_fragment))
    }

    pub fn wait_for_quarantined_signature(&self, path_fragment: &str) -> String {
        poll_until(
            &format!("a quarantine entry for {path_fragment}"),
            Duration::from_secs(60),
            || self.quarantined_signature(path_fragment),
        )
    }

    fn entry_count_row(&self) -> rusqlite::Result<i64> {
        self.read_connection()?
            .query_row("SELECT COUNT(*) FROM quarantine", [], |count_row| {
                count_row.get(0)
            })
    }

    pub fn quarantine_entry_count(&self) -> i64 {
        read_outcome("the quarantine entry count", self.entry_count_row())
    }

    fn original_path_rows(&self) -> rusqlite::Result<Vec<Vec<u8>>> {
        let database_connection = self.read_connection()?;
        let mut path_statement =
            database_connection.prepare("SELECT original_path FROM quarantine")?;
        path_statement
            .query_map([], |path_row| path_row.get(0))?
            .collect()
    }

    pub fn quarantined_original_paths(&self) -> Vec<Vec<u8>> {
        read_outcome("the quarantined original paths", self.original_path_rows())
    }

    fn detection_row(&self, file_path: &str) -> rusqlite::Result<Option<StoredDetection>> {
        self.read_connection()?
            .query_row(
                "SELECT signature_name, detection_sources, COALESCE(resolution_kind, '') FROM \
                 background_detections LEFT JOIN detection_resolutions USING (file_path, \
                 signature_name) WHERE CAST(file_path AS TEXT) = ?1",
                params![file_path],
                |detection_row| {
                    Ok(StoredDetection {
                        signature_name: detection_row.get(0)?,
                        detection_sources: detection_row.get(1)?,
                        resolution_kind: detection_row.get(2)?,
                    })
                },
            )
            .optional()
    }

    pub fn stored_detection(&self, file_path: &str) -> Option<StoredDetection> {
        read_outcome("a stored detection", self.detection_row(file_path))
    }

    pub fn wait_for_stored_detection(&self, file_path: &str) -> StoredDetection {
        poll_until(
            &format!("a stored detection of {file_path}"),
            Duration::from_secs(60),
            || self.stored_detection(file_path),
        )
    }

    fn submission_status_row(
        &self,
        submission_key: &SubmissionKey<'_>,
    ) -> rusqlite::Result<Option<String>> {
        self.read_connection()?
            .query_row(
                "SELECT submission_status FROM submissions WHERE content_sha256 = ?1 AND \
                 submission_service = ?2 AND submission_kind = ?3",
                params![
                    submission_key.content_sha256,
                    submission_key.submission_service.stored_text(),
                    submission_key.submission_kind.stored_text()
                ],
                |status_row| status_row.get(0),
            )
            .optional()
    }

    pub fn submission_status(&self, submission_key: &SubmissionKey<'_>) -> Option<String> {
        read_outcome(
            "a submission status",
            self.submission_status_row(submission_key),
        )
    }

    pub fn wait_for_submission_status(
        &self,
        submission_key: &SubmissionKey<'_>,
        expected_status: SubmissionStatus,
        timeout: Duration,
    ) {
        let reached_status = holds_within(timeout, || {
            self.submission_status(submission_key).as_deref() == Some(expected_status.stored_text())
        });
        assert!(
            reached_status,
            "the {:?} {:?} submission of {} stayed {:?}, expected {:?}",
            submission_key.submission_service,
            submission_key.submission_kind,
            submission_key.content_sha256,
            self.submission_status(submission_key),
            expected_status
        );
    }

    fn trusted_count_row(&self, content_sha256: &str) -> rusqlite::Result<i64> {
        self.read_connection()?.query_row(
            "SELECT COUNT(*) FROM trusted_contents WHERE content_sha256 = ?1",
            params![content_sha256],
            |count_row| count_row.get(0),
        )
    }

    pub fn is_trusted_content(&self, content_sha256: &str) -> bool {
        read_outcome(
            "the trusted contents",
            self.trusted_count_row(content_sha256),
        ) > 0
    }

    fn update_status_rows(&self) -> rusqlite::Result<Vec<String>> {
        let database_connection = self.read_connection()?;
        let mut status_statement = database_connection.prepare(
            "SELECT activity_status FROM activity_entries WHERE activity_status LIKE 'update_%'",
        )?;
        status_statement
            .query_map([], |status_row| status_row.get(0))?
            .collect()
    }

    pub fn update_activity_statuses(&self) -> Vec<String> {
        read_outcome("the update activity statuses", self.update_status_rows())
    }

    fn activity_output_rows(&self) -> rusqlite::Result<Vec<Option<String>>> {
        let database_connection = self.read_connection()?;
        let mut output_statement =
            database_connection.prepare("SELECT activity_output FROM activity_entries")?;
        output_statement
            .query_map([], |output_row| output_row.get(0))?
            .collect()
    }

    pub fn activity_outputs(&self) -> String {
        read_outcome("the activity outputs", self.activity_output_rows())
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn raw_file_contents(&self) -> Vec<u8> {
        let mut database_bytes = fs::read(&self.database_path).unwrap_or_else(|read_failure| {
            panic!(
                "{} is unreadable: {read_failure}",
                self.database_path.display()
            )
        });
        let write_ahead_log = self.database_path.with_extension("db-wal");
        if let Ok(log_bytes) = fs::read(write_ahead_log) {
            database_bytes.extend(log_bytes);
        }
        database_bytes
    }
}
