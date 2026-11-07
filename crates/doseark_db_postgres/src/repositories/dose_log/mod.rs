//! PostgreSQL dose history and transactional intake persistence.
//!
//! The repository delegates reads and stock-affecting writes to focused modules;
//! public application port signatures remain unchanged.

mod mapping;
mod read;
mod support;
mod write;

use async_trait::async_trait;
use doseark_application::{ApplicationResult, DoseLogRepository};
use doseark_domain::{
    CreateDoseLogInput, DoseLog, DoseLogListInput, DoseLogPage, UpdateDoseLogInput,
};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgDoseLogRepository {
    pool: PgPool,
}

impl PgDoseLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DoseLogRepository for PgDoseLogRepository {
    async fn list_dose_logs(&self, input: DoseLogListInput) -> ApplicationResult<DoseLogPage> {
        read::list_dose_logs(&self.pool, input).await
    }

    async fn create_dose_log(&self, input: CreateDoseLogInput) -> ApplicationResult<DoseLog> {
        write::create::create_dose_log(&self.pool, input).await
    }

    async fn update_dose_log(
        &self,
        id: Uuid,
        input: UpdateDoseLogInput,
    ) -> ApplicationResult<DoseLog> {
        write::update::update_dose_log(&self.pool, id, input).await
    }

    async fn cancel_dose_log(&self, id: Uuid) -> ApplicationResult<DoseLog> {
        write::cancel::cancel_dose_log(&self.pool, id).await
    }
}
