//! Obsługa partii magazynowych i kontrola ilości

use crate::{ApplicationError, ApplicationResult, InventoryLotRepository};
use doseark_domain::{CreateInventoryLotInput, InventoryLot, UpdateInventoryLotInput};
use std::sync::Arc;
use uuid::Uuid;

/// Validates inventory quantities before persisting lots
#[derive(Clone)]
pub struct InventoryLotService {
    repo: Arc<dyn InventoryLotRepository>,
}

impl InventoryLotService {
    pub fn new(repo: Arc<dyn InventoryLotRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_inventory_lots(&self) -> ApplicationResult<Vec<InventoryLot>> {
        self.repo.list_inventory_lots().await
    }

    /// Checks initial and remaining stock before creation
    pub async fn create_inventory_lot(
        &self,
        input: CreateInventoryLotInput,
    ) -> ApplicationResult<InventoryLot> {
        let units_remaining = input.units_remaining.unwrap_or(input.units_initial);

        validate_inventory_lot_numbers(input.units_initial, units_remaining, input.price_amount)?;

        self.repo.create_inventory_lot(input).await
    }

    /// Validates edited stock bounds before updating an existing lot
    pub async fn update_inventory_lot(
        &self,
        id: Uuid,
        input: UpdateInventoryLotInput,
    ) -> ApplicationResult<InventoryLot> {
        validate_inventory_lot_numbers(
            input.units_initial,
            input.units_remaining,
            input.price_amount,
        )?;

        self.repo.update_inventory_lot(id, input).await
    }
}

/// Ensures stock bounds and purchase price are consistent
fn validate_inventory_lot_numbers(
    units_initial: f64,
    units_remaining: f64,
    price_amount: Option<f64>,
) -> ApplicationResult<()> {
    if units_initial <= 0.0 {
        return Err(ApplicationError::Validation(
            "Initial units must be greater than zero".into(),
        ));
    }

    if units_remaining < 0.0 {
        return Err(ApplicationError::Validation(
            "Remaining units cannot be negative".into(),
        ));
    }

    if units_remaining > units_initial {
        return Err(ApplicationError::Validation(
            "Remaining units cannot be greater than initial units".into(),
        ));
    }

    if let Some(price) = price_amount {
        if price < 0.0 {
            return Err(ApplicationError::Validation(
                "Price amount cannot be negative".into(),
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_inventory_lot_numbers;

    #[test]
    fn accepts_valid_stock_bounds() {
        assert!(validate_inventory_lot_numbers(30.0, 0.0, None).is_ok());
        assert!(validate_inventory_lot_numbers(30.0, 15.0, Some(20.0)).is_ok());
    }

    #[test]
    fn rejects_invalid_stock_bounds() {
        assert!(validate_inventory_lot_numbers(0.0, 0.0, None).is_err());
        assert!(validate_inventory_lot_numbers(10.0, -1.0, None).is_err());
        assert!(validate_inventory_lot_numbers(10.0, 11.0, None).is_err());
        assert!(validate_inventory_lot_numbers(10.0, 5.0, Some(-2.0)).is_err());
    }
}
