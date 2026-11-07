//! Product composition entries and inputs for replacing that composition.
//!
//! Nutrient identifiers link ingredients to the shared catalog. Per-serving
//! quantities here describe a product, not a historical intake snapshot.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A nutrient quantity declared per standard product serving
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductNutrient {
    pub id: Uuid,
    pub product_id: Uuid,
    pub nutrient_id: Uuid,

    /// Nutrient quantity per standard product serving, in `unit`
    pub amount_per_serving: f64,
    pub unit: String,

    pub label_daily_value_percent: Option<f64>,
    /// Reference amount printed on the product label, if provided
    pub label_reference_amount: Option<f64>,
    pub label_reference_unit: Option<String>,

    pub source: String,
    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Values used to save one nutrient in a product composition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveProductNutrientInput {
    pub nutrient_id: Uuid,
    /// Nutrient quantity per standard product serving, in `unit`
    pub amount_per_serving: f64,
    pub unit: String,

    pub label_daily_value_percent: Option<f64>,
    /// Reference amount printed on the product label, if provided
    pub label_reference_amount: Option<f64>,
    pub label_reference_unit: Option<String>,

    pub source: String,
    pub notes: Option<String>,
}
