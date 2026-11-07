//! Serwisy magazynu, ruchów i lokalizacji

mod locations;
mod lots;
mod movements;

pub use self::locations::StorageLocationService;
pub use self::lots::InventoryLotService;
pub use self::movements::StockMovementService;
