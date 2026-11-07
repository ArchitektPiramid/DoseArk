//! Historyczne wartości składników zapisanych wraz z dawką

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::NutrientKind;

/// Snapshot of one nutrient captured for a specific intake event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoseLogNutrient {
    pub id: Uuid,

    pub dose_log_id: Uuid,
    pub nutrient_id: Uuid,

    pub nutrient_canonical_name: String,
    pub nutrient_display_name: String,
    pub nutrient_kind: NutrientKind,

    /// Amount stored at intake time, independent of later composition edits
    pub amount: f64,
    pub unit: String,

    pub reference_daily_amount: Option<f64>,
    pub reference_daily_unit: Option<String>,
    pub reference_source: Option<String>,

    pub percent_of_reference: Option<f64>,

    pub created_at: DateTime<Utc>,
}
