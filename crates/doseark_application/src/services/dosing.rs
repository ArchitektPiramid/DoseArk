//! Dose intake history and saved nutrient snapshot use cases.
use crate::{ApplicationError, ApplicationResult, DoseLogRepository, DoseLogNutrientRepository};
use doseark_domain::{DoseLog, DoseLogPage, DoseLogListInput, DoseLogNutrient, CreateDoseLogInput, UpdateDoseLogInput};
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
    pub async fn create_dose_log(
        &self,
        input: CreateDoseLogInput,
    ) -> ApplicationResult<DoseLog> {
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

/// Checks required intake fields before saving.
fn validate_dose_log(input: &CreateDoseLogInput) -> ApplicationResult<()> {
    validate_quantity_fields(input.units_taken, &input.unit, input.serving_multiplier)
}

/// Checks editable intake fields before updating.
fn validate_update_dose_log(input: &UpdateDoseLogInput) -> ApplicationResult<()> {
    validate_quantity_fields(input.units_taken, &input.unit, input.serving_multiplier)
}

/// Rejects invalid dose quantities before they reach persistence.
fn validate_quantity_fields(units: f64, unit: &str, multiplier: f64) -> ApplicationResult<()> {
    if !units.is_finite() || units <= 0.0 {
        return Err(ApplicationError::Validation(
            "Units taken must be finite and greater than zero".into(),
        ));
    }

    if unit.trim().is_empty() {
        return Err(ApplicationError::Validation("Dose unit is required".into()));
    }

    if !multiplier.is_finite() || multiplier <= 0.0 {
        return Err(ApplicationError::Validation(
            "Serving multiplier must be finite and greater than zero".into(),
        ));
    }

    Ok(())
}

/// Validates the client-provided fraction and returns the server-calculated value.
/// Call this with product data read inside the same database transaction as the dose write.
pub fn validated_serving_multiplier(
    units_taken: f64,
    unit: &str,
    requested_multiplier: f64,
    serving_size_units: f64,
    serving_unit: &str,
) -> ApplicationResult<f64> {
    validate_quantity_fields(units_taken, unit, requested_multiplier)?;

    if !serving_size_units.is_finite() || serving_size_units <= 0.0 {
        return Err(ApplicationError::Validation(
            "Product serving size must be finite and greater than zero".into(),
        ));
    }

    if serving_unit.trim().is_empty() || !unit.trim().eq_ignore_ascii_case(serving_unit.trim()) {
        return Err(ApplicationError::Validation(
            "Dose unit does not match the product serving unit".into(),
        ));
    }

    let calculated = units_taken / serving_size_units;
    if !calculated.is_finite() || calculated <= 0.0 {
        return Err(ApplicationError::Validation(
            "Calculated serving multiplier is outside the supported range".into(),
        ));
    }

    // Relative tolerance allows normal floating-point rounding, but not conflicting doses.
    let tolerance = 1e-9 * calculated.abs().max(requested_multiplier.abs());
    if (requested_multiplier - calculated).abs() > tolerance {
        return Err(ApplicationError::Validation(
            "Serving multiplier does not match the product serving size".into(),
        ));
    }

    Ok(calculated)
}

#[cfg(test)]
mod tests {
    use super::validated_serving_multiplier;

    #[test]
    fn derives_fractional_and_multiple_servings() {
        assert_eq!(
            validated_serving_multiplier(1.0, "tablet", 0.5, 2.0, "tablet").unwrap(),
            0.5,
        );
        assert_eq!(
            validated_serving_multiplier(3.0, "tablet", 1.5, 2.0, "tablet").unwrap(),
            1.5,
        );
    }

    #[test]
    fn tolerates_floating_point_rounding() {
        assert!(validated_serving_multiplier(
            1.0,
            " Tablet ",
            1.0 / 3.0 + 1e-12,
            3.0,
            "tablet",
        ).is_ok());
    }

    #[test]
    fn rejects_mismatched_multiplier_and_unit() {
        assert!(validated_serving_multiplier(1.0, "tablet", 100.0, 2.0, "tablet").is_err());
        assert!(validated_serving_multiplier(1.0, "capsule", 0.5, 2.0, "tablet").is_err());
    }

    #[test]
    fn rejects_invalid_numbers_and_overflow() {
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(validated_serving_multiplier(value, "tablet", 1.0, 2.0, "tablet").is_err());
            assert!(validated_serving_multiplier(1.0, "tablet", value, 2.0, "tablet").is_err());
            assert!(validated_serving_multiplier(1.0, "tablet", 1.0, value, "tablet").is_err());
        }
        assert!(
            validated_serving_multiplier(1e308, "tablet", 1.0, 1e-308, "tablet").is_err(),
        );
    }
}

// == Dose Log Nutrient Service ==
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
