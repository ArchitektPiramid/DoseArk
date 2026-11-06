use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductUserImportance {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductCategory {
    Supplement,
    Medication,
    Food,
    Herb,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DosageForm {
    Tablet,
    Capsule,
    Softgel,
    Powder,
    Liquid,
    Drops,
    Spray,
    Gummy,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub brand_id: Option<Uuid>,
    pub name: String,
    pub category: ProductCategory,
    pub dosage_form: DosageForm,
    pub serving_unit: String,
    pub serving_size_units: f64,
    pub unit_weight_mg: Option<f64>,
    pub barcode: Option<String>,
    pub description: Option<String>,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub ui_color: Option<String>,
    pub ui_icon: Option<String>,
    pub user_importance: ProductUserImportance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductInput {
    pub brand_id: Option<Uuid>,
    pub name: String,
    pub category: ProductCategory,
    pub dosage_form: DosageForm,
    pub serving_unit: String,
    pub serving_size_units: f64,
    pub unit_weight_mg: Option<f64>,
    pub barcode: Option<String>,
    pub description: Option<String>,
    pub ui_color: Option<String>,
    pub ui_icon: Option<String>,
    pub user_importance: Option<ProductUserImportance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProductInput {
    pub brand_id: Option<Uuid>,
    pub name: String,
    pub category: ProductCategory,
    pub dosage_form: DosageForm,
    pub serving_unit: String,
    pub serving_size_units: f64,
    pub unit_weight_mg: Option<f64>,
    pub barcode: Option<String>,
    pub description: Option<String>,
    pub ui_color: Option<String>,
    pub ui_icon: Option<String>,
    pub user_importance: Option<ProductUserImportance>,
}
