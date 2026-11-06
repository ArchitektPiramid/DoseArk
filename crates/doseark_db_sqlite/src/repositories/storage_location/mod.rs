mod mapping;

use async_trait::async_trait;
use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult, StorageLocationRepository};
use doseark_domain::{
    CreateStorageLocationInput, StorageLocation, UpdateStorageLocationInput,
};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::repositories::storage_location::mapping::StorageLocationRow;

#[derive(Clone)]
pub struct SqliteStorageLocationRepository {
    pool: SqlitePool,
}

impl SqliteStorageLocationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StorageLocationRepository for SqliteStorageLocationRepository {
    async fn list_storage_locations(
        &self,
        include_archived: bool,
    ) -> ApplicationResult<Vec<StorageLocation>> {
        tracing::trace!(
            include_archived,
            "DB query started: list storage locations",
        );

        let rows = sqlx::query_as::<_, StorageLocationRow>(
            r#"
            SELECT
                id,
                name,
                description,
                color,
                icon,
                sort_order,
                is_archived,
                created_at,
                updated_at
            FROM storage_locations
            WHERE (?1 = TRUE OR is_archived = FALSE)
            ORDER BY sort_order ASC, name ASC
            "#,
        )
        .bind(include_archived)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::debug!(
            row_count = rows.len(),
            "DB query finished: list storage locations",
        );

        Ok(rows.into_iter().map(StorageLocation::from).collect())
    }

    async fn create_storage_location(
        &self,
        input: CreateStorageLocationInput,
    ) -> ApplicationResult<StorageLocation> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        tracing::info!(
            storage_location_id = %id,
            name = %input.name,
            "Creating storage location",
        );

        let row = sqlx::query_as::<_, StorageLocationRow>(
            r#"
            INSERT INTO storage_locations (
                id,
                name,
                description,
                color,
                icon,
                sort_order,
                is_archived,
                created_at,
                updated_at
            )
            VALUES (
                ?1,
                ?2,
                ?3,
                ?4,
                ?5,
                ?6,
                FALSE,
                ?7,
                ?8
            )
            RETURNING
                id,
                name,
                description,
                color,
                icon,
                sort_order,
                is_archived,
                created_at,
                updated_at
            "#,
        )
        .bind(id.to_string())
        .bind(input.name.trim())
        .bind(input.description)
        .bind(input.color)
        .bind(input.icon)
        .bind(input.sort_order.unwrap_or(0))
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::info!(
            storage_location_id = %row.id.0,
            "Storage location created",
        );

        Ok(StorageLocation::from(row))
    }

    async fn update_storage_location(
        &self,
        id: Uuid,
        input: UpdateStorageLocationInput,
    ) -> ApplicationResult<StorageLocation> {
        let now = Utc::now();

        tracing::info!(
            storage_location_id = %id,
            name = %input.name,
            "Updating storage location",
        );

        let row = sqlx::query_as::<_, StorageLocationRow>(
            r#"
            UPDATE storage_locations
            SET
                name = ?2,
                description = ?3,
                color = ?4,
                icon = ?5,
                sort_order = ?6,
                is_archived = ?7,
                updated_at = ?8
            WHERE id = ?1
            RETURNING
                id,
                name,
                description,
                color,
                icon,
                sort_order,
                is_archived,
                created_at,
                updated_at
            "#,
        )
        .bind(id.to_string())
        .bind(input.name.trim())
        .bind(input.description)
        .bind(input.color)
        .bind(input.icon)
        .bind(input.sort_order.unwrap_or(0))
        .bind(input.is_archived)
        .bind(now)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?
        .ok_or_else(|| ApplicationError::NotFound(format!("Storage location not found: {id}")))?;

        tracing::info!(
            storage_location_id = %row.id.0,
            "Storage location updated",
        );

        Ok(StorageLocation::from(row))
    }

    async fn archive_storage_location(
        &self,
        id: Uuid,
    ) -> ApplicationResult<StorageLocation> {
        let now = Utc::now();

        tracing::info!(
            storage_location_id = %id,
            "Archiving storage location",
        );

        let row = sqlx::query_as::<_, StorageLocationRow>(
            r#"
            UPDATE storage_locations
            SET
                is_archived = TRUE,
                updated_at = ?2
            WHERE id = ?1
            RETURNING
                id,
                name,
                description,
                color,
                icon,
                sort_order,
                is_archived,
                created_at,
                updated_at
            "#,
        )
        .bind(id.to_string())
        .bind(now)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?
        .ok_or_else(|| ApplicationError::NotFound(format!("Storage location not found: {id}")))?;

        Ok(StorageLocation::from(row))
    }
}
