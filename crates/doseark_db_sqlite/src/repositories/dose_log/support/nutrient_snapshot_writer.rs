//! Utrwalanie historycznych ilości składników przyjętej dawki

use crate::sqlite_types::SqliteUuid;
use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

/// Product composition fields captured at the time of a dose
#[derive(Debug, sqlx::FromRow)]
struct ProductNutrientSnapshotSourceRow {
    nutrient_id: SqliteUuid,
    nutrient_canonical_name: String,
    nutrient_display_name: String,
    nutrient_kind: String,
    amount_per_serving: f64,
    unit: String,
    label_daily_value_percent: Option<f64>,
    label_reference_amount: Option<f64>,
    label_reference_unit: Option<String>,
    reference_daily_amount: Option<f64>,
    reference_daily_unit: Option<String>,
    reference_source: Option<String>,
}

/// Loads product nutrients and stores their calculated dose amounts
/// The multiplier comes from the validated product serving size
pub(in crate::repositories::dose_log) async fn insert_dose_log_nutrient_snapshots(
    tx: &mut Transaction<'_, Sqlite>,
    dose_log_id: Uuid,
    product_id: Uuid,
    serving_multiplier: f64,
) -> ApplicationResult<()> {
    let rows = sqlx::query_as::<_, ProductNutrientSnapshotSourceRow>(
        r#"
        SELECT
            pn.nutrient_id,
            n.canonical_name AS nutrient_canonical_name,
            n.display_name AS nutrient_display_name,
            n.kind AS nutrient_kind,
            pn.amount_per_serving,
            pn.unit,
            pn.label_daily_value_percent,
            pn.label_reference_amount,
            pn.label_reference_unit,
            n.reference_daily_amount,
            n.reference_daily_unit,
            n.reference_source
        FROM product_nutrients pn
        INNER JOIN nutrients n ON n.id = pn.nutrient_id
        WHERE pn.product_id = ?1
        ORDER BY n.display_name ASC
        "#,
    )
    .bind(product_id.to_string())
    .fetch_all(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    tracing::debug!(
        dose_log_id = %dose_log_id,
        product_id = %product_id,
        row_count = rows.len(),
        "Creating SQLite dose nutrient snapshots",
    );

    let now = Utc::now();

    for row in rows {
        let amount = row.amount_per_serving * serving_multiplier;
        let reference = resolve_reference(&row, amount);

        sqlx::query(
            r#"
            INSERT INTO dose_log_nutrients (
                id,
                dose_log_id,
                nutrient_id,
                nutrient_canonical_name,
                nutrient_display_name,
                nutrient_kind,
                amount,
                unit,
                reference_daily_amount,
                reference_daily_unit,
                reference_source,
                percent_of_reference,
                created_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
            "#,
        )
        .bind(Uuid::new_v4().to_string())
        .bind(dose_log_id.to_string())
        .bind(row.nutrient_id.0.to_string())
        .bind(row.nutrient_canonical_name)
        .bind(row.nutrient_display_name)
        .bind(row.nutrient_kind)
        .bind(amount)
        .bind(row.unit)
        .bind(reference.reference_daily_amount)
        .bind(reference.reference_daily_unit)
        .bind(reference.reference_source)
        .bind(reference.percent_of_reference)
        .bind(now)
        .execute(&mut **tx)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;
    }

    Ok(())
}

/// Scale original amounts without replacing historical nutrient metadata
/// The ratio refers to the recorded unit count, not the current product recipe
pub(in crate::repositories::dose_log) async fn rescale_dose_log_nutrient_snapshots(
    tx: &mut Transaction<'_, Sqlite>,
    dose_log_id: Uuid,
    units_ratio: f64,
) -> ApplicationResult<()> {
    sqlx::query(
        r#"
        UPDATE dose_log_nutrients
        SET
            amount = amount * ?2,
            percent_of_reference = percent_of_reference * ?2
        WHERE dose_log_id = ?1
        "#,
    )
    .bind(dose_log_id.to_string())
    .bind(units_ratio)
    .execute(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    Ok(())
}

/// Reference value and its source after resolving overrides
struct ResolvedReference {
    reference_daily_amount: Option<f64>,
    reference_daily_unit: Option<String>,
    reference_source: Option<String>,
    percent_of_reference: Option<f64>,
}

/// Prefers label-specific reference amounts before global values
fn resolve_reference(row: &ProductNutrientSnapshotSourceRow, amount: f64) -> ResolvedReference {
    if let (Some(reference_amount), Some(reference_unit)) = (
        row.reference_daily_amount,
        row.reference_daily_unit.as_deref(),
    ) {
        if reference_amount > 0.0 && reference_unit == row.unit {
            return ResolvedReference {
                reference_daily_amount: Some(reference_amount),
                reference_daily_unit: Some(reference_unit.to_string()),
                reference_source: Some(
                    row.reference_source
                        .clone()
                        .unwrap_or_else(|| "global_nutrient_reference".to_string()),
                ),
                percent_of_reference: Some((amount / reference_amount) * 100.0),
            };
        }
    }

    if let (Some(reference_amount), Some(reference_unit)) = (
        row.label_reference_amount,
        row.label_reference_unit.as_deref(),
    ) {
        if reference_amount > 0.0 && reference_unit == row.unit {
            return ResolvedReference {
                reference_daily_amount: Some(reference_amount),
                reference_daily_unit: Some(reference_unit.to_string()),
                reference_source: Some("product_label_reference".to_string()),
                percent_of_reference: Some((amount / reference_amount) * 100.0),
            };
        }
    }

    if let Some(label_percent) = row.label_daily_value_percent {
        if label_percent > 0.0 && row.amount_per_serving > 0.0 {
            let inferred_reference_amount = row.amount_per_serving / (label_percent / 100.0);

            return ResolvedReference {
                reference_daily_amount: Some(inferred_reference_amount),
                reference_daily_unit: Some(row.unit.clone()),
                reference_source: Some("product_label_percent".to_string()),
                percent_of_reference: Some(label_percent * (amount / row.amount_per_serving)),
            };
        }
    }

    ResolvedReference {
        reference_daily_amount: None,
        reference_daily_unit: None,
        reference_source: None,
        percent_of_reference: None,
    }
}
