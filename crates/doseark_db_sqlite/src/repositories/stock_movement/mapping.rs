//! Mapowanie ruchów magazynowych na modele domenowe

use crate::sqlite_types::{into_optional_uuid, into_uuid, SqliteUuid};
use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{StockMovement, StockMovementType};

#[derive(Debug, sqlx::FromRow)]
pub(super) struct StockMovementRow {
    pub(super) id: SqliteUuid,
    pub(super) inventory_lot_id: SqliteUuid,
    pub(super) product_id: SqliteUuid,
    pub(super) dose_log_id: Option<SqliteUuid>,
    pub(super) occurred_at: chrono::DateTime<Utc>,
    pub(super) movement_type: String,
    pub(super) quantity_delta: f64,
    pub(super) quantity_after: f64,
    pub(super) reason: Option<String>,
    pub(super) notes: Option<String>,
    pub(super) created_at: chrono::DateTime<Utc>,
}

impl TryFrom<StockMovementRow> for StockMovement {
    type Error = ApplicationError;

    fn try_from(row: StockMovementRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: into_uuid(row.id),
            inventory_lot_id: into_uuid(row.inventory_lot_id),
            product_id: into_uuid(row.product_id),
            dose_log_id: into_optional_uuid(row.dose_log_id),
            occurred_at: row.occurred_at,
            movement_type: stock_movement_type_from_db(&row.movement_type)?,
            quantity_delta: row.quantity_delta,
            quantity_after: row.quantity_after,
            reason: row.reason,
            notes: row.notes,
            created_at: row.created_at,
        })
    }
}

fn stock_movement_type_from_db(value: &str) -> ApplicationResult<StockMovementType> {
    match value {
        "dose" => Ok(StockMovementType::Dose),
        "manual_add" => Ok(StockMovementType::ManualAdd),
        "manual_remove" => Ok(StockMovementType::ManualRemove),
        "correction" => Ok(StockMovementType::Correction),
        "discard" => Ok(StockMovementType::Discard),
        "open_package" => Ok(StockMovementType::OpenPackage),
        "reconcile" => Ok(StockMovementType::Reconcile),
        other => Err(ApplicationError::Repository(format!(
            "Unknown stock movement type: {other}"
        ))),
    }
}
