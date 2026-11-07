//! Repozytorium partii magazynowych — lista oraz transakcje zapisu

mod create;
mod mapping;
mod quantity;
mod queries;
mod read;
mod update;

use async_trait::async_trait;
use doseark_application::{ApplicationResult, InventoryLotRepository};
use doseark_domain::{CreateInventoryLotInput, InventoryLot, UpdateInventoryLotInput};
use sqlx::SqlitePool;
use uuid::Uuid;

pub(crate) use quantity::normalize_stock_quantity;

/// SQLite implementation of the inventory lot repository
#[derive(Clone)]
pub struct SqliteInventoryLotRepository {
    pool: SqlitePool,
}

impl SqliteInventoryLotRepository {
    /// Shares the existing connection pool with inventory operations
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InventoryLotRepository for SqliteInventoryLotRepository {
    async fn list_inventory_lots(&self) -> ApplicationResult<Vec<InventoryLot>> {
        read::list_inventory_lots(&self.pool).await
    }

    async fn create_inventory_lot(
        &self,
        input: CreateInventoryLotInput,
    ) -> ApplicationResult<InventoryLot> {
        create::create_inventory_lot(&self.pool, input).await
    }

    async fn update_inventory_lot(
        &self,
        id: Uuid,
        input: UpdateInventoryLotInput,
    ) -> ApplicationResult<InventoryLot> {
        update::update_inventory_lot(&self.pool, id, input).await
    }
}
