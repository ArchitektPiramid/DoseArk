//! Diagnostyka i metadane aktywnej bazy SQLite

use async_trait::async_trait;
use doseark_application::{
    ApplicationError, ApplicationResult, StorageCapabilities, StorageDiagnostics, StorageEngine,
    StorageOverview, StorageRuntimeInfo, StorageTableStats,
};
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct SqliteDiagnostics {
    pool: SqlitePool,
    database_name: String,
}

impl SqliteDiagnostics {
    pub fn new(pool: SqlitePool, database_name: impl Into<String>) -> Self {
        Self {
            pool,
            database_name: database_name.into(),
        }
    }

    /// Reads the SQLite version and current database settings
    async fn query_runtime_info(&self) -> Result<StorageRuntimeInfo, sqlx::Error> {
        let engine_version: String = sqlx::query_scalar("SELECT sqlite_version()")
            .fetch_one(&self.pool)
            .await?;

        let (applied_migrations, applied_migration_revision): (i64, i64) = sqlx::query_as(
            r#"
            SELECT
                COUNT(*),
                COALESCE(MAX(version), 0)
            FROM _sqlx_migrations
            WHERE success = 1
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(StorageRuntimeInfo {
            engine: StorageEngine::Sqlite,
            database_name: self.database_name.clone(),
            engine_version,
            applied_migrations,
            applied_migration_revision,
        })
    }
}

#[async_trait]
impl StorageDiagnostics for SqliteDiagnostics {
    fn engine(&self) -> StorageEngine {
        StorageEngine::Sqlite
    }

    fn capabilities(&self) -> StorageCapabilities {
        StorageCapabilities {
            connection_stats: false,
            table_stats: false,
            backups: true,
            managed_backups: true,
        }
    }

    async fn runtime_info(&self) -> ApplicationResult<StorageRuntimeInfo> {
        self.query_runtime_info().await.map_err(map_sqlx_error)
    }

    /// Returns overall size and table statistics for the current database
    async fn overview(&self) -> ApplicationResult<StorageOverview> {
        let runtime = self.query_runtime_info().await.map_err(map_sqlx_error)?;
        let page_count: i64 = sqlx::query_scalar("PRAGMA page_count")
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx_error)?;
        let page_size: i64 = sqlx::query_scalar("PRAGMA page_size")
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(StorageOverview {
            runtime,
            database_size_bytes: page_count.saturating_mul(page_size),
            active_connections: None,
            max_connections: None,
            pool_size: Some(self.pool.size()),
            pool_idle: Some(self.pool.num_idle()),
            capabilities: self.capabilities(),
        })
    }

    async fn table_stats(&self) -> ApplicationResult<Vec<StorageTableStats>> {
        Ok(Vec::new())
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApplicationError {
    ApplicationError::Repository(error.to_string())
}
