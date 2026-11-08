//! Backup filename validation and directory listing regression tests.

use super::{backup_format, is_safe_file_name, is_supported_backup_name};

#[test]
fn accepts_regular_backup_names_without_directories() {
    for name in [
        "doseark.db",
        "doseark.sqlite",
        "export.dump",
        "archive.sql.gz",
    ] {
        assert!(is_safe_file_name(name));
        assert!(is_supported_backup_name(name));
    }
    assert_eq!(backup_format("export.dump"), "postgres-custom");
}

#[test]
fn rejects_directory_traversal_and_unsupported_extensions() {
    for name in ["", "../private.sqlite", "some/path.db", r"some\path.db"] {
        assert!(!is_safe_file_name(name));
    }
    assert!(!is_supported_backup_name("secrets.txt"));
}

#[cfg(unix)]
#[tokio::test]
async fn listing_ignores_symlinked_backup_names() -> std::io::Result<()> {
    use std::os::unix::fs::symlink;

    let directory =
        std::env::temp_dir().join(format!("doseark-api-backups-{}", uuid::Uuid::new_v4()));
    tokio::fs::create_dir(&directory).await?;
    let target = directory.join("not-a-backup.txt");
    tokio::fs::write(&target, b"not a backup").await?;
    symlink(&target, directory.join("linked.dump"))?;

    let result = super::collect_backups(&directory).await;
    let cleanup = tokio::fs::remove_dir_all(&directory).await;
    let files = result?;
    cleanup?;
    assert!(files.is_empty());
    Ok(())
}
