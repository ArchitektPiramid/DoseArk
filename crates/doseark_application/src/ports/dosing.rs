//! Porty historii dawek oraz zapisanych składników

use crate::ApplicationResult;
use async_trait::async_trait;
use doseark_domain::{
    CreateDoseLogInput, DoseLog, DoseLogListInput, DoseLogNutrient, DoseLogPage, UpdateDoseLogInput,
};
use uuid::Uuid;

/// Records, edits, cancels and lists doses
#[async_trait]
pub trait DoseLogRepository: Send + Sync {
    async fn list_dose_logs(&self, input: DoseLogListInput) -> ApplicationResult<DoseLogPage>;

    async fn create_dose_log(&self, input: CreateDoseLogInput) -> ApplicationResult<DoseLog>;

    async fn update_dose_log(
        &self,
        id: Uuid,
        input: UpdateDoseLogInput,
    ) -> ApplicationResult<DoseLog>;

    async fn cancel_dose_log(&self, id: Uuid) -> ApplicationResult<DoseLog>;
}

/// Loads nutrient snapshots for recorded doses
#[async_trait]
pub trait DoseLogNutrientRepository: Send + Sync {
    async fn list_dose_log_nutrients(
        &self,
        dose_log_id: Uuid,
    ) -> ApplicationResult<Vec<DoseLogNutrient>>;
}
