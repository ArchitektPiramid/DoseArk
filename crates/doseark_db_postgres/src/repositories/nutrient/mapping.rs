use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{Nutrient, NutrientKind};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub(super) struct NutrientRow {
    pub(super) id: Uuid,
    pub(super) canonical_name: String,
    pub(super) display_name: String,
    pub(super) kind: String,
    pub(super) default_unit: String,
    pub(super) notes: Option<String>,
    pub(super) reference_daily_amount: Option<f64>,
    pub(super) reference_daily_unit: Option<String>,
    pub(super) reference_source: Option<String>,
    pub(super) reference_notes: Option<String>,
    pub(super) created_at: chrono::DateTime<Utc>,
    pub(super) updated_at: chrono::DateTime<Utc>,
}

impl TryFrom<NutrientRow> for Nutrient {
    type Error = ApplicationError;

    fn try_from(row: NutrientRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            canonical_name: row.canonical_name,
            display_name: row.display_name,
            kind: nutrient_kind_from_db(&row.kind)?,
            default_unit: row.default_unit,
            notes: row.notes,
            reference_daily_amount: row.reference_daily_amount,
            reference_daily_unit: row.reference_daily_unit,
            reference_source: row.reference_source,
            reference_notes: row.reference_notes,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

pub(super) fn nutrient_kind_to_db(value: &NutrientKind) -> &'static str {
    match value {
        NutrientKind::Vitamin => "vitamin",
        NutrientKind::Mineral => "mineral",
        NutrientKind::AminoAcid => "amino_acid",
        NutrientKind::FattyAcid => "fatty_acid",
        NutrientKind::HerbExtract => "herb_extract",
        NutrientKind::DrugSubstance => "drug_substance",
        NutrientKind::Probiotic => "probiotic",
        NutrientKind::Other => "other",
    }
}

fn nutrient_kind_from_db(value: &str) -> ApplicationResult<NutrientKind> {
    match value {
        "vitamin" => Ok(NutrientKind::Vitamin),
        "mineral" => Ok(NutrientKind::Mineral),
        "amino_acid" => Ok(NutrientKind::AminoAcid),
        "fatty_acid" => Ok(NutrientKind::FattyAcid),
        "herb_extract" => Ok(NutrientKind::HerbExtract),
        "drug_substance" => Ok(NutrientKind::DrugSubstance),
        "probiotic" => Ok(NutrientKind::Probiotic),
        "other" => Ok(NutrientKind::Other),
        other => Err(ApplicationError::Repository(format!(
            "Unknown nutrient kind: {other}"
        ))),
    }
}
