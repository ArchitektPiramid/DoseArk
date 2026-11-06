use chrono::{DateTime, NaiveDate, Utc};
use doseark_application::ApplicationError;
use doseark_domain::{InventoryLot, InventoryLotStatus};
use crate::sqlite_types::{into_optional_uuid, into_uuid, SqliteUuid};

#[derive(Debug, sqlx::FromRow)]
pub(super) struct InventoryLotRow {
    pub(super) id: SqliteUuid,
    pub(super) product_id: SqliteUuid,
    pub(super) lot_number: Option<String>,
    pub(super) storage_location_id: Option<SqliteUuid>,
    pub(super) storage_location: Option<String>,
    pub(super) units_initial: f64,
    pub(super) units_remaining: f64,
    pub(super) purchase_date: Option<NaiveDate>,
    pub(super) opened_date: Option<NaiveDate>,
    pub(super) expires_at: Option<NaiveDate>,
    pub(super) price_amount: Option<f64>,
    pub(super) price_currency: Option<String>,
    pub(super) status: String,
    pub(super) notes: Option<String>,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}


impl InventoryLotRow {
    pub(super) fn id_uuid(&self) -> uuid::Uuid {
        self.id.0
    }

    pub(super) fn product_id_uuid(&self) -> uuid::Uuid {
        self.product_id.0
    }
}

impl TryFrom<InventoryLotRow> for InventoryLot {
    type Error = ApplicationError;

    fn try_from(row: InventoryLotRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: into_uuid(row.id),
            product_id: into_uuid(row.product_id),
            lot_number: row.lot_number,
            storage_location: row.storage_location,
            storage_location_id: into_optional_uuid(row.storage_location_id),
            units_initial: row.units_initial,
            units_remaining: row.units_remaining,
            purchase_date: row.purchase_date,
            opened_date: row.opened_date,
            expires_at: row.expires_at,
            price_amount: row.price_amount,
            price_currency: row.price_currency,
            status: inventory_lot_status_from_db(&row.status)?,
            notes: row.notes,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

fn inventory_lot_status_from_db(value: &str) -> Result<InventoryLotStatus, ApplicationError> {
    match value {
        "active" => Ok(InventoryLotStatus::Active),
        "reserve" => Ok(InventoryLotStatus::Reserve),
        "on_hold" => Ok(InventoryLotStatus::OnHold),
        "empty" => Ok(InventoryLotStatus::Empty),
        "expired" => Ok(InventoryLotStatus::Expired),
        "discarded" => Ok(InventoryLotStatus::Discarded),
        "archived" => Ok(InventoryLotStatus::Archived),
        other => Err(ApplicationError::Repository(format!(
            "Unknown inventory lot status: {other}"
        ))),
    }
}

pub(super) fn inventory_lot_status_to_db(value: &InventoryLotStatus) -> &'static str {
    match value {
        InventoryLotStatus::Active => "active",
        InventoryLotStatus::Reserve => "reserve",
        InventoryLotStatus::OnHold => "on_hold",
        InventoryLotStatus::Empty => "empty",
        InventoryLotStatus::Expired => "expired",
        InventoryLotStatus::Discarded => "discarded",
        InventoryLotStatus::Archived => "archived",
    }
}
