use std::sync::Arc;

use doseark_application::{
    CatalogRepositoryBundle, InventoryDosingRepositoryBundle, RepositoryBundle, StorageBackupManager, StorageDiagnostics,
};
use sqlx::SqlitePool;

use crate::{
    SqliteBackupManager, SqliteBrandRepository, SqliteDailyIntakeReportRepository, SqliteDiagnostics,
    SqliteDoseLogNutrientRepository, SqliteDoseLogRepository, SqliteInventoryLotRepository,
    SqliteNutrientRepository, SqliteProductNutrientRepository, SqliteProductRepository,
    SqliteStockMovementRepository, SqliteStorageLocationRepository,
};

#[derive(Clone)]
pub struct SqliteStorage {
    pool: SqlitePool,
    database_name: String,
}

impl SqliteStorage {
    pub fn new(pool: SqlitePool, database_name: impl Into<String>) -> Self {
        Self {
            pool,
            database_name: database_name.into(),
        }
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    pub fn catalog_repositories(&self) -> CatalogRepositoryBundle {
        CatalogRepositoryBundle {
            brands: Arc::new(SqliteBrandRepository::new(self.pool.clone())),
            products: Arc::new(SqliteProductRepository::new(self.pool.clone())),
            nutrients: Arc::new(SqliteNutrientRepository::new(self.pool.clone())),
            product_nutrients: Arc::new(SqliteProductNutrientRepository::new(self.pool.clone())),
            storage_locations: Arc::new(SqliteStorageLocationRepository::new(self.pool.clone())),
        }
    }

    pub fn inventory_dosing_repositories(&self) -> InventoryDosingRepositoryBundle {
        InventoryDosingRepositoryBundle {
            inventory_lots: Arc::new(SqliteInventoryLotRepository::new(self.pool.clone())),
            stock_movements: Arc::new(SqliteStockMovementRepository::new(self.pool.clone())),
            dose_logs: Arc::new(SqliteDoseLogRepository::new(self.pool.clone())),
            dose_log_nutrients: Arc::new(SqliteDoseLogNutrientRepository::new(self.pool.clone())),
        }
    }

    pub fn repositories(&self) -> RepositoryBundle {
        RepositoryBundle {
            brands: Arc::new(SqliteBrandRepository::new(self.pool.clone())),
            products: Arc::new(SqliteProductRepository::new(self.pool.clone())),
            nutrients: Arc::new(SqliteNutrientRepository::new(self.pool.clone())),
            product_nutrients: Arc::new(SqliteProductNutrientRepository::new(self.pool.clone())),
            inventory_lots: Arc::new(SqliteInventoryLotRepository::new(self.pool.clone())),
            stock_movements: Arc::new(SqliteStockMovementRepository::new(self.pool.clone())),
            dose_logs: Arc::new(SqliteDoseLogRepository::new(self.pool.clone())),
            dose_log_nutrients: Arc::new(SqliteDoseLogNutrientRepository::new(self.pool.clone())),
            reports: Arc::new(SqliteDailyIntakeReportRepository::new(self.pool.clone())),
            storage_locations: Arc::new(SqliteStorageLocationRepository::new(self.pool.clone())),
        }
    }

    pub fn diagnostics(&self) -> Arc<dyn StorageDiagnostics> {
        Arc::new(SqliteDiagnostics::new(
            self.pool.clone(),
            self.database_name.clone(),
        ))
    }

    pub fn backup_manager(&self) -> Arc<dyn StorageBackupManager> {
        Arc::new(SqliteBackupManager::new(self.pool.clone()))
    }
}
