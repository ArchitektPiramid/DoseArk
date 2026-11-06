use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InventoryLotStatus {
    Active,
    Reserve,
    OnHold,
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
    pub storage_location: Option<String>,
pub storage_location_id: Option<Uuid>,

    pub units_initial: f64,
    pub units_remaining: f64,

    pub purchase_date: Option<NaiveDate>,
    pub opened_date: Option<NaiveDate>,
    pub expires_at: Option<NaiveDate>,

    pub price_amount: Option<f64>,
    pub price_currency: Option<String>,

    pub status: InventoryLotStatus,
    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInventoryLotInput {
    pub product_id: Uuid,

    pub lot_number: Option<String>,
    pub storage_location: Option<String>,
    pub storage_location_id: Option<Uuid>,

    pub units_initial: f64,
    pub units_remaining: Option<f64>,

    pub purchase_date: Option<NaiveDate>,
    pub opened_date: Option<NaiveDate>,
    pub expires_at: Option<NaiveDate>,

    pub price_amount: Option<f64>,
    pub price_currency: Option<String>,

    pub status: Option<InventoryLotStatus>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInventoryLotInput {
    pub product_id: Uuid,

    pub lot_number: Option<String>,
    pub storage_location: Option<String>,
    pub storage_location_id: Option<Uuid>,

    pub units_initial: f64,
    pub units_remaining: f64,

    pub purchase_date: Option<NaiveDate>,
    pub opened_date: Option<NaiveDate>,
    pub expires_at: Option<NaiveDate>,

    pub price_amount: Option<f64>,
    pub price_currency: Option<String>,

    pub status: InventoryLotStatus,
    pub notes: Option<String>,
}
