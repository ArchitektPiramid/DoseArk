use std::sync::Arc;

use anyhow::Context;
use doseark_application::{AppServices, StorageBackupManager, StorageDiagnostics};
use doseark_db_postgres::{run_migrations as run_postgres_migrations, PostgresStorage};
use doseark_db_sqlite::{
    create_sqlite_pool, run_migrations as run_sqlite_migrations, SqliteStorage,
    SqliteStorageConfig,
};

use crate::{
    config::{ApiConfig, StorageBackend},
    database,
};

pub struct InitializedStorage {
    pub services: AppServices,
    pub diagnostics: Arc<dyn StorageDiagnostics>,
    pub backup: Option<Arc<dyn StorageBackupManager>>,
}

pub async fn initialize_storage(config: &ApiConfig) -> anyhow::Result<InitializedStorage> {
    match config.storage_backend {
        StorageBackend::Postgres => initialize_postgres(config).await,
        StorageBackend::Sqlite => initialize_sqlite(config).await,
    }
}

async fn initialize_postgres(config: &ApiConfig) -> anyhow::Result<InitializedStorage> {
    if config.database_url.trim().is_empty() {
        anyhow::bail!("DATABASE_URL must be set when DOSEARK_STORAGE=postgres");
    }

    let pool = database::connect(config).await?;
    run_postgres_migrations(&pool)
        .await
        .context("PostgreSQL migrations failed")?;

    let storage = PostgresStorage::new(pool);

    tracing::info!(storage = "postgresql", "Storage adapter initialized");

    Ok(InitializedStorage {
        services: AppServices::from_repositories(storage.repositories()),
        diagnostics: storage.diagnostics(),
        backup: None,
    })
}

async fn initialize_sqlite(config: &ApiConfig) -> anyhow::Result<InitializedStorage> {
    let sqlite_config = SqliteStorageConfig::new(&config.sqlite_path);

    tracing::info!(
        storage = "sqlite",
        path = %sqlite_config.path().display(),
        max_connections = sqlite_config.max_connections,
        busy_timeout_ms = u64::try_from(sqlite_config.busy_timeout.as_millis()).unwrap_or(u64::MAX),
        "Opening SQLite storage"
    );

    let pool = create_sqlite_pool(&sqlite_config)
        .await
        .with_context(|| {
            format!(
                "could not open SQLite database at {}",
                sqlite_config.path().display()
            )
        })?;

    run_sqlite_migrations(&pool)
        .await
        .context("SQLite migrations failed")?;

    let storage = SqliteStorage::new(pool, sqlite_config.path().display().to_string());

    tracing::info!(
        storage = "sqlite",
        path = %sqlite_config.path().display(),
        "Storage adapter initialized"
    );

    Ok(InitializedStorage {
        services: AppServices::from_repositories(storage.repositories()),
        diagnostics: storage.diagnostics(),
        backup: Some(storage.backup_manager()),
    })
}
