use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockMovement {
    pub id: Uuid,
    pub inventory_lot_id: Uuid,
    pub product_id: Uuid,
    pub dose_log_id: Option<Uuid>,

    pub occurred_at: DateTime<Utc>,

    pub movement_type: StockMovementType,
    pub quantity_delta: f64,
    pub quantity_after: f64,

    pub reason: Option<String>,
    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
}
