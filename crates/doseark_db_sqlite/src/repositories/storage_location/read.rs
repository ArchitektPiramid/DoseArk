//! Lista lokalizacji magazynowych z obsługą rekordów archiwalnych

use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::StorageLocation;
use sqlx::SqlitePool;

use super::mapping::StorageLocationRow;

/// Lists locations, optionally including archived ones
pub(super) async fn list_storage_locations(
    pool: &SqlitePool,
    include_archived: bool,
) -> ApplicationResult<Vec<StorageLocation>> {
    tracing::trace!(include_archived, "DB query started: list storage locations",);

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
    .fetch_all(pool)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    tracing::debug!(
        row_count = rows.len(),
        "DB query finished: list storage locations",
    );

    Ok(rows.into_iter().map(StorageLocation::from).collect())
}
