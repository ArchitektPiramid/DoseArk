//! Skład produktu i kontrola etykiet składników

use crate::{ApplicationError, ApplicationResult, ProductNutrientRepository};
use doseark_domain::{ProductNutrient, SaveProductNutrientInput};
use std::{collections::HashSet, sync::Arc};
use uuid::Uuid;

/// Validates and replaces product nutrient composition
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

    /// Checks the full replacement before passing it to storage
    pub async fn replace_product_nutrients(
        &self,
        product_id: Uuid,
        items: Vec<SaveProductNutrientInput>,
    ) -> ApplicationResult<Vec<ProductNutrient>> {
        validate_product_nutrients(&items)?;
        self.repo.replace_product_nutrients(product_id, items).await
    }
}

/// Rejects duplicate ingredients and invalid label values
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

#[cfg(test)]
mod tests {
    use super::validate_product_nutrients;
    use doseark_domain::SaveProductNutrientInput;
    use uuid::Uuid;

    // A complete composition must not repeat the same nutrient
    fn ingredient(id: Uuid, amount: f64) -> SaveProductNutrientInput {
        SaveProductNutrientInput {
            nutrient_id: id,
            amount_per_serving: amount,
            unit: "mg".into(),
            label_daily_value_percent: None,
            label_reference_amount: None,
            label_reference_unit: None,
            source: "manual".into(),
            notes: None,
        }
    }

    #[test]
    fn rejects_duplicate_nutrients() {
        let id = Uuid::new_v4();
        assert!(
            validate_product_nutrients(&[ingredient(id, 100.0), ingredient(id, 50.0)]).is_err()
        );
    }

    #[test]
    fn accepts_distinct_nutrients() {
        assert!(validate_product_nutrients(&[
            ingredient(Uuid::new_v4(), 100.0),
            ingredient(Uuid::new_v4(), 50.0),
        ])
        .is_ok());
    }

    #[test]
    fn rejects_non_positive_amounts() {
        assert!(validate_product_nutrients(&[ingredient(Uuid::new_v4(), 0.0)]).is_err());
    }
}
