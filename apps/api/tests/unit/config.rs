//! Unit tests for API runtime configuration.

use std::path::{Path, PathBuf};

use super::{default_backup_dir, summarize_database_url, StorageBackend};

#[test]
fn hides_database_credentials() {
    assert_eq!(
        summarize_database_url("postgres://user:secret@db.local:5432/medtaker"),
        "db.local:5432/medtaker"
    );
}

#[test]
fn removes_query_parameters() {
    assert_eq!(
        summarize_database_url("postgres://localhost/medtaker?sslmode=disable"),
        "localhost/medtaker"
    );
}

#[test]
fn sqlite_backups_default_next_to_database() {
    assert_eq!(
        default_backup_dir(StorageBackend::Sqlite, Path::new("./data/doseark.sqlite")),
        PathBuf::from("./data/backups")
    );
}
