use std::{fs, path::PathBuf, time::Duration};

use rusqlite::{Connection, OpenFlags, OptionalExtension, params};

use crate::vm_journeys::harness::{journey_failure::JourneyFailure, system_commands::wait_until};

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

impl LazaretDatabase {
    pub fn for_home(home_directory: &str) -> Self {
        Self {
            database_path: PathBuf::from(home_directory).join(".local/share/lazaret/lazaret.db"),
        }
    }

    fn read_connection(&self) -> Result<Connection, JourneyFailure> {
        let database_connection = Connection::open_with_flags(
            &self.database_path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        database_connection.busy_timeout(BUSY_TIMEOUT)?;
        Ok(database_connection)
    }

    pub fn quarantined_signature(
        &self,
        path_fragment: &str,
    ) -> Result<Option<String>, JourneyFailure> {
        Ok(self
            .read_connection()?
            .query_row(
                "SELECT signature_name FROM quarantine JOIN quarantine_signatures USING \
                 (entry_id) WHERE instr(CAST(original_path AS TEXT), ?1) > 0",
                params![path_fragment],
                |signature_row| signature_row.get(0),
            )
            .optional()?)
    }

    pub fn wait_for_quarantined_signature(
        &self,
        path_fragment: &str,
    ) -> Result<String, JourneyFailure> {
        wait_until(
            &format!("a quarantine entry for {path_fragment}"),
            Duration::from_secs(60),
            || Ok(self.quarantined_signature(path_fragment)?.is_some()),
        )?;
        self.quarantined_signature(path_fragment)?.ok_or_else(|| {
            JourneyFailure::ExpectationFailed(format!("no quarantine entry for {path_fragment}"))
        })
    }

    pub fn quarantine_entry_count(&self) -> Result<i64, JourneyFailure> {
        Ok(self.read_connection()?.query_row(
            "SELECT COUNT(*) FROM quarantine",
            [],
            |count_row| count_row.get(0),
        )?)
    }

    pub fn quarantined_original_paths(&self) -> Result<Vec<Vec<u8>>, JourneyFailure> {
        let database_connection = self.read_connection()?;
        let mut path_statement =
            database_connection.prepare("SELECT original_path FROM quarantine")?;
        let original_paths = path_statement
            .query_map([], |path_row| path_row.get(0))?
            .collect::<Result<Vec<Vec<u8>>, _>>()?;
        Ok(original_paths)
    }

    pub fn stored_detection(
        &self,
        file_path: &str,
    ) -> Result<Option<StoredDetection>, JourneyFailure> {
        Ok(self
            .read_connection()?
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
            .optional()?)
    }

    pub fn wait_for_stored_detection(
        &self,
        file_path: &str,
    ) -> Result<StoredDetection, JourneyFailure> {
        wait_until(
            &format!("a stored detection of {file_path}"),
            Duration::from_secs(60),
            || Ok(self.stored_detection(file_path)?.is_some()),
        )?;
        self.stored_detection(file_path)?.ok_or_else(|| {
            JourneyFailure::ExpectationFailed(format!("no stored detection of {file_path}"))
        })
    }

    pub fn submission_status(
        &self,
        submission_key: &SubmissionKey<'_>,
    ) -> Result<Option<String>, JourneyFailure> {
        Ok(self
            .read_connection()?
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
            .optional()?)
    }

    pub fn wait_for_submission_status(
        &self,
        submission_key: &SubmissionKey<'_>,
        expected_status: SubmissionStatus,
        timeout: Duration,
    ) -> Result<(), JourneyFailure> {
        let waiting_outcome = wait_until(
            &format!(
                "the {:?} {:?} submission reaching {:?}",
                submission_key.submission_service, submission_key.submission_kind, expected_status
            ),
            timeout,
            || {
                Ok(self.submission_status(submission_key)?.as_deref()
                    == Some(expected_status.stored_text()))
            },
        );
        waiting_outcome.map_err(|_timed_out| {
            JourneyFailure::ExpectationFailed(format!(
                "the {:?} {:?} submission of {} stayed {:?}, expected {:?}",
                submission_key.submission_service,
                submission_key.submission_kind,
                submission_key.content_sha256,
                self.submission_status(submission_key).ok().flatten(),
                expected_status
            ))
        })
    }

    pub fn is_trusted_content(&self, content_sha256: &str) -> Result<bool, JourneyFailure> {
        Ok(self.read_connection()?.query_row(
            "SELECT COUNT(*) FROM trusted_contents WHERE content_sha256 = ?1",
            params![content_sha256],
            |count_row| count_row.get::<_, i64>(0),
        )? > 0)
    }

    pub fn update_activity_statuses(&self) -> Result<Vec<String>, JourneyFailure> {
        let database_connection = self.read_connection()?;
        let mut status_statement = database_connection.prepare(
            "SELECT activity_status FROM activity_entries WHERE activity_status LIKE 'update_%'",
        )?;
        let activity_statuses = status_statement
            .query_map([], |status_row| status_row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;
        Ok(activity_statuses)
    }

    pub fn activity_outputs(&self) -> Result<String, JourneyFailure> {
        let database_connection = self.read_connection()?;
        let mut output_statement =
            database_connection.prepare("SELECT activity_output FROM activity_entries")?;
        let activity_outputs = output_statement
            .query_map([], |output_row| output_row.get::<_, Option<String>>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(activity_outputs
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join("\n"))
    }

    pub fn raw_file_contents(&self) -> Result<Vec<u8>, JourneyFailure> {
        let mut database_bytes = fs::read(&self.database_path)?;
        let write_ahead_log = self.database_path.with_extension("db-wal");
        if let Ok(log_bytes) = fs::read(write_ahead_log) {
            database_bytes.extend(log_bytes);
        }
        Ok(database_bytes)
    }
}
