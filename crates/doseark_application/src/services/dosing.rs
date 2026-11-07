//! Historia dawek i odczyt zapisanych składników

mod validation;

pub use self::validation::validated_serving_multiplier;
use self::validation::{validate_dose_log, validate_update_dose_log};

use crate::{ApplicationResult, DoseLogNutrientRepository, DoseLogRepository};
use doseark_domain::{
    CreateDoseLogInput, DoseLog, DoseLogListInput, DoseLogNutrient, DoseLogPage, UpdateDoseLogInput,
};
use std::sync::Arc;
use uuid::Uuid;

/// Validates intake requests and manages dose history.
#[derive(Clone)]
pub struct DoseLogService {
    repo: Arc<dyn DoseLogRepository>,
}

impl DoseLogService {
    pub fn new(repo: Arc<dyn DoseLogRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_dose_logs(&self, input: DoseLogListInput) -> ApplicationResult<DoseLogPage> {
        let limit = input.limit.clamp(1, 100);
        let offset = input.offset.max(0);

        self.repo
            .list_dose_logs(DoseLogListInput {
                limit,
                offset,
                product_id: input.product_id,
            })
            .await
    }

    /// Validates a new intake before repository persistence.
    pub async fn create_dose_log(&self, input: CreateDoseLogInput) -> ApplicationResult<DoseLog> {
        validate_dose_log(&input)?;
        self.repo.create_dose_log(input).await
    }

    pub async fn update_dose_log(
        &self,
        id: Uuid,
        input: UpdateDoseLogInput,
    ) -> ApplicationResult<DoseLog> {
        validate_update_dose_log(&input)?;
        self.repo.update_dose_log(id, input).await
    }

    /// Cancels a recorded intake through its repository.
    pub async fn cancel_dose_log(&self, id: Uuid) -> ApplicationResult<DoseLog> {
        self.repo.cancel_dose_log(id).await
    }
}

/// Reads nutrient snapshots saved with dose entries.
#[derive(Clone)]
pub struct DoseLogNutrientService {
    repo: Arc<dyn DoseLogNutrientRepository>,
}

impl DoseLogNutrientService {
    pub fn new(repo: Arc<dyn DoseLogNutrientRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_dose_log_nutrients(
        &self,
        dose_log_id: Uuid,
    ) -> ApplicationResult<Vec<DoseLogNutrient>> {
        self.repo.list_dose_log_nutrients(dose_log_id).await
    }
}
