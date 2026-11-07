//! Sprawdza zachowanie publicznych ścieżek po podziale portów na moduły

use doseark_application::ports::{
    BrandRepository, DailyIntakeReportRepository, DoseLogNutrientRepository, DoseLogRepository,
    InventoryLotRepository, NutrientRepository, ProductNutrientRepository, ProductRepository,
    StockMovementRepository, StorageLocationRepository,
};

// Importing every trait via the original module path is the compatibility check
#[test]
fn all_repository_ports_remain_public() {
    fn available<T: ?Sized>() {}

    available::<dyn BrandRepository>();
    available::<dyn ProductRepository>();
    available::<dyn NutrientRepository>();
    available::<dyn ProductNutrientRepository>();
    available::<dyn InventoryLotRepository>();
    available::<dyn StockMovementRepository>();
    available::<dyn StorageLocationRepository>();
    available::<dyn DoseLogRepository>();
    available::<dyn DoseLogNutrientRepository>();
    available::<dyn DailyIntakeReportRepository>();
}
