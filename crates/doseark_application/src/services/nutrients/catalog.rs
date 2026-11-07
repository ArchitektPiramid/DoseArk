//! Katalog składników i walidacja ich podstawowych pól

use crate::{ApplicationError, ApplicationResult, NutrientRepository};
use doseark_domain::{CreateNutrientInput, Nutrient, UpdateNutrientInput};
use std::sync::Arc;
use uuid::Uuid;

/// Validates nutrient definitions before storage
#[derive(Clone)]
pub struct NutrientService {
    repo: Arc<dyn NutrientRepository>,
}

impl NutrientService {
    pub fn new(repo: Arc<dyn NutrientRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_nutrients(&self) -> ApplicationResult<Vec<Nutrient>> {
        self.repo.list_nutrients().await
    }

    pub async fn create_nutrient(&self, input: CreateNutrientInput) -> ApplicationResult<Nutrient> {
        validate_nutrient(
            &input.canonical_name,
            &input.display_name,
            &input.default_unit,
        )?;
        self.repo.create_nutrient(input).await
    }

    pub async fn update_nutrient(
        &self,
        id: Uuid,
        input: UpdateNutrientInput,
    ) -> ApplicationResult<Nutrient> {
        validate_nutrient(
            &input.canonical_name,
            &input.display_name,
            &input.default_unit,
        )?;
        self.repo.update_nutrient(id, input).await
    }
}

/// Requires the basic nutrient identity and unit.
fn validate_nutrient(
    canonical_name: &str,
    display_name: &str,
    default_unit: &str,
) -> ApplicationResult<()> {
    if canonical_name.trim().is_empty() {
        return Err(ApplicationError::Validation(
            "Nutrient canonical name is required".into(),
        ));
    }

    if display_name.trim().is_empty() {
        return Err(ApplicationError::Validation(
            "Nutrient display name is required".into(),
        ));
    }

    if default_unit.trim().is_empty() {
        return Err(ApplicationError::Validation(
            "Nutrient default unit is required".into(),
        ));
    }

    Ok(())
}
