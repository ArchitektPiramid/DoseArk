//! Application use cases organized by domain; public paths remain unchanged.
mod catalog;
mod dosing;
mod inventory;
mod nutrients;
mod reports;

pub use catalog::{BrandService, ProductService};
pub use dosing::{validated_serving_multiplier, DoseLogNutrientService, DoseLogService};
pub use inventory::{InventoryLotService, StockMovementService, StorageLocationService};
pub use nutrients::{NutrientService, ProductCompositionService};
pub use reports::DailyIntakeReportService;
