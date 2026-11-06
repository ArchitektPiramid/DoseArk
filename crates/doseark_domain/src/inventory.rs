use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InventoryLotStatus {
    Sealed,
    Active,
    Empty,
    Expired,
    Discarded,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryLot {
    pub id: Uuid,
    pub product_id: Uuid,
    pub lot_number: Option<String>,
    pub package_label: Option<String>,
    pub units_initial: f64,
    pub units_remaining: f64,
    pub opened_at: Option<DateTime<Utc>>,
    pub purchased_at: Option<DateTime<Utc>>,
    pub expires_at: Option<NaiveDate>,
    pub price_total: Option<f64>,
    pub currency: Option<String>,
    pub storage_location: Option<String>,
    pub status: InventoryLotStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
