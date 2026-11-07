//! Tworzenie i aktualizacja słownika składników

mod mapping;

use async_trait::async_trait;
use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult, NutrientRepository};
use doseark_domain::{CreateNutrientInput, Nutrient, UpdateNutrientInput};
use sqlx::SqlitePool;
use uuid::Uuid;

use self::mapping::{nutrient_kind_to_db, NutrientRow};

#[derive(Clone)]
pub struct SqliteNutrientRepository {
    pool: SqlitePool,
}

impl SqliteNutrientRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NutrientRepository for SqliteNutrientRepository {
    async fn list_nutrients(&self) -> ApplicationResult<Vec<Nutrient>> {
        tracing::trace!("DB query started: list nutrients");

        let rows = sqlx::query_as::<_, NutrientRow>(
            r#"
            SELECT
                id,
                canonical_name,
                display_name,
                kind,
                default_unit,
                notes,
                reference_daily_amount,
                reference_daily_unit,
                reference_source,
                reference_notes,
                created_at,
                updated_at
            FROM nutrients
            ORDER BY display_name ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::debug!(row_count = rows.len(), "DB query finished: list nutrients");

        rows.into_iter().map(Nutrient::try_from).collect()
    }

    /// Adds a reusable nutrient definition to the catalog
    async fn create_nutrient(&self, input: CreateNutrientInput) -> ApplicationResult<Nutrient> {
        tracing::info!(
            canonical_name = %input.canonical_name,
            display_name = %input.display_name,
            kind = ?input.kind,
            "Creating nutrient",
        );

        let now = Utc::now();
        let id = Uuid::new_v4();

        let row = sqlx::query_as::<_, NutrientRow>(
            r#"
            INSERT INTO nutrients (
                id,
                canonical_name,
                display_name,
                kind,
                default_unit,
                notes,
                reference_daily_amount,
                reference_daily_unit,
                reference_source,
                reference_notes,
                created_at,
                updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            RETURNING
                id,
                canonical_name,
                display_name,
                kind,
                default_unit,
                notes,
                reference_daily_amount,
                reference_daily_unit,
                reference_source,
                reference_notes,
                created_at,
                updated_at
            "#,
        )
        .bind(id.to_string())
        .bind(input.canonical_name)
        .bind(input.display_name)
        .bind(nutrient_kind_to_db(&input.kind))
        .bind(input.default_unit)
        .bind(input.notes)
        .bind(input.reference_daily_amount)
        .bind(input.reference_daily_unit)
        .bind(input.reference_source)
        .bind(input.reference_notes)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::info!(nutrient_id = %row.id.0, display_name = %row.display_name, "Nutrient created");

        Nutrient::try_from(row)
    }

    /// Updates nutrient metadata while preserving its identifier
    async fn update_nutrient(
        &self,
        id: Uuid,
        input: UpdateNutrientInput,
    ) -> ApplicationResult<Nutrient> {
        tracing::info!(
            nutrient_id = %id,
            canonical_name = %input.canonical_name,
            display_name = %input.display_name,
            kind = ?input.kind,
            "Updating nutrient",
        );

        let now = Utc::now();

        let row = sqlx::query_as::<_, NutrientRow>(
            r#"
            UPDATE nutrients
            SET
                canonical_name = ?2,
                display_name = ?3,
                kind = ?4,
                default_unit = ?5,
                notes = ?6,
                reference_daily_amount = ?7,
                reference_daily_unit = ?8,
                reference_source = ?9,
                reference_notes = ?10,
                updated_at = ?11
            WHERE id = ?1
            RETURNING
                id,
                canonical_name,
                display_name,
                kind,
                default_unit,
                notes,
                reference_daily_amount,
                reference_daily_unit,
                reference_source,
                reference_notes,
                created_at,
                updated_at
            "#,
        )
        .bind(id.to_string())
        .bind(input.canonical_name)
        .bind(input.display_name)
        .bind(nutrient_kind_to_db(&input.kind))
        .bind(input.default_unit)
        .bind(input.notes)
        .bind(input.reference_daily_amount)
        .bind(input.reference_daily_unit)
        .bind(input.reference_source)
        .bind(input.reference_notes)
        .bind(now)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?
        .ok_or_else(|| ApplicationError::NotFound(format!("Nutrient not found: {id}")))?;

        tracing::info!(nutrient_id = %row.id.0, display_name = %row.display_name, "Nutrient updated");

        Nutrient::try_from(row)
    }
}
