//! Nutrient definitions and their optional daily reference values.
//!
//! The catalog defines ingredient identity, display names and reference units;
//! amounts for a particular product belong to product composition records.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Classification of a catalog nutrient or active substance
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

/// Canonical and display names with optional reference intake values
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

/// Data required to register a nutrient
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

/// Editable nutrient details and reference values
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
