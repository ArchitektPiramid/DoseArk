mod mapping;

use async_trait::async_trait;
use doseark_application::{ApplicationError, ApplicationResult, DoseLogNutrientRepository};
use doseark_domain::DoseLogNutrient;
use sqlx::PgPool;
use uuid::Uuid;

use crate::repositories::dose_log_nutrient::mapping::DoseLogNutrientRow;

#[derive(Clone)]
pub struct PgDoseLogNutrientRepository {
    pool: PgPool,
}

impl PgDoseLogNutrientRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DoseLogNutrientRepository for PgDoseLogNutrientRepository {
    async fn list_dose_log_nutrients(
        &self,
        dose_log_id: Uuid,
    ) -> ApplicationResult<Vec<DoseLogNutrient>> {
        tracing::trace!(
            dose_log_id = %dose_log_id,
            "DB query started: list dose log nutrients",
        );

        let rows = sqlx::query_as::<_, DoseLogNutrientRow>(
            r#"
            SELECT
                dln.id,

                dln.dose_log_id,
                dln.nutrient_id,

                COALESCE(dln.nutrient_canonical_name, n.canonical_name) AS nutrient_canonical_name,
                COALESCE(dln.nutrient_display_name, n.display_name) AS nutrient_display_name,
                COALESCE(dln.nutrient_kind, n.kind) AS nutrient_kind,

                dln.amount,
                dln.unit,

                dln.reference_daily_amount,
                dln.reference_daily_unit,
                dln.reference_source,

                dln.percent_of_reference,

                dln.created_at
            FROM dose_log_nutrients dln
            LEFT JOIN nutrients n ON n.id = dln.nutrient_id
            WHERE dln.dose_log_id = $1
            ORDER BY nutrient_display_name ASC
            "#,
        )
        .bind(dose_log_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::debug!(
            dose_log_id = %dose_log_id,
            row_count = rows.len(),
            "DB query finished: list dose log nutrients",
        );

        rows.into_iter().map(DoseLogNutrient::try_from).collect()
    }
}
