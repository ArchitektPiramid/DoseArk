use chrono::Utc;
use doseark_application::ApplicationError;
use doseark_domain::{DoseLog, DoseLogStatus};
use crate::sqlite_types::{into_optional_uuid, into_uuid, SqliteUuid};

#[derive(Debug, sqlx::FromRow)]
pub(super) struct DoseLogRow {
    pub(super) id: SqliteUuid,
    pub(super) product_id: SqliteUuid,
    pub(super) inventory_lot_id: Option<SqliteUuid>,
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


impl DoseLogRow {
    pub(super) fn id_uuid(&self) -> uuid::Uuid {
        self.id.0
    }

    pub(super) fn product_id_uuid(&self) -> uuid::Uuid {
        self.product_id.0
    }

    pub(super) fn inventory_lot_id_uuid(&self) -> Option<uuid::Uuid> {
        self.inventory_lot_id.as_ref().map(|value| value.0)
    }
}

#[derive(Debug, sqlx::FromRow)]
pub(super) struct DoseLogHistoryRow {
    pub(super) id: SqliteUuid,
    pub(super) product_id: SqliteUuid,
    pub(super) inventory_lot_id: Option<SqliteUuid>,
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
            id: into_uuid(row.id),
            product_id: into_uuid(row.product_id),
            inventory_lot_id: into_optional_uuid(row.inventory_lot_id),
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
            id: into_uuid(row.id),
            product_id: into_uuid(row.product_id),
            inventory_lot_id: into_optional_uuid(row.inventory_lot_id),
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
    pub(super) id: SqliteUuid,
    pub(super) product_id: SqliteUuid,
    pub(super) units_remaining: f64,
}


impl DoseInventoryLotRow {
    pub(super) fn id_uuid(&self) -> uuid::Uuid {
        self.id.0
    }

    pub(super) fn product_id_uuid(&self) -> uuid::Uuid {
        self.product_id.0
    }
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
