use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NutrientKind {
    Vitamin,
    Mineral,
    AminoAcid,
    FattyAcid,
    HerbExtract,
    DrugSubstance,
    Probiotic,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nutrient {
    pub id: Uuid,
    pub canonical_name: String,
    pub display_name: String,
    pub kind: NutrientKind,
    pub default_unit: String,
    pub notes: Option<String>,

    pub reference_daily_amount: Option<f64>,
    pub reference_daily_unit: Option<String>,
    pub reference_source: Option<String>,
    pub reference_notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNutrientInput {
    pub canonical_name: String,
    pub display_name: String,
    pub kind: NutrientKind,
    pub default_unit: String,
    pub notes: Option<String>,

    pub reference_daily_amount: Option<f64>,
    pub reference_daily_unit: Option<String>,
    pub reference_source: Option<String>,
    pub reference_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNutrientInput {
    pub canonical_name: String,
    pub display_name: String,
    pub kind: NutrientKind,
    pub default_unit: String,
    pub notes: Option<String>,

    pub reference_daily_amount: Option<f64>,
    pub reference_daily_unit: Option<String>,
    pub reference_source: Option<String>,
    pub reference_notes: Option<String>,
}

// // TODO: remove
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ProductNutrient {
//     pub id: Uuid,
//     pub product_id: Uuid,
//     pub nutrient_id: Uuid,
//     pub amount_per_serving: f64,
//     pub unit: String,
//     pub daily_value_percent: Option<f64>,
//     pub source: String,
//     pub notes: Option<String>,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
// }
