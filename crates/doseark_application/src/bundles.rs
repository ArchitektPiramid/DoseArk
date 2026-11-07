//! Zestawy zależności wstrzykiwanych do serwisów aplikacji

use std::sync::Arc;

use crate::{
    BrandRepository, BrandService, DailyIntakeReportRepository, DailyIntakeReportService,
    DoseLogNutrientRepository, DoseLogNutrientService, DoseLogRepository, DoseLogService,
    InventoryLotRepository, InventoryLotService, NutrientRepository, NutrientService,
    ProductCompositionService, ProductNutrientRepository, ProductRepository, ProductService,
    StockMovementRepository, StockMovementService, StorageLocationRepository,
    StorageLocationService,
};

#[derive(Clone)]
/// Catalog adapters shared by catalog-oriented services
pub struct CatalogRepositoryBundle {
    pub brands: Arc<dyn BrandRepository>,
    pub products: Arc<dyn ProductRepository>,
    pub nutrients: Arc<dyn NutrientRepository>,
    pub product_nutrients: Arc<dyn ProductNutrientRepository>,
    pub storage_locations: Arc<dyn StorageLocationRepository>,
}

#[derive(Clone)]
/// Storage adapters used for inventory and dosing
pub struct InventoryDosingRepositoryBundle {
    pub inventory_lots: Arc<dyn InventoryLotRepository>,
    pub stock_movements: Arc<dyn StockMovementRepository>,
    pub dose_logs: Arc<dyn DoseLogRepository>,
    pub dose_log_nutrients: Arc<dyn DoseLogNutrientRepository>,
}

#[derive(Clone)]
/// Complete set of persistence ports supplied by the runtime
pub struct RepositoryBundle {
    pub brands: Arc<dyn BrandRepository>,
    pub products: Arc<dyn ProductRepository>,
    pub nutrients: Arc<dyn NutrientRepository>,
    pub product_nutrients: Arc<dyn ProductNutrientRepository>,
    pub inventory_lots: Arc<dyn InventoryLotRepository>,
    pub stock_movements: Arc<dyn StockMovementRepository>,
    pub dose_logs: Arc<dyn DoseLogRepository>,
    pub dose_log_nutrients: Arc<dyn DoseLogNutrientRepository>,
    pub reports: Arc<dyn DailyIntakeReportRepository>,
    pub storage_locations: Arc<dyn StorageLocationRepository>,
}

#[derive(Clone)]
/// Application services wired to their repository implementations
pub struct AppServices {
    pub brands: BrandService,
    pub products: ProductService,
    pub nutrients: NutrientService,
    pub product_composition: ProductCompositionService,
    pub inventory_lots: InventoryLotService,
    pub stock_movements: StockMovementService,
    pub dose_logs: DoseLogService,
    pub dose_log_nutrients: DoseLogNutrientService,
    pub reports: DailyIntakeReportService,
    pub storage_locations: StorageLocationService,
}

impl AppServices {
    /// Construct the service graph from injected repository ports
    pub fn from_repositories(repositories: RepositoryBundle) -> Self {
        Self {
            brands: BrandService::new(repositories.brands),
            products: ProductService::new(repositories.products),
            nutrients: NutrientService::new(repositories.nutrients),
            product_composition: ProductCompositionService::new(repositories.product_nutrients),
            inventory_lots: InventoryLotService::new(repositories.inventory_lots),
            stock_movements: StockMovementService::new(repositories.stock_movements),
            dose_logs: DoseLogService::new(repositories.dose_logs),
            dose_log_nutrients: DoseLogNutrientService::new(repositories.dose_log_nutrients),
            reports: DailyIntakeReportService::new(repositories.reports),
            storage_locations: StorageLocationService::new(repositories.storage_locations),
        }
    }
}
