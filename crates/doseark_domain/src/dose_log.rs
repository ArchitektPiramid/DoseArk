//! Historia dawek, jej statusy oraz dane wejściowe operacji

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Current lifecycle state of an intake record
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DoseLogStatus {
    Active,
    Cancelled,
}

/// An intake event and optional stock projections for history views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoseLog {
    pub id: Uuid,
    pub product_id: Uuid,
    pub inventory_lot_id: Option<Uuid>,

    pub taken_at: DateTime<Utc>,

    /// Quantity consumed in the named product unit
    pub units_taken: f64,
    pub unit: String,

    /// Number of standard product servings represented by this intake
    pub serving_multiplier: f64,

    /// Chronological stock projection for history views.
    /// These values are populated by list queries and remain `None` for write responses.
    #[serde(default)]
    pub inventory_quantity_before: Option<f64>,
    #[serde(default)]
    pub inventory_quantity_after: Option<f64>,
    #[serde(default)]
    pub inventory_units_initial: Option<f64>,

    pub reason: Option<String>,
    pub notes: Option<String>,

    pub status: DoseLogStatus,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancellation_reason: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Filters and pagination requested for intake history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoseLogListInput {
    pub limit: i64,
    pub offset: i64,
    pub product_id: Option<Uuid>,
}

/// A paginated collection of intake events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoseLogPage {
    pub items: Vec<DoseLog>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

/// Values submitted when recording a new dose
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDoseLogInput {
    pub product_id: Uuid,
    pub inventory_lot_id: Option<Uuid>,

    pub taken_at: DateTime<Utc>,

    /// Quantity consumed in the named product unit
    pub units_taken: f64,
    pub unit: String,

    /// Number of standard product servings represented by this intake
    pub serving_multiplier: f64,

    pub reason: Option<String>,
    pub notes: Option<String>,
}

/// Replacement values submitted when editing an existing dose
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDoseLogInput {
    pub product_id: Uuid,
    pub inventory_lot_id: Option<Uuid>,

    pub taken_at: DateTime<Utc>,

    /// Quantity consumed in the named product unit
    pub units_taken: f64,
    pub unit: String,

    /// Number of standard product servings represented by this intake
    pub serving_multiplier: f64,

    pub reason: Option<String>,
    pub notes: Option<String>,
}
