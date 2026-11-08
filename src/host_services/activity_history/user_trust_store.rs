use std::collections::{BTreeMap, BTreeSet};

use clamd_configuration::trust_entry_names::{validated_sha256, validated_signature_name};
use jiff::Timestamp;
use turso::transaction::TransactionBehavior;

use crate::host_services::activity_history::activity_history_database::{
    ActivityHistory, ActivityHistoryFailure,
};

const UNREFERENCED_CONTENT_REMOVAL: &str =
    "DELETE FROM file_contents WHERE content_sha256 = ?1 AND NOT EXISTS (SELECT 1 FROM quarantine \
     WHERE quarantine.content_sha256 = ?1) AND NOT EXISTS (SELECT 1 FROM trusted_contents WHERE \
     trusted_contents.content_sha256 = ?1)";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UserTrustList {
    pub trusted_content_sizes: BTreeMap<String, u64>,
    pub ignored_signatures: BTreeSet<String>,
}

impl UserTrustList {
    pub fn is_empty(&self) -> bool {
        self.trusted_content_sizes.is_empty() && self.ignored_signatures.is_empty()
    }

    pub fn trusts_content(&self, sha256_hex: &str) -> bool {
        self.trusted_content_sizes
            .contains_key(&sha256_hex.to_ascii_lowercase())
    }

    pub fn trusts_content_of_size(&self, byte_size: u64) -> bool {
        self.trusted_content_sizes
            .values()
            .any(|trusted_size| *trusted_size == byte_size)
    }

    pub fn ignores_signature(&self, signature_name: &str) -> bool {
        validated_signature_name(signature_name)
            .is_ok_and(|normalised_name| self.ignored_signatures.contains(&normalised_name))
    }
}

impl ActivityHistory {
    pub async fn trust_content_for_user(
        &self,
        sha256_hex: &str,
        byte_size: u64,
    ) -> Result<(), ActivityHistoryFailure> {
        let trusted_sha256 = validated_sha256(sha256_hex)?;
        let byte_size = i64::try_from(byte_size)
            .map_err(|_oversized| ActivityHistoryFailure::ContentSizeUnstorable)?;
        let mut database_connection = self.open_connection().await?;
        let trust_transaction = database_connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .await?;
        trust_transaction
            .execute(
                "INSERT OR IGNORE INTO file_contents (content_sha256, byte_size) VALUES (?1, ?2)",
                (trusted_sha256.as_str(), byte_size),
            )
            .await?;
        trust_transaction
            .execute(
                "INSERT OR IGNORE INTO trusted_contents (content_sha256, trusted_at_unix) VALUES \
                 (?1, ?2)",
                (trusted_sha256.as_str(), Timestamp::now().as_second()),
            )
            .await?;
        trust_transaction.commit().await?;
        Ok(())
    }

    pub async fn forget_content_for_user(
        &self,
        sha256_hex: &str,
    ) -> Result<(), ActivityHistoryFailure> {
        let forgotten_sha256 = validated_sha256(sha256_hex)?;
        let mut database_connection = self.open_connection().await?;
        let forget_transaction = database_connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .await?;
        forget_transaction
            .execute(
                "DELETE FROM trusted_contents WHERE content_sha256 = ?1",
                (forgotten_sha256.as_str(),),
            )
            .await?;
        forget_transaction
            .execute(UNREFERENCED_CONTENT_REMOVAL, (forgotten_sha256.as_str(),))
            .await?;
        forget_transaction.commit().await?;
        Ok(())
    }

    pub async fn ignore_signature_for_user(
        &self,
        signature_name: &str,
    ) -> Result<(), ActivityHistoryFailure> {
        let ignored_name = validated_signature_name(signature_name)?;
        self.open_connection()
            .await?
            .execute(
                "INSERT OR IGNORE INTO ignored_signatures (signature_name, ignored_at_unix) \
                 VALUES (?1, ?2)",
                (ignored_name.as_str(), Timestamp::now().as_second()),
            )
            .await?;
        Ok(())
    }

    pub async fn stop_ignoring_signature_for_user(
        &self,
        signature_name: &str,
    ) -> Result<(), ActivityHistoryFailure> {
        let reported_name = validated_signature_name(signature_name)?;
        self.open_connection()
            .await?
            .execute(
                "DELETE FROM ignored_signatures WHERE signature_name = ?1",
                (reported_name.as_str(),),
            )
            .await?;
        Ok(())
    }

    pub async fn load_user_trust(&self) -> Result<UserTrustList, ActivityHistoryFailure> {
        let database_connection = self.open_connection().await?;
        let mut trusted_content_sizes = BTreeMap::new();
        let mut content_rows = database_connection
            .query(
                "SELECT trusted_contents.content_sha256, file_contents.byte_size FROM \
                 trusted_contents JOIN file_contents ON file_contents.content_sha256 = \
                 trusted_contents.content_sha256",
                (),
            )
            .await?;
        while let Some(content_row) = content_rows.next().await? {
            let stored_size = content_row.get::<i64>(1)?;
            trusted_content_sizes.insert(
                content_row.get::<String>(0)?,
                u64::try_from(stored_size)
                    .map_err(|_negative_size| ActivityHistoryFailure::StoredTrustUnreadable)?,
            );
        }
        let mut ignored_signatures = BTreeSet::new();
        let mut signature_rows = database_connection
            .query("SELECT signature_name FROM ignored_signatures", ())
            .await?;
        while let Some(signature_row) = signature_rows.next().await? {
            ignored_signatures.insert(signature_row.get::<String>(0)?);
        }
        Ok(UserTrustList {
            trusted_content_sizes,
            ignored_signatures,
        })
    }
}

#[cfg(test)]
mod user_trust_store_tests {
    use super::*;

    const EICAR_SHA256: &str = "275a021bbfb6489e54d471899f7db9d1663fc695ec2fe2a2c4538aabf651fd0f";

    fn temporary_history(temporary_root: &tempfile::TempDir) -> ActivityHistory {
        ActivityHistory::at_database_path(temporary_root.path().join("lazaret.db"))
    }

    #[tokio::test]
    async fn trusted_contents_and_ignored_signatures_round_trip() {
        let temporary_root = tempfile::tempdir().expect("temporary root");
        let activity_history = temporary_history(&temporary_root);
        activity_history
            .trust_content_for_user(&EICAR_SHA256.to_ascii_uppercase(), 68)
            .await
            .expect("trusted");
        activity_history
            .trust_content_for_user(EICAR_SHA256, 68)
            .await
            .expect("trusted twice");
        activity_history
            .ignore_signature_for_user("Sanesecurity.Foo.UNOFFICIAL")
            .await
            .expect("ignored");
        let user_trust = activity_history.load_user_trust().await.expect("loaded");
        assert_eq!(
            user_trust.trusted_content_sizes,
            BTreeMap::from([(EICAR_SHA256.to_owned(), 68)])
        );
        assert!(user_trust.trusts_content(&EICAR_SHA256.to_ascii_uppercase()));
        assert!(user_trust.trusts_content_of_size(68));
        assert!(user_trust.ignores_signature("Sanesecurity.Foo"));
        assert!(user_trust.ignores_signature("Sanesecurity.Foo.UNOFFICIAL"));
        activity_history
            .forget_content_for_user(EICAR_SHA256)
            .await
            .expect("forgotten");
        activity_history
            .stop_ignoring_signature_for_user("Sanesecurity.Foo")
            .await
            .expect("reported again");
        assert!(
            activity_history
                .load_user_trust()
                .await
                .expect("loaded")
                .is_empty()
        );
    }

    #[tokio::test]
    async fn heuristics_and_malformed_entries_are_refused() {
        let temporary_root = tempfile::tempdir().expect("temporary root");
        let activity_history = temporary_history(&temporary_root);
        assert!(matches!(
            activity_history
                .ignore_signature_for_user("Heuristics.Encrypted.Zip")
                .await,
            Err(ActivityHistoryFailure::TrustEntryRefused(_))
        ));
        assert!(matches!(
            activity_history.trust_content_for_user("abc", 3).await,
            Err(ActivityHistoryFailure::TrustEntryRefused(_))
        ));
    }

    #[test]
    fn the_schema_refuses_ignored_signatures_that_cannot_exist() {
        let temporary_root = tempfile::tempdir().expect("temporary root");
        let database_path = temporary_root.path().join("lazaret.db");
        tokio::runtime::Runtime::new()
            .expect("runtime")
            .block_on(temporary_history(&temporary_root).load_user_trust())
            .expect("migrated");
        let checked_connection = rusqlite::Connection::open(database_path).expect("sqlite");
        [
            "INSERT INTO ignored_signatures VALUES ('Heuristics.Encrypted.Zip', 0)",
            "INSERT INTO ignored_signatures VALUES ('Foo.UNOFFICIAL', 0)",
            "INSERT INTO ignored_signatures VALUES ('a b', 0)",
            "INSERT INTO trusted_contents VALUES ('00', 0)",
        ]
        .into_iter()
        .for_each(|invalid_insert| {
            assert!(
                checked_connection
                    .execute_batch(&format!("PRAGMA foreign_keys=ON; {invalid_insert}"))
                    .is_err(),
                "{invalid_insert}"
            );
        });
    }

    #[tokio::test]
    async fn quarantine_cleanup_keeps_trusted_contents() {
        let temporary_root = tempfile::tempdir().expect("temporary root");
        let activity_history = temporary_history(&temporary_root);
        activity_history
            .trust_content_for_user(EICAR_SHA256, 68)
            .await
            .expect("trusted");
        activity_history
            .open_connection()
            .await
            .expect("connection")
            .execute(
                "DELETE FROM file_contents WHERE NOT EXISTS (SELECT 1 FROM quarantine WHERE \
                 quarantine.content_sha256 = file_contents.content_sha256) AND NOT EXISTS (SELECT \
                 1 FROM trusted_contents WHERE trusted_contents.content_sha256 = \
                 file_contents.content_sha256)",
                (),
            )
            .await
            .expect("cleanup");
        assert!(
            activity_history
                .load_user_trust()
                .await
                .expect("loaded")
                .trusts_content(EICAR_SHA256)
        );
    }
}
