//! Repozytorium historii dawek oraz operacji wpływających na stan magazynu

mod mapping;
mod read;
mod support;
mod write;

use async_trait::async_trait;
use doseark_application::{ApplicationResult, DoseLogRepository};
use doseark_domain::{
    CreateDoseLogInput, DoseLog, DoseLogListInput, DoseLogPage, UpdateDoseLogInput,
};
use sqlx::SqlitePool;
use uuid::Uuid;

/// SQLite-backed dose history and stock-aware write operations
#[derive(Clone)]
pub struct SqliteDoseLogRepository {
    pool: SqlitePool,
}

impl SqliteDoseLogRepository {
    /// Wraps the pool shared by dose history and transaction handlers
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DoseLogRepository for SqliteDoseLogRepository {
    async fn list_dose_logs(&self, input: DoseLogListInput) -> ApplicationResult<DoseLogPage> {
        read::list_dose_logs(&self.pool, input).await
    }

    async fn create_dose_log(&self, input: CreateDoseLogInput) -> ApplicationResult<DoseLog> {
        write::create_dose_log(&self.pool, input).await
    }

    async fn update_dose_log(
        &self,
        id: Uuid,
        input: UpdateDoseLogInput,
    ) -> ApplicationResult<DoseLog> {
        write::update_dose_log(&self.pool, id, input).await
    }

    async fn cancel_dose_log(&self, id: Uuid) -> ApplicationResult<DoseLog> {
        write::cancel_dose_log(&self.pool, id).await
    }
}
