//! Inventory, stock movements and storage location use cases.
use crate::{ApplicationError, ApplicationResult, InventoryLotRepository, StockMovementRepository, StorageLocationRepository};
use doseark_domain::{InventoryLot, StockMovement, StorageLocation, CreateInventoryLotInput, UpdateInventoryLotInput, CreateStorageLocationInput, UpdateStorageLocationInput};
use std::sync::Arc;
use uuid::Uuid;

/// Validates inventory quantities before persisting lots.
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

    /// Checks initial and remaining stock before creation.
    pub async fn create_inventory_lot(
        &self,
        input: CreateInventoryLotInput,
    ) -> ApplicationResult<InventoryLot> {
        let units_remaining = input.units_remaining.unwrap_or(input.units_initial);

        validate_inventory_lot_numbers(input.units_initial, units_remaining, input.price_amount)?;

        self.repo.create_inventory_lot(input).await
    }

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

// == Stock Movement Service ==
/// Reads recorded stock movements for inventory lots.
#[derive(Clone)]
pub struct StockMovementService {
    repo: Arc<dyn StockMovementRepository>,
}

impl StockMovementService {
    pub fn new(repo: Arc<dyn StockMovementRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_stock_movements(&self) -> ApplicationResult<Vec<StockMovement>> {
        self.repo.list_stock_movements().await
    }

    pub async fn list_stock_movements_for_lot(
        &self,
        inventory_lot_id: Uuid,
    ) -> ApplicationResult<Vec<StockMovement>> {
        self.repo
            .list_stock_movements_for_lot(inventory_lot_id)
            .await
    }
}

/// Manages and archives locations used by inventory.
#[derive(Clone)]
pub struct StorageLocationService {
    repo: Arc<dyn StorageLocationRepository>,
}

impl StorageLocationService {
    pub fn new(repo: Arc<dyn StorageLocationRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_storage_locations(
        &self,
        include_archived: bool,
    ) -> ApplicationResult<Vec<StorageLocation>> {
        self.repo.list_storage_locations(include_archived).await
    }

    pub async fn create_storage_location(
        &self,
        input: CreateStorageLocationInput,
    ) -> ApplicationResult<StorageLocation> {
        validate_storage_location_name(&input.name)?;
        self.repo.create_storage_location(input).await
    }

    pub async fn update_storage_location(
        &self,
        id: Uuid,
        input: UpdateStorageLocationInput,
    ) -> ApplicationResult<StorageLocation> {
        validate_storage_location_name(&input.name)?;
        self.repo.update_storage_location(id, input).await
    }

    pub async fn archive_storage_location(
        &self,
        id: Uuid,
    ) -> ApplicationResult<StorageLocation> {
        self.repo.archive_storage_location(id).await
    }
}

/// Requires a non-empty location name.
fn validate_storage_location_name(name: &str) -> ApplicationResult<()> {
    if name.trim().is_empty() {
        return Err(ApplicationError::Validation(
            "Storage location name is required".into(),
        ));
    }

    Ok(())
}

// == Dose Log Service ==

/// Ensures stock bounds and purchase price are consistent.
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
