//! Konwersja pól produktu i wartości słownikowych SQLite

use crate::sqlite_types::{into_optional_uuid, into_uuid, SqliteUuid};
use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{DosageForm, Product, ProductCategory, ProductUserImportance};

#[derive(Debug, sqlx::FromRow)]
pub(super) struct ProductRow {
    pub(super) id: SqliteUuid,
    pub(super) brand_id: Option<SqliteUuid>,
    pub(super) name: String,
    pub(super) category: String,
    pub(super) dosage_form: String,
    pub(super) serving_unit: String,
    pub(super) serving_size_units: f64,
    pub(super) unit_weight_mg: Option<f64>,
    pub(super) barcode: Option<String>,
    pub(super) description: Option<String>,
    pub(super) ui_color: Option<String>,
    pub(super) ui_icon: Option<String>,
    pub(super) user_importance: String,
    pub(super) archived: bool,
    pub(super) created_at: chrono::DateTime<Utc>,
    pub(super) updated_at: chrono::DateTime<Utc>,
}

impl TryFrom<ProductRow> for Product {
    type Error = ApplicationError;

    fn try_from(row: ProductRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: into_uuid(row.id),
            brand_id: into_optional_uuid(row.brand_id),
            name: row.name,
            category: product_category_from_db(&row.category)?,
            dosage_form: dosage_form_from_db(&row.dosage_form)?,
            serving_unit: row.serving_unit,
            serving_size_units: row.serving_size_units,
            unit_weight_mg: row.unit_weight_mg,
            barcode: row.barcode,
            description: row.description,
            ui_color: row.ui_color,
            ui_icon: row.ui_icon,
            user_importance: product_user_importance_from_db(&row.user_importance)?,
            archived: row.archived,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

pub(super) fn product_category_to_db(value: &ProductCategory) -> &'static str {
    match value {
        ProductCategory::Supplement => "supplement",
        ProductCategory::Medication => "medication",
        ProductCategory::Food => "food",
        ProductCategory::Herb => "herb",
        ProductCategory::Other => "other",
    }
}

fn product_category_from_db(value: &str) -> ApplicationResult<ProductCategory> {
    match value {
        "supplement" => Ok(ProductCategory::Supplement),
        "medication" => Ok(ProductCategory::Medication),
        "food" => Ok(ProductCategory::Food),
        "herb" => Ok(ProductCategory::Herb),
        "other" => Ok(ProductCategory::Other),
        other => Err(ApplicationError::Repository(format!(
            "Unknown product category: {other}"
        ))),
    }
}

pub(super) fn dosage_form_to_db(value: &DosageForm) -> &'static str {
    match value {
        DosageForm::Tablet => "tablet",
        DosageForm::Capsule => "capsule",
        DosageForm::Softgel => "softgel",
        DosageForm::Powder => "powder",
        DosageForm::Liquid => "liquid",
        DosageForm::Drops => "drops",
        DosageForm::Spray => "spray",
        DosageForm::Gummy => "gummy",
        DosageForm::Other => "other",
    }
}

fn dosage_form_from_db(value: &str) -> ApplicationResult<DosageForm> {
    match value {
        "tablet" => Ok(DosageForm::Tablet),
        "capsule" => Ok(DosageForm::Capsule),
        "softgel" => Ok(DosageForm::Softgel),
        "powder" => Ok(DosageForm::Powder),
        "liquid" => Ok(DosageForm::Liquid),
        "drops" => Ok(DosageForm::Drops),
        "spray" => Ok(DosageForm::Spray),
        "gummy" => Ok(DosageForm::Gummy),
        "other" => Ok(DosageForm::Other),
        other => Err(ApplicationError::Repository(format!(
            "Unknown dosage form: {other}"
        ))),
    }
}

fn product_user_importance_from_db(value: &str) -> ApplicationResult<ProductUserImportance> {
    match value {
        "low" => Ok(ProductUserImportance::Low),
        "normal" => Ok(ProductUserImportance::Normal),
        "high" => Ok(ProductUserImportance::High),
        "critical" => Ok(ProductUserImportance::Critical),
        other => Err(ApplicationError::Repository(format!(
            "Unknown product user importance: {other}"
        ))),
    }
}

pub(super) fn product_user_importance_to_db(value: &ProductUserImportance) -> &'static str {
    match value {
        ProductUserImportance::Low => "low",
        ProductUserImportance::Normal => "normal",
        ProductUserImportance::High => "high",
        ProductUserImportance::Critical => "critical",
    }
}
