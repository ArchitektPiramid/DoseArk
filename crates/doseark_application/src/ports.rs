//! Porty dostępu do danych, podzielone według obszarów aplikacji

mod catalog;
mod dosing;
mod inventory;
mod reports;

pub use self::catalog::{
    BrandRepository, NutrientRepository, ProductNutrientRepository, ProductRepository,
};
pub use self::dosing::{DoseLogNutrientRepository, DoseLogRepository};
pub use self::inventory::{
    InventoryLotRepository, StockMovementRepository, StorageLocationRepository,
};
pub use self::reports::DailyIntakeReportRepository;
