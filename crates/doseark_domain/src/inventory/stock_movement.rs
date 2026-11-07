//! Stock movement events used to trace changes in a lot's quantity.
//!
//! Each record stores a signed delta and the resulting balance, allowing
//! inventory history to distinguish dose consumption from manual adjustments.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Reason category for a recorded stock movement
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StockMovementType {
    Dose,
    ManualAdd,
    ManualRemove,
    Correction,
    Discard,
    OpenPackage,
    Reconcile,
}

/// A signed stock change and the resulting lot quantity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockMovement {
    pub id: Uuid,
    pub inventory_lot_id: Uuid,
    pub product_id: Uuid,
    pub dose_log_id: Option<Uuid>,

    pub occurred_at: DateTime<Utc>,

    pub movement_type: StockMovementType,
    /// Signed change to stock: positive for additions, negative for removals
    pub quantity_delta: f64,
    /// Lot balance after applying this movement
    pub quantity_after: f64,

    pub reason: Option<String>,
    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
}
