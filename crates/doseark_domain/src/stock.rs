use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StockMovementType {
    Purchase,
    Intake,
    Correction,
    Discard,
    Expired,
    Split,
    Merge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockMovement {
    pub id: Uuid,
    pub inventory_lot_id: Uuid,
    pub dose_log_id: Option<Uuid>,
    pub movement_type: StockMovementType,
    pub delta_units: f64,
    pub remaining_after: f64,
    pub occurred_at: DateTime<Utc>,
    pub note: Option<String>,
}
