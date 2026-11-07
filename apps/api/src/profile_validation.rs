//! Runtime profile validation for HTTP administration endpoints.
//!
//! Dispatches profile checks by storage backend and reports diagnostic results.
//! Each database probe has its own timeout; secret values are never returned.

mod backup;
mod credentials;
mod postgres;
mod sqlite;

use doseark_application::EXPECTED_MIGRATION_REVISION;
use doseark_runtime_config::{CredentialsResolver, ProfileConfig};

use self::{
    backup::validate_backup_path,
    credentials::{resolve_credentials, validate_credentials},
    postgres::validate_postgres,
    sqlite::validate_sqlite,
};

/// Named outcome of a profile diagnostic probe.
#[derive(Debug, Clone)]
pub struct ValidationCheck {
    pub key: &'static str,
    pub status: &'static str,
    pub message: String,
}

pub async fn validate_profile(
    profile: &ProfileConfig,
    credentials_resolver: &CredentialsResolver,
) -> Vec<ValidationCheck> {
    let mut checks = vec![ValidationCheck {
        key: "profile",
        status: "ok",
        message: format!("Profile '{}' loaded (mode {}).", profile.name, profile.mode),
    }];

    if profile.mode == "tauri-local" {
        checks.push(ValidationCheck {
            key: "runtime",
            status: "warning",
            message: "tauri-local paths belong to the future Tauri runtime; the HTTP API cannot validate the client filesystem yet.".to_string(),
        });
        return checks;
    }

    match profile.database.database_type.trim().to_ascii_lowercase().as_str() {
        "sqlite" => {
            checks.push(validate_credentials(profile, credentials_resolver, false).await);
            checks.extend(validate_sqlite(profile).await);
        }
        "postgres" | "postgresql" => {
            let credentials = resolve_credentials(profile, credentials_resolver).await;
            checks.push(credentials.check);
            checks.extend(validate_postgres(profile, credentials.value).await);
        }
        "auto" => checks.push(ValidationCheck {
            key: "database",
            status: "warning",
            message: "Database type is auto; select SQLite or PostgreSQL to validate a concrete profile target.".to_string(),
        }),
        other => checks.push(ValidationCheck {
            key: "database",
            status: "error",
            message: format!("Unsupported database type '{other}'."),
        }),
    }

    checks.push(validate_backup_path(profile).await);
    checks
}

fn schema_result(revision: i64) -> ValidationCheck {
    if revision == EXPECTED_MIGRATION_REVISION {
        ValidationCheck {
            key: "schema",
            status: "ok",
            message: format!(
                "Schema migration revision {revision}/{EXPECTED_MIGRATION_REVISION} is compatible."
            ),
        }
    } else {
        error_check(
            "schema",
            format!(
                "Schema migration revision {revision}/{EXPECTED_MIGRATION_REVISION} is not compatible."
            ),
        )
    }
}

fn error_check(key: &'static str, message: impl Into<String>) -> ValidationCheck {
    ValidationCheck {
        key,
        status: "error",
        message: message.into(),
    }
}

#[cfg(test)]
mod tests;
