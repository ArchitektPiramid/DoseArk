use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductNutrient {
    pub id: Uuid,
    pub product_id: Uuid,
    pub nutrient_id: Uuid,

    pub amount_per_serving: f64,
    pub unit: String,

    pub label_daily_value_percent: Option<f64>,
    pub label_reference_amount: Option<f64>,
    pub label_reference_unit: Option<String>,

    pub source: String,
    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveProductNutrientInput {
    pub nutrient_id: Uuid,
    pub amount_per_serving: f64,
    pub unit: String,

    pub label_daily_value_percent: Option<f64>,
    pub label_reference_amount: Option<f64>,
    pub label_reference_unit: Option<String>,

    pub source: String,
    pub notes: Option<String>,
}
