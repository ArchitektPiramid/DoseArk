use std::sync::Arc;

use doseark_application::{
    CatalogRepositoryBundle, InventoryDosingRepositoryBundle, RepositoryBundle, StorageDiagnostics,
};
use sqlx::PgPool;

use crate::{
    PgBrandRepository, PgDailyIntakeReportRepository, PgDoseLogNutrientRepository,
    PgDoseLogRepository, PgInventoryLotRepository, PgNutrientRepository,
    PgProductNutrientRepository, PgProductRepository, PgStockMovementRepository,
    PgStorageLocationRepository, PostgresDiagnostics,
};

#[derive(Clone)]
pub struct PostgresStorage {
    pool: PgPool,
}

impl PostgresStorage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn catalog_repositories(&self) -> CatalogRepositoryBundle {
        CatalogRepositoryBundle {
            brands: Arc::new(PgBrandRepository::new(self.pool.clone())),
            products: Arc::new(PgProductRepository::new(self.pool.clone())),
            nutrients: Arc::new(PgNutrientRepository::new(self.pool.clone())),
            product_nutrients: Arc::new(PgProductNutrientRepository::new(self.pool.clone())),
            storage_locations: Arc::new(PgStorageLocationRepository::new(self.pool.clone())),
        }
    }

    pub fn inventory_dosing_repositories(&self) -> InventoryDosingRepositoryBundle {
        InventoryDosingRepositoryBundle {
            inventory_lots: Arc::new(PgInventoryLotRepository::new(self.pool.clone())),
            stock_movements: Arc::new(PgStockMovementRepository::new(self.pool.clone())),
            dose_logs: Arc::new(PgDoseLogRepository::new(self.pool.clone())),
            dose_log_nutrients: Arc::new(PgDoseLogNutrientRepository::new(self.pool.clone())),
        }
    }

    pub fn repositories(&self) -> RepositoryBundle {
        RepositoryBundle {
            brands: Arc::new(PgBrandRepository::new(self.pool.clone())),
            products: Arc::new(PgProductRepository::new(self.pool.clone())),
            nutrients: Arc::new(PgNutrientRepository::new(self.pool.clone())),
            product_nutrients: Arc::new(PgProductNutrientRepository::new(self.pool.clone())),
            inventory_lots: Arc::new(PgInventoryLotRepository::new(self.pool.clone())),
            stock_movements: Arc::new(PgStockMovementRepository::new(self.pool.clone())),
            dose_logs: Arc::new(PgDoseLogRepository::new(self.pool.clone())),
            dose_log_nutrients: Arc::new(PgDoseLogNutrientRepository::new(self.pool.clone())),
            reports: Arc::new(PgDailyIntakeReportRepository::new(self.pool.clone())),
            storage_locations: Arc::new(PgStorageLocationRepository::new(self.pool.clone())),
        }
    }

    pub fn diagnostics(&self) -> Arc<dyn StorageDiagnostics> {
        Arc::new(PostgresDiagnostics::new(self.pool.clone()))
    }
}
