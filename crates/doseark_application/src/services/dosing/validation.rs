//! Walidacja ilości dawek oraz zgodności porcji produktu

use crate::{ApplicationError, ApplicationResult};
use doseark_domain::{CreateDoseLogInput, UpdateDoseLogInput};

/// Checks required intake fields before saving
pub(super) fn validate_dose_log(input: &CreateDoseLogInput) -> ApplicationResult<()> {
    validate_quantity_fields(input.units_taken, &input.unit, input.serving_multiplier)
}

/// Checks editable intake fields before updating
pub(super) fn validate_update_dose_log(input: &UpdateDoseLogInput) -> ApplicationResult<()> {
    validate_quantity_fields(input.units_taken, &input.unit, input.serving_multiplier)
}

/// Rejects invalid dose quantities before they reach persistence
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

/// Compares the client fraction with the serving size read inside the write transaction
/// Returns the server-calculated fraction for the nutrient snapshot
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

    // Relative tolerance allows normal floating-point rounding without accepting conflicting doses
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
        assert!(
            validated_serving_multiplier(1.0, " Tablet ", 1.0 / 3.0 + 1e-12, 3.0, "tablet",)
                .is_ok()
        );
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
        assert!(validated_serving_multiplier(1e308, "tablet", 1.0, 1e-308, "tablet").is_err(),);
    }
}

#[cfg(test)]
mod quantity_tests {
    use super::validate_quantity_fields;

    #[test]
    fn rejects_invalid_input_before_storage() {
        assert!(validate_quantity_fields(0.0, "tablet", 1.0).is_err());
        assert!(validate_quantity_fields(1.0, "  ", 1.0).is_err());
        assert!(validate_quantity_fields(1.0, "tablet", f64::NAN).is_err());
    }

    #[test]
    fn accepts_finite_positive_quantities() {
        assert!(validate_quantity_fields(2.0, "capsule", 0.5).is_ok());
    }
}
