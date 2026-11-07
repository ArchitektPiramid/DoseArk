//! Dodawanie lokalizacji z nazwą i ustawieniami wyglądu

use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{CreateStorageLocationInput, StorageLocation};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::mapping::StorageLocationRow;

/// Creates a location using a trimmed name and default sort order
pub(super) async fn create_storage_location(
    pool: &SqlitePool,
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
    .fetch_one(pool)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    tracing::info!(
        storage_location_id = %row.id.0,
        "Storage location created",
    );

    Ok(StorageLocation::from(row))
}
