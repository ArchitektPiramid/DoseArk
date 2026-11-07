//! PostgreSQL inventory lot persistence and explicit stock-change auditing.
//!
//! Lot creation and updates run in transactions so the lot row and its
//! corresponding manual stock movement are committed together.

mod create;
mod mapping;
mod read;
mod support;
mod update;

use async_trait::async_trait;
use doseark_application::{ApplicationResult, InventoryLotRepository};
use doseark_domain::{CreateInventoryLotInput, InventoryLot, UpdateInventoryLotInput};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgInventoryLotRepository {
    pool: PgPool,
}

impl PgInventoryLotRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InventoryLotRepository for PgInventoryLotRepository {
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
