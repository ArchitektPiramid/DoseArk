//! Lista partii wraz z nazwami lokalizacji magazynowych

use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::InventoryLot;
use sqlx::SqlitePool;

use super::mapping::InventoryLotRow;
use super::queries::inventory_lot_select_sql;

/// Loads all lots, including held and archived entries
pub(super) async fn list_inventory_lots(pool: &SqlitePool) -> ApplicationResult<Vec<InventoryLot>> {
    tracing::trace!("DB query started: list SQLite inventory lots");

    let rows = sqlx::query_as::<_, InventoryLotRow>(inventory_lot_select_sql())
        .fetch_all(pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    tracing::debug!(
        row_count = rows.len(),
        "DB query finished: list SQLite inventory lots",
    );

    rows.into_iter().map(InventoryLot::try_from).collect()
}
