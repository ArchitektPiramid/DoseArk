//! Operacje tworzenia i przywracania kopii bazy SQLite

use std::path::Path;

use async_trait::async_trait;
use doseark_application::{
    ApplicationError, ApplicationResult, StorageBackupManager, StorageEngine,
};
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct SqliteBackupManager {
    pool: SqlitePool,
}

impl SqliteBackupManager {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StorageBackupManager for SqliteBackupManager {
    fn engine(&self) -> StorageEngine {
        StorageEngine::Sqlite
    }

    /// Uses SQLite VACUUM INTO to create a consistent backup file
    async fn create_backup(&self, target: &Path) -> ApplicationResult<()> {
        if let Some(parent) = target.parent().filter(|path| !path.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent)
                .map_err(|error| ApplicationError::Repository(error.to_string()))?;
        }

        let target = target
            .to_str()
            .ok_or_else(|| ApplicationError::Validation("Backup path is not valid UTF-8".into()))?;

        // VACUUM INTO creates a consistent standalone SQLite snapshot even while
        // the active database is using WAL mode.
        sqlx::query("VACUUM INTO ?")
            .bind(target)
            .execute(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(())
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApplicationError {
    ApplicationError::Repository(error.to_string())
}
