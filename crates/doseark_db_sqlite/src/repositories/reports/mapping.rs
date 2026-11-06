use chrono::{NaiveDate, Utc};
use doseark_application::ApplicationError;
use doseark_domain::{
    DailyIntakeReportRow, DoseLog, DoseLogStatus, NutrientKind, NutrientTrendDay,
    NutrientTrendEvent, NutrientTrendOption,
};
use crate::sqlite_types::{into_optional_uuid, into_uuid, SqliteUuid};

/// Daily nutrient totals returned by the SQLite aggregate
#[derive(Debug, sqlx::FromRow)]
pub(super) struct DailyIntakeReportRowSql {
    pub(super) nutrient_id: SqliteUuid,

    pub(super) nutrient_canonical_name: String,
    pub(super) nutrient_display_name: String,
    pub(super) nutrient_kind: String,

    pub(super) amount_total: f64,
    pub(super) unit: String,

    pub(super) reference_daily_amount: Option<f64>,
    pub(super) reference_daily_unit: Option<String>,
    pub(super) reference_source: Option<String>,

    pub(super) percent_of_reference_total: Option<f64>,
    pub(super) dose_count: i64,
}

impl TryFrom<DailyIntakeReportRowSql> for DailyIntakeReportRow {
    type Error = ApplicationError;

    fn try_from(row: DailyIntakeReportRowSql) -> Result<Self, Self::Error> {
        let dose_count = u32::try_from(row.dose_count).map_err(|_| {
            ApplicationError::Repository(format!("Dose count is out of range: {}", row.dose_count))
        })?;

        Ok(Self {
            nutrient_id: into_uuid(row.nutrient_id),
            nutrient_canonical_name: row.nutrient_canonical_name,
            nutrient_display_name: row.nutrient_display_name,
            nutrient_kind: nutrient_kind_from_db(&row.nutrient_kind)?,
            amount_total: row.amount_total,
            unit: row.unit,
            reference_daily_amount: row.reference_daily_amount,
            reference_daily_unit: row.reference_daily_unit,
            reference_source: row.reference_source,
            percent_of_reference_total: row.percent_of_reference_total,
            dose_count,
        })
    }
}

/// nutrient choices and counts for a trend filter
#[derive(Debug, sqlx::FromRow)]
pub(super) struct NutrientTrendOptionSql {
    pub(super) nutrient_id: SqliteUuid,
    pub(super) nutrient_canonical_name: String,
    pub(super) nutrient_display_name: String,
    pub(super) nutrient_kind: String,
    pub(super) unit: String,
    pub(super) amount_total: f64,
    pub(super) dose_count: i64,
    pub(super) recorded_day_count: i64,
}

impl TryFrom<NutrientTrendOptionSql> for NutrientTrendOption {
    type Error = ApplicationError;

    fn try_from(row: NutrientTrendOptionSql) -> Result<Self, Self::Error> {
        let dose_count = u32::try_from(row.dose_count).map_err(|_| {
            ApplicationError::Repository(format!("Dose count is out of range: {}", row.dose_count))
        })?;

        let recorded_day_count = u32::try_from(row.recorded_day_count).map_err(|_| {
            ApplicationError::Repository(format!(
                "Recorded day count is out of range: {}",
                row.recorded_day_count
            ))
        })?;

        Ok(Self {
            nutrient_id: into_uuid(row.nutrient_id),
            nutrient_canonical_name: row.nutrient_canonical_name,
            nutrient_display_name: row.nutrient_display_name,
            nutrient_kind: nutrient_kind_from_db(&row.nutrient_kind)?,
            unit: row.unit,
            amount_total: row.amount_total,
            dose_count,
            recorded_day_count,
        })
    }
}

/// One intake event with its product and lot labels
#[derive(Debug, sqlx::FromRow)]
pub(super) struct NutrientTrendEventSql {
    pub(super) dose_log_id: SqliteUuid,
    pub(super) product_id: SqliteUuid,
    pub(super) product_name: String,
    pub(super) product_brand_name: Option<String>,
    pub(super) taken_at: chrono::DateTime<Utc>,
    pub(super) amount: f64,
    pub(super) unit: String,
    pub(super) dose_units_taken: f64,
    pub(super) dose_unit: String,
    pub(super) inventory_lot_number: Option<String>,
    pub(super) notes: Option<String>,
}

impl From<NutrientTrendEventSql> for NutrientTrendEvent {
    fn from(row: NutrientTrendEventSql) -> Self {
        Self {
            dose_log_id: into_uuid(row.dose_log_id),
            product_id: into_uuid(row.product_id),
            product_name: row.product_name,
            product_brand_name: row.product_brand_name,
            taken_at: row.taken_at,
            amount: row.amount,
            unit: row.unit,
            dose_units_taken: row.dose_units_taken,
            dose_unit: row.dose_unit,
            inventory_lot_number: row.inventory_lot_number,
            notes: row.notes,
        }
    }
}

/// aggregated amounts and reference values for one day
#[derive(Debug, sqlx::FromRow)]
pub(super) struct NutrientTrendDaySql {
    pub(super) date: NaiveDate,
    pub(super) amount_total: f64,
    pub(super) unit: String,
    pub(super) reference_daily_amount: Option<f64>,
    pub(super) reference_daily_unit: Option<String>,
    pub(super) reference_source: Option<String>,
    pub(super) percent_of_reference_total: Option<f64>,
    pub(super) dose_count: i64,
}

impl TryFrom<NutrientTrendDaySql> for NutrientTrendDay {
    type Error = ApplicationError;

    fn try_from(row: NutrientTrendDaySql) -> Result<Self, Self::Error> {
        let dose_count = u32::try_from(row.dose_count).map_err(|_| {
            ApplicationError::Repository(format!("Dose count is out of range: {}", row.dose_count))
        })?;

        Ok(Self {
            date: row.date,
            amount_total: row.amount_total,
            unit: row.unit,
            reference_daily_amount: row.reference_daily_amount,
            reference_daily_unit: row.reference_daily_unit,
            reference_source: row.reference_source,
            percent_of_reference_total: row.percent_of_reference_total,
            dose_count,
        })
    }
}

/// Dose fields included in daily report details
#[derive(Debug, sqlx::FromRow)]
pub(super) struct ReportDoseLogRow {
    pub(super) id: SqliteUuid,
    pub(super) product_id: SqliteUuid,
    pub(super) inventory_lot_id: Option<SqliteUuid>,

    pub(super) taken_at: chrono::DateTime<Utc>,

    pub(super) units_taken: f64,
    pub(super) unit: String,

    pub(super) serving_multiplier: f64,

    pub(super) reason: Option<String>,
    pub(super) notes: Option<String>,

    pub(super) status: String,
    pub(super) cancelled_at: Option<chrono::DateTime<Utc>>,
    pub(super) cancellation_reason: Option<String>,

    pub(super) created_at: chrono::DateTime<Utc>,
    pub(super) updated_at: chrono::DateTime<Utc>,
}

impl TryFrom<ReportDoseLogRow> for DoseLog {
    type Error = ApplicationError;

    fn try_from(row: ReportDoseLogRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: into_uuid(row.id),
            product_id: into_uuid(row.product_id),
            inventory_lot_id: into_optional_uuid(row.inventory_lot_id),
            taken_at: row.taken_at,
            units_taken: row.units_taken,
            unit: row.unit,
            serving_multiplier: row.serving_multiplier,
            inventory_quantity_before: None,
            inventory_quantity_after: None,
            inventory_units_initial: None,
            reason: row.reason,
            notes: row.notes,
            status: dose_log_status_from_db(&row.status)?,
            cancelled_at: row.cancelled_at,
            cancellation_reason: row.cancellation_reason,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

fn dose_log_status_from_db(value: &str) -> Result<DoseLogStatus, ApplicationError> {
    match value {
        "active" => Ok(DoseLogStatus::Active),
        "cancelled" => Ok(DoseLogStatus::Cancelled),
        other => Err(ApplicationError::Repository(format!(
            "Unknown dose log status: {other}"
        ))),
    }
}

/// Convert a stored nutrient kind to its domain variant
pub(super) fn nutrient_kind_from_db(value: &str) -> Result<NutrientKind, ApplicationError> {
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
