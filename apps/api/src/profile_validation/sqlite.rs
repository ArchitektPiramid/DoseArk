//! Read-only SQLite profile connectivity and migration checks.

use std::time::Duration;
use doseark_runtime_config::ProfileConfig;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tokio::time::timeout;
use super::{error_check, schema_result, ValidationCheck};

pub(super) async fn validate_sqlite(profile: &ProfileConfig) -> Vec<ValidationCheck> {
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
