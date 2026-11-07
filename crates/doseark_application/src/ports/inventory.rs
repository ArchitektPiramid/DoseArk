//! Porty partii magazynowych, ruchów i lokalizacji

use crate::ApplicationResult;
use async_trait::async_trait;
use doseark_domain::{
    CreateInventoryLotInput, CreateStorageLocationInput, InventoryLot, StockMovement,
    StorageLocation, UpdateInventoryLotInput, UpdateStorageLocationInput,
};
use uuid::Uuid;

/// Persists inventory lots and their quantities
#[async_trait]
pub trait InventoryLotRepository: Send + Sync {
    async fn list_inventory_lots(&self) -> ApplicationResult<Vec<InventoryLot>>;

    async fn create_inventory_lot(
        &self,
        input: CreateInventoryLotInput,
    ) -> ApplicationResult<InventoryLot>;

    async fn update_inventory_lot(
        &self,
        id: Uuid,
        input: UpdateInventoryLotInput,
    ) -> ApplicationResult<InventoryLot>;
}

/// Exposes recorded changes to inventory quantities
#[async_trait]
pub trait StockMovementRepository: Send + Sync {
    async fn list_stock_movements(&self) -> ApplicationResult<Vec<StockMovement>>;

    async fn list_stock_movements_for_lot(
        &self,
        inventory_lot_id: Uuid,
    ) -> ApplicationResult<Vec<StockMovement>>;
}

/// Manages inventory locations, including archived ones
#[async_trait]
pub trait StorageLocationRepository: Send + Sync {
    async fn list_storage_locations(
        &self,
        include_archived: bool,
    ) -> ApplicationResult<Vec<StorageLocation>>;

    async fn create_storage_location(
        &self,
        input: CreateStorageLocationInput,
    ) -> ApplicationResult<StorageLocation>;

    async fn update_storage_location(
        &self,
        id: Uuid,
        input: UpdateStorageLocationInput,
    ) -> ApplicationResult<StorageLocation>;

    async fn archive_storage_location(&self, id: Uuid) -> ApplicationResult<StorageLocation>;
}
