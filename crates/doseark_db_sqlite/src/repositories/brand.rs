//! Operacje katalogowe na markach produktów

use crate::sqlite_types::{into_uuid, SqliteUuid};
use async_trait::async_trait;
use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult, BrandRepository};
use doseark_domain::{Brand, CreateBrandInput, UpdateBrandInput};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Clone)]
pub struct SqliteBrandRepository {
    pool: SqlitePool,
}

impl SqliteBrandRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BrandRepository for SqliteBrandRepository {
    async fn list_brands(&self) -> ApplicationResult<Vec<Brand>> {
        tracing::trace!("DB query started: list brands");

        let rows = sqlx::query_as::<_, BrandRow>(
            r#"
            SELECT id, name, country, website, notes, created_at, updated_at
            FROM brands
            ORDER BY name ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::debug!(row_count = rows.len(), "DB query finished: list brands");

        Ok(rows.into_iter().map(Brand::from).collect())
    }

    /// Saves a brand and returns its assigned identifier
    async fn create_brand(&self, input: CreateBrandInput) -> ApplicationResult<Brand> {
        tracing::info!(brand_name = %input.name, "Creating brand");

        let now = Utc::now();
        let id = Uuid::new_v4();

        let row = sqlx::query_as::<_, BrandRow>(
            r#"
            INSERT INTO brands (id, name, country, website, notes, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            RETURNING id, name, country, website, notes, created_at, updated_at
            "#,
        )
        .bind(id.to_string())
        .bind(input.name)
        .bind(input.country)
        .bind(input.website)
        .bind(input.notes)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::info!(brand_id = %row.id.0, brand_name = %row.name, "Brand created");

        Ok(row.into())
    }

    /// Updates brand details without modifying linked products
    async fn update_brand(&self, id: Uuid, input: UpdateBrandInput) -> ApplicationResult<Brand> {
        tracing::info!(brand_id = %id, brand_name = %input.name, "Updating brand");

        let now = Utc::now();

        let row = sqlx::query_as::<_, BrandRow>(
            r#"
            UPDATE brands
            SET
                name = ?2,
                country = ?3,
                website = ?4,
                notes = ?5,
                updated_at = ?6
            WHERE id = ?1
            RETURNING id, name, country, website, notes, created_at, updated_at
            "#,
        )
        .bind(id.to_string())
        .bind(input.name)
        .bind(input.country)
        .bind(input.website)
        .bind(input.notes)
        .bind(now)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?
        .ok_or_else(|| ApplicationError::NotFound(format!("Brand not found: {id}")))?;

        tracing::info!(brand_id = %row.id.0, brand_name = %row.name, "Brand updated");

        Ok(row.into())
    }
}

#[derive(Debug, sqlx::FromRow)]
struct BrandRow {
    id: SqliteUuid,
    name: String,
    country: Option<String>,
    website: Option<String>,
    notes: Option<String>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl From<BrandRow> for Brand {
    fn from(row: BrandRow) -> Self {
        Self {
            id: into_uuid(row.id),
            name: row.name,
            country: row.country,
            website: row.website,
            notes: row.notes,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
