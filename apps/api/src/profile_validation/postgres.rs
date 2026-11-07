//! PostgreSQL profile checks, including bounded connectivity and schema revision.

use std::time::Duration;
use doseark_runtime_config::{ProfileConfig, ProfileCredentials};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use tokio::time::timeout;
use super::{error_check, schema_result, ValidationCheck};

pub(super) async fn validate_postgres(
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

fn postgres_ssl_mode(value: &str) -> PgSslMode {
    match value.trim().to_ascii_lowercase().as_str() {
        "disable" => PgSslMode::Disable,
        "require" => PgSslMode::Require,
        _ => PgSslMode::Prefer,
    }
}
