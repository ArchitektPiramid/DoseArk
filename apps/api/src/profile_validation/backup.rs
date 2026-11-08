//! Backup directory write access probe for profile validation.

use doseark_runtime_config::ProfileConfig;
use uuid::Uuid;
use super::{error_check, ValidationCheck};

pub(super) async fn validate_backup_path(profile: &ProfileConfig) -> ValidationCheck {
    let Some(path) = profile.backup_path.as_ref() else {
        return ValidationCheck {
            key: "backup",
            status: "skipped",
            message: "No backupPath configured in the profile preview.".to_string(),
        };
    };

    if let Err(error) = tokio::fs::create_dir_all(path).await {
        return error_check("backup", format!("Backup directory cannot be created: {error}"));
    }

    let probe = path.join(format!(".doseark-write-check-{}", Uuid::new_v4()));
    match tokio::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&probe)
        .await
    {
        Ok(_) => {
            let _ = tokio::fs::remove_file(&probe).await;
            ValidationCheck {
                key: "backup",
                status: "ok",
                message: format!("Backup directory is writable: {}", path.display()),
            }
        }
        Err(error) => error_check(
            "backup",
            format!("Backup directory is not writable: {error}"),
        ),
    }
}
