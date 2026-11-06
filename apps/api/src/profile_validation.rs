use std::time::Duration;

use doseark_application::EXPECTED_MIGRATION_REVISION;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use tokio::time::timeout;
use uuid::Uuid;

use doseark_runtime_config::{CredentialsResolver, ProfileConfig, ProfileCredentials};

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

struct CredentialsResolution {
    check: ValidationCheck,
    value: Option<ProfileCredentials>,
}

async fn resolve_credentials(
    profile: &ProfileConfig,
    resolver: &CredentialsResolver,
) -> CredentialsResolution {
    let Some(reference) = profile.credentials_ref.as_deref() else {
        return CredentialsResolution {
            check: ValidationCheck {
                key: "credentials",
                status: "warning",
                message: "No credentialsRef configured; PostgreSQL validation will try without file credentials.".to_string(),
            },
            value: None,
        };
    };

    match resolver.resolve(reference).await {
        Ok(value) => CredentialsResolution {
            check: credentials_success(reference, resolver, &value),
            value: Some(value),
        },
        Err(error) => CredentialsResolution {
            check: ValidationCheck {
                key: "credentials",
                status: "error",
                message: format!("credentialsRef '{reference}' could not be resolved: {error}"),
            },
            value: None,
        },
    }
}

async fn validate_credentials(
    profile: &ProfileConfig,
    resolver: &CredentialsResolver,
    required: bool,
) -> ValidationCheck {
    let Some(reference) = profile.credentials_ref.as_deref() else {
        return ValidationCheck {
            key: "credentials",
            status: if required { "warning" } else { "skipped" },
            message: if required {
                "No credentialsRef configured.".to_string()
            } else {
                "SQLite does not require credentials.".to_string()
            },
        };
    };

    match resolver.resolve(reference).await {
        Ok(value) => credentials_success(reference, resolver, &value),
        Err(error) => ValidationCheck {
            key: "credentials",
            status: "error",
            message: format!("credentialsRef '{reference}' could not be resolved: {error}"),
        },
    }
}

fn credentials_success(
    reference: &str,
    resolver: &CredentialsResolver,
    value: &ProfileCredentials,
) -> ValidationCheck {
    ValidationCheck {
        key: "credentials",
        status: "ok",
        message: format!(
            "credentialsRef '{reference}' resolved server-side from {} (username {}, password {}, API token {}).",
            resolver.path().display(),
            presence(value.username.as_deref()),
            presence(value.password.as_deref()),
            presence(value.api_token.as_deref())
        ),
    }
}

async fn validate_sqlite(profile: &ProfileConfig) -> Vec<ValidationCheck> {
    let mut checks = Vec::new();
    let Some(path) = profile.database.path.as_ref() else {
        checks.push(error_check("database", "SQLite profile has no database path."));
        return checks;
    };

    if !path.exists() {
        checks.push(error_check(
            "database",
            format!("SQLite file does not exist: {}", path.display()),
        ));
        return checks;
    }

    let options = SqliteConnectOptions::new()
        .filename(path)
        .read_only(true)
        .create_if_missing(false);

    let pool = match timeout(
        Duration::from_secs(5),
        SqlitePoolOptions::new().max_connections(1).connect_with(options),
    )
    .await
    {
        Ok(Ok(pool)) => pool,
        Ok(Err(error)) => {
            checks.push(error_check(
                "database",
                format!("SQLite could not be opened read-only: {error}"),
            ));
            return checks;
        }
        Err(_) => {
            checks.push(error_check(
                "database",
                "SQLite validation timed out after 5 seconds.",
            ));
            return checks;
        }
    };

    checks.push(ValidationCheck {
        key: "database",
        status: "ok",
        message: format!("SQLite opened successfully: {}", path.display()),
    });
    checks.push(schema_check_sqlite(&pool).await);
    pool.close().await;
    checks
}

async fn schema_check_sqlite(pool: &sqlx::SqlitePool) -> ValidationCheck {
    match sqlx::query_scalar::<_, i64>(
        "SELECT COALESCE(MAX(version), 0) FROM _sqlx_migrations WHERE success = 1",
    )
    .fetch_one(pool)
    .await
    {
        Ok(revision) => schema_result(revision),
        Err(error) => error_check(
            "schema",
            format!("Could not inspect SQLite schema migrations: {error}"),
        ),
    }
}

async fn validate_postgres(
    profile: &ProfileConfig,
    credentials: Option<ProfileCredentials>,
) -> Vec<ValidationCheck> {
    let mut checks = Vec::new();
    let Some(host) = profile.database.host.as_deref().filter(|value| !value.trim().is_empty()) else {
        checks.push(error_check("database", "PostgreSQL profile has no host."));
        return checks;
    };
    let database_name = profile.database.name.trim();
    if database_name.is_empty() {
        checks.push(error_check("database", "PostgreSQL profile has no database name."));
        return checks;
    }

    let mut options = PgConnectOptions::new()
        .host(host)
        .port(profile.database.port.unwrap_or(5432))
        .database(database_name)
        .ssl_mode(postgres_ssl_mode(&profile.database.ssl_mode));

    if let Some(credentials) = credentials {
        if let Some(username) = credentials.username.as_deref().filter(|value| !value.is_empty()) {
            options = options.username(username);
        }
        if let Some(password) = credentials.password.as_deref() {
            options = options.password(password);
        }
    }

    let pool = match timeout(
        Duration::from_secs(5),
        PgPoolOptions::new().max_connections(1).connect_with(options),
    )
    .await
    {
        Ok(Ok(pool)) => pool,
        Ok(Err(error)) => {
            checks.push(error_check(
                "database",
                format!("PostgreSQL connection failed: {error}"),
            ));
            return checks;
        }
        Err(_) => {
            checks.push(error_check(
                "database",
                "PostgreSQL validation timed out after 5 seconds.",
            ));
            return checks;
        }
    };

    checks.push(ValidationCheck {
        key: "database",
        status: "ok",
        message: format!(
            "PostgreSQL responded at {}:{}/{}.",
            host,
            profile.database.port.unwrap_or(5432),
            database_name
        ),
    });
    checks.push(schema_check_postgres(&pool).await);
    pool.close().await;
    checks
}

async fn schema_check_postgres(pool: &sqlx::PgPool) -> ValidationCheck {
    match sqlx::query_scalar::<_, i64>(
        "SELECT COALESCE(MAX(version), 0)::BIGINT FROM _sqlx_migrations WHERE success = TRUE",
    )
    .fetch_one(pool)
    .await
    {
        Ok(revision) => schema_result(revision),
        Err(error) => error_check(
            "schema",
            format!("Could not inspect PostgreSQL schema migrations: {error}"),
        ),
    }
}

async fn validate_backup_path(profile: &ProfileConfig) -> ValidationCheck {
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

fn postgres_ssl_mode(value: &str) -> PgSslMode {
    match value.trim().to_ascii_lowercase().as_str() {
        "disable" => PgSslMode::Disable,
        "require" => PgSslMode::Require,
        _ => PgSslMode::Prefer,
    }
}

fn presence(value: Option<&str>) -> &'static str {
    if value.is_some_and(|value| !value.is_empty()) {
        "set"
    } else {
        "not set"
    }
}
