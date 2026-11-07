//! Odczyt historii zmian stanów magazynowych

mod mapping;
pub(crate) mod writer;

use async_trait::async_trait;
use doseark_application::{ApplicationError, ApplicationResult, StockMovementRepository};
use doseark_domain::StockMovement;
use sqlx::SqlitePool;
use uuid::Uuid;

use self::mapping::StockMovementRow;

#[derive(Clone)]
pub struct SqliteStockMovementRepository {
    pool: SqlitePool,
}

impl SqliteStockMovementRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StockMovementRepository for SqliteStockMovementRepository {
    async fn list_stock_movements(&self) -> ApplicationResult<Vec<StockMovement>> {
        tracing::trace!("DB query started: list SQLite stock movements");

        let rows = sqlx::query_as::<_, StockMovementRow>(stock_movement_select_sql())
            .fetch_all(&self.pool)
            .await
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::debug!(
            row_count = rows.len(),
            "DB query finished: list SQLite stock movements",
        );

        rows.into_iter().map(StockMovement::try_from).collect()
    }

    /// Reads the movement history for a single inventory lot
    async fn list_stock_movements_for_lot(
        &self,
        inventory_lot_id: Uuid,
    ) -> ApplicationResult<Vec<StockMovement>> {
        tracing::trace!(
            inventory_lot_id = %inventory_lot_id,
            "DB query started: list SQLite stock movements for lot",
        );

        let rows = sqlx::query_as::<_, StockMovementRow>(
            r#"
            SELECT
                id,
                inventory_lot_id,
                product_id,
                dose_log_id,
                occurred_at,
                movement_type,
                quantity_delta,
                quantity_after,
                reason,
                notes,
                created_at
            FROM stock_movements
            WHERE inventory_lot_id = ?1
            ORDER BY occurred_at DESC, created_at DESC, id DESC
            "#,
        )
        .bind(inventory_lot_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::debug!(
            inventory_lot_id = %inventory_lot_id,
            row_count = rows.len(),
            "DB query finished: list SQLite stock movements for lot",
        );

        rows.into_iter().map(StockMovement::try_from).collect()
    }
}

/// SQL shared by full and lot-specific movement history
fn stock_movement_select_sql() -> &'static str {
    r#"
    SELECT
        id,
        inventory_lot_id,
        product_id,
        dose_log_id,
        occurred_at,
        movement_type,
        quantity_delta,
        quantity_after,
        reason,
        notes,
        created_at
    FROM stock_movements
    ORDER BY occurred_at DESC, created_at DESC, id DESC
    LIMIT 500
    "#
}
