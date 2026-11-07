mod cancel;
mod create;
mod history;
mod mapping;
mod nutrient_snapshot_writer;
mod queries;
mod serving;
mod stock;
mod update;

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
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DoseLogRepository for SqliteDoseLogRepository {
    async fn list_dose_logs(&self, input: DoseLogListInput) -> ApplicationResult<DoseLogPage> {
        history::list_dose_logs(&self.pool, input).await
    }

    async fn create_dose_log(&self, input: CreateDoseLogInput) -> ApplicationResult<DoseLog> {
        create::create_dose_log(&self.pool, input).await
    }

    async fn update_dose_log(
        &self,
        id: Uuid,
        input: UpdateDoseLogInput,
    ) -> ApplicationResult<DoseLog> {
        update::update_dose_log(&self.pool, id, input).await
    }

    async fn cancel_dose_log(&self, id: Uuid) -> ApplicationResult<DoseLog> {
        cancel::cancel_dose_log(&self.pool, id).await
    }
}
