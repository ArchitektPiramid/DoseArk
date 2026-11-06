use chrono::Utc;
use doseark_application::ApplicationError;
use doseark_domain::{DoseLogNutrient, NutrientKind};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub(super) struct DoseLogNutrientRow {
    pub(super) id: Uuid,

    pub(super) dose_log_id: Uuid,
    pub(super) nutrient_id: Uuid,

    pub(super) nutrient_canonical_name: String,
    pub(super) nutrient_display_name: String,
    pub(super) nutrient_kind: String,

    pub(super) amount: f64,
    pub(super) unit: String,

    pub(super) reference_daily_amount: Option<f64>,
    pub(super) reference_daily_unit: Option<String>,
    pub(super) reference_source: Option<String>,

    pub(super) percent_of_reference: Option<f64>,

    pub(super) created_at: chrono::DateTime<Utc>,
}

impl TryFrom<DoseLogNutrientRow> for DoseLogNutrient {
    type Error = ApplicationError;

    fn try_from(row: DoseLogNutrientRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            dose_log_id: row.dose_log_id,
            nutrient_id: row.nutrient_id,
            nutrient_canonical_name: row.nutrient_canonical_name,
            nutrient_display_name: row.nutrient_display_name,
            nutrient_kind: nutrient_kind_from_db(&row.nutrient_kind)?,
            amount: row.amount,
            unit: row.unit,
            reference_daily_amount: row.reference_daily_amount,
            reference_daily_unit: row.reference_daily_unit,
            reference_source: row.reference_source,
            percent_of_reference: row.percent_of_reference,
            created_at: row.created_at,
        })
    }
}

fn nutrient_kind_from_db(value: &str) -> Result<NutrientKind, ApplicationError> {
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
