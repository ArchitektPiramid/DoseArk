//! Partie magazynowe, ich stany oraz polecenia utworzenia i edycji

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Availability and lifecycle states of a stored product lot
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

/// A physical product lot with its current remaining stock
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryLot {
    pub id: Uuid,
    /// Product identity of this lot; persisted lots must retain it
    pub product_id: Uuid,

    pub lot_number: Option<String>,
    pub storage_location: Option<String>,
    pub storage_location_id: Option<Uuid>,

    /// Original quantity in units of the associated product
    pub units_initial: f64,
    /// Current quantity remaining in the same units
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

/// Attributes required to create an inventory lot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInventoryLotInput {
    /// Product identity of this lot; persisted lots must retain it
    pub product_id: Uuid,

    pub lot_number: Option<String>,
    pub storage_location: Option<String>,
    pub storage_location_id: Option<Uuid>,

    /// Original quantity in units of the associated product
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

/// Editable lot attributes, including its current stock balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInventoryLotInput {
    /// Product identity of this lot; persisted lots must retain it
    pub product_id: Uuid,

    pub lot_number: Option<String>,
    pub storage_location: Option<String>,
    pub storage_location_id: Option<Uuid>,

    /// Original quantity in units of the associated product
    pub units_initial: f64,
    /// Current quantity remaining in the same units
    pub units_remaining: f64,

    pub purchase_date: Option<NaiveDate>,
    pub opened_date: Option<NaiveDate>,
    pub expires_at: Option<NaiveDate>,

    pub price_amount: Option<f64>,
    pub price_currency: Option<String>,

    pub status: InventoryLotStatus,
    pub notes: Option<String>,
}
