//! Daily, hourly, and nutrient trend report contracts shared by core and persistence.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{DoseLog, NutrientKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyIntakeReportInput {
    pub date: NaiveDate,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyIntakeReport {
    pub date: NaiveDate,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub rows: Vec<DailyIntakeReportRow>,
    pub dose_logs: Vec<DoseLog>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyIntakeReportRow {
    pub nutrient_id: Uuid,

    pub nutrient_canonical_name: String,
    pub nutrient_display_name: String,
    pub nutrient_kind: NutrientKind,

    pub amount_total: f64,
    pub unit: String,

    pub reference_daily_amount: Option<f64>,
    pub reference_daily_unit: Option<String>,
    pub reference_source: Option<String>,

    pub percent_of_reference_total: Option<f64>,
    pub dose_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientTrendReportInput {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub timezone_offset_minutes: i32,
    pub day_start_hour: u8,
    pub nutrient_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientTrendReport {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub selected_nutrient_id: Option<Uuid>,
    pub selected_product_id: Option<Uuid>,
    pub selected_unit: Option<String>,
    pub day_start_hour: u8,
    pub nutrients: Vec<NutrientTrendOption>,
    pub days: Vec<NutrientTrendDay>,
    #[serde(default)]
    pub events: Vec<NutrientTrendEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientTrendOption {
    pub nutrient_id: Uuid,
    pub nutrient_canonical_name: String,
    pub nutrient_display_name: String,
    pub nutrient_kind: NutrientKind,
    pub unit: String,
    pub amount_total: f64,
    pub dose_count: u32,
    pub recorded_day_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientTrendEvent {
    pub dose_log_id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub product_brand_name: Option<String>,
    pub taken_at: DateTime<Utc>,
    pub amount: f64,
    pub unit: String,
    pub dose_units_taken: f64,
    pub dose_unit: String,
    pub inventory_lot_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientTrendDay {
    pub date: NaiveDate,
    pub amount_total: f64,
    pub unit: String,
    pub reference_daily_amount: Option<f64>,
    pub reference_daily_unit: Option<String>,
    pub reference_source: Option<String>,
    pub percent_of_reference_total: Option<f64>,
    pub dose_count: u32,
}
