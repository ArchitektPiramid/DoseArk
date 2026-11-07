//! Product catalog entities, classification and serving configuration.
//!
//! A serving size expresses how many product units form one standard serving.
//! The same model is shared by API use cases and persistence adapters.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User-assigned priority of a product
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductUserImportance {
    Low,
    Normal,
    High,
    Critical,
}

/// Catalog category used to classify products
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductCategory {
    Supplement,
    Medication,
    Food,
    Herb,
    Other,
}

/// Physical form used to dispense a product
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

/// A catalog product with its standard serving configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub brand_id: Option<Uuid>,
    pub name: String,
    pub category: ProductCategory,
    pub dosage_form: DosageForm,
    pub serving_unit: String,
    /// Product units represented by one standard serving
    pub serving_size_units: f64,
    /// Optional mass in milligrams per product unit
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

/// Data required to create a catalog product
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductInput {
    pub brand_id: Option<Uuid>,
    pub name: String,
    pub category: ProductCategory,
    pub dosage_form: DosageForm,
    pub serving_unit: String,
    /// Product units represented by one standard serving
    pub serving_size_units: f64,
    /// Optional mass in milligrams per product unit
    pub unit_weight_mg: Option<f64>,
    pub barcode: Option<String>,
    pub description: Option<String>,
    pub ui_color: Option<String>,
    pub ui_icon: Option<String>,
    pub user_importance: Option<ProductUserImportance>,
}

/// Editable attributes of an existing product
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProductInput {
    pub brand_id: Option<Uuid>,
    pub name: String,
    pub category: ProductCategory,
    pub dosage_form: DosageForm,
    pub serving_unit: String,
    /// Product units represented by one standard serving
    pub serving_size_units: f64,
    /// Optional mass in milligrams per product unit
    pub unit_weight_mg: Option<f64>,
    pub barcode: Option<String>,
    pub description: Option<String>,
    pub ui_color: Option<String>,
    pub ui_icon: Option<String>,
    pub user_importance: Option<ProductUserImportance>,
}
