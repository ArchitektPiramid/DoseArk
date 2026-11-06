//! Nutrient catalog and per-product composition use cases.
use crate::{ApplicationError, ApplicationResult, NutrientRepository, ProductNutrientRepository};
use doseark_domain::{Nutrient, ProductNutrient, CreateNutrientInput, UpdateNutrientInput, SaveProductNutrientInput};
use std::{collections::HashSet, sync::Arc};
use uuid::Uuid;

/// Validates nutrient definitions before storage.
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

// == Product Nutrient Service ==
/// Validates and replaces product nutrient composition.
#[derive(Clone)]
pub struct ProductCompositionService {
    repo: Arc<dyn ProductNutrientRepository>,
}

impl ProductCompositionService {
    pub fn new(repo: Arc<dyn ProductNutrientRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_product_nutrients(
        &self,
        product_id: Uuid,
    ) -> ApplicationResult<Vec<ProductNutrient>> {
        self.repo.list_product_nutrients(product_id).await
    }

    /// Validates a complete composition replacement.
    pub async fn replace_product_nutrients(
        &self,
        product_id: Uuid,
        items: Vec<SaveProductNutrientInput>,
    ) -> ApplicationResult<Vec<ProductNutrient>> {
        validate_product_nutrients(&items)?;
        self.repo.replace_product_nutrients(product_id, items).await
    }
}

/// Rejects duplicate ingredients and invalid label values.
fn validate_product_nutrients(items: &[SaveProductNutrientInput]) -> ApplicationResult<()> {
    let mut seen = HashSet::new();

    for item in items {
        if !seen.insert(item.nutrient_id) {
            return Err(ApplicationError::Validation(format!(
                "Duplicate nutrient in composition: {}",
                item.nutrient_id
            )));
        }

        if item.amount_per_serving <= 0.0 {
            return Err(ApplicationError::Validation(
                "Amount per serving must be greater than zero".into(),
            ));
        }

        if item.unit.trim().is_empty() {
            return Err(ApplicationError::Validation("Unit is required".into()));
        }

        if let Some(percent) = item.label_daily_value_percent {
            if percent < 0.0 {
                return Err(ApplicationError::Validation(
                    "Label daily value percent cannot be negative".into(),
                ));
            }
        }

        if let Some(amount) = item.label_reference_amount {
            if amount <= 0.0 {
                return Err(ApplicationError::Validation(
                    "Label reference amount must be greater than zero".into(),
                ));
            }
        }
    }

    Ok(())
}
