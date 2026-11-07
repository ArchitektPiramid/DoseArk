//! Inventory lot listing with resolved storage-location names.

use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::InventoryLot;
use sqlx::PgPool;

use super::mapping::InventoryLotRow;
use super::support::inventory_lot_select_sql;

pub(super) async fn list_inventory_lots(pool: &PgPool) -> ApplicationResult<Vec<InventoryLot>> {
    tracing::trace!("DB query started: list inventory lots");

    let rows = sqlx::query_as::<_, InventoryLotRow>(inventory_lot_select_sql())
        .fetch_all(pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    tracing::debug!(
        row_count = rows.len(),
        "DB query finished: list inventory lots",
    );

    rows.into_iter().map(InventoryLot::try_from).collect()
}
