//! PostgreSQL row mappings for dose history and locked inventory lots.

use chrono::Utc;
use doseark_application::ApplicationError;
use doseark_domain::{DoseLog, DoseLogStatus};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub(super) struct DoseLogRow {
    pub(super) id: Uuid,
    pub(super) product_id: Uuid,
    pub(super) inventory_lot_id: Option<Uuid>,

    pub(super) taken_at: chrono::DateTime<Utc>,

    pub(super) units_taken: f64,
    pub(super) unit: String,

    pub(super) serving_multiplier: f64,

    pub(super) reason: Option<String>,
    pub(super) notes: Option<String>,

    pub(super) status: String,
    pub(super) cancelled_at: Option<chrono::DateTime<Utc>>,
    pub(super) cancellation_reason: Option<String>,

    pub(super) created_at: chrono::DateTime<Utc>,
    pub(super) updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub(super) struct DoseLogHistoryRow {
    pub(super) id: Uuid,
    pub(super) product_id: Uuid,
    pub(super) inventory_lot_id: Option<Uuid>,

    pub(super) taken_at: chrono::DateTime<Utc>,

    pub(super) units_taken: f64,
    pub(super) unit: String,

    pub(super) serving_multiplier: f64,

    pub(super) inventory_quantity_before: Option<f64>,
    pub(super) inventory_quantity_after: Option<f64>,
    pub(super) inventory_units_initial: Option<f64>,

    pub(super) reason: Option<String>,
    pub(super) notes: Option<String>,

    pub(super) status: String,
    pub(super) cancelled_at: Option<chrono::DateTime<Utc>>,
    pub(super) cancellation_reason: Option<String>,

    pub(super) created_at: chrono::DateTime<Utc>,
    pub(super) updated_at: chrono::DateTime<Utc>,
}

impl TryFrom<DoseLogRow> for DoseLog {
    type Error = ApplicationError;

    fn try_from(row: DoseLogRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            product_id: row.product_id,
            inventory_lot_id: row.inventory_lot_id,
            taken_at: row.taken_at,
            units_taken: row.units_taken,
            unit: row.unit,
            serving_multiplier: row.serving_multiplier,
            inventory_quantity_before: None,
            inventory_quantity_after: None,
            inventory_units_initial: None,
            reason: row.reason,
            notes: row.notes,
            status: dose_log_status_from_db(&row.status)?,
            cancelled_at: row.cancelled_at,
            cancellation_reason: row.cancellation_reason,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

impl TryFrom<DoseLogHistoryRow> for DoseLog {
    type Error = ApplicationError;

    fn try_from(row: DoseLogHistoryRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            product_id: row.product_id,
            inventory_lot_id: row.inventory_lot_id,
            taken_at: row.taken_at,
            units_taken: row.units_taken,
            unit: row.unit,
            serving_multiplier: row.serving_multiplier,
            inventory_quantity_before: row.inventory_quantity_before,
            inventory_quantity_after: row.inventory_quantity_after,
            inventory_units_initial: row.inventory_units_initial,
            reason: row.reason,
            notes: row.notes,
            status: dose_log_status_from_db(&row.status)?,
            cancelled_at: row.cancelled_at,
            cancellation_reason: row.cancellation_reason,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

#[derive(Debug, sqlx::FromRow)]
pub(super) struct DoseInventoryLotRow {
    pub(super) id: Uuid,
    pub(super) product_id: Uuid,
    pub(super) units_remaining: f64,
    pub(super) status: String,
}

fn dose_log_status_from_db(value: &str) -> Result<DoseLogStatus, ApplicationError> {
    match value {
        "active" => Ok(DoseLogStatus::Active),
        "cancelled" => Ok(DoseLogStatus::Cancelled),
        other => Err(ApplicationError::Repository(format!(
            "Unknown dose log status: {other}"
        ))),
    }
}
