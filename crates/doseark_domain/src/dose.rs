use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoseLog {
    pub id: Uuid,
    pub product_id: Uuid,
    pub inventory_lot_id: Option<Uuid>,
    pub taken_at: DateTime<Utc>,
    pub amount_units: f64,
    pub serving_multiplier: f64,
    pub note: Option<String>,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
