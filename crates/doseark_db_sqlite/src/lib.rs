//! SQLite persistence adapter for DoseArk.
//!
//! Provides SQLite connection configuration, migrations, diagnostics, and the
//! repository implementations for catalog, storage locations, inventory, stock
//! movements, dosing, and reports. The adapter can back the complete HTTP API
//! runtime without a PostgreSQL server.

pub mod diagnostics;
pub mod backup;
pub mod repositories;
pub mod storage;
mod sqlite_types;

use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};

pub use diagnostics::*;
pub use backup::*;
pub use repositories::*;
pub use storage::*;

#[derive(Debug, Clone)]
pub struct SqliteStorageConfig {
    pub path: PathBuf,
    pub max_connections: u32,
    pub busy_timeout: Duration,
}

impl SqliteStorageConfig {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            max_connections: 4,
            busy_timeout: Duration::from_secs(5),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

pub async fn create_sqlite_pool(config: &SqliteStorageConfig) -> Result<SqlitePool, sqlx::Error> {
    if let Some(parent) = config.path.parent().filter(|path| !path.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(sqlx::Error::Io)?;
    }

    let options = SqliteConnectOptions::new()
        .filename(&config.path)
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(config.busy_timeout);

    SqlitePoolOptions::new()
        .max_connections(config.max_connections.max(1))
        .connect_with(options)
        .await
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::migrate::MigrateError> {
    tracing::info!("Running SQLite migrations");

    sqlx::migrate!("./migrations").run(pool).await?;

    tracing::info!("SQLite migrations finished");
    Ok(())
}
