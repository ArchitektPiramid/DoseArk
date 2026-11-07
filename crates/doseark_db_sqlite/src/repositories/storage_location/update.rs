//! Modyfikacja danych lokalizacji i oznaczanie jej jako archiwalnej

use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{StorageLocation, UpdateStorageLocationInput};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::mapping::StorageLocationRow;

/// Applies edited location fields without changing linked inventory lots
pub(super) async fn update_storage_location(
    pool: &SqlitePool,
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
    .fetch_optional(pool)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?
    .ok_or_else(|| ApplicationError::NotFound(format!("Storage location not found: {id}")))?;

    tracing::info!(
        storage_location_id = %row.id.0,
        "Storage location updated",
    );

    Ok(StorageLocation::from(row))
}

/// Marks the location archived while retaining its references
pub(super) async fn archive_storage_location(
    pool: &SqlitePool,
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
    .fetch_optional(pool)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?
    .ok_or_else(|| ApplicationError::NotFound(format!("Storage location not found: {id}")))?;

    Ok(StorageLocation::from(row))
}
