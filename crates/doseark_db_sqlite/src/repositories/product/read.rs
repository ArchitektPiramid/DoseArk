//! Odczyt katalogu produktów w kolejności dodania

use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::Product;
use sqlx::SqlitePool;

use super::mapping::ProductRow;

/// Lists every product, including archived definitions
pub(super) async fn list_products(pool: &SqlitePool) -> ApplicationResult<Vec<Product>> {
    tracing::trace!("DB query started: list products");

    let rows = sqlx::query_as::<_, ProductRow>(
        r#"
        SELECT
            id,
            brand_id,
            name,
            category,
            dosage_form,
            serving_unit,
            serving_size_units,
            unit_weight_mg,
            barcode,
            description,
            ui_color,
            ui_icon,
            user_importance,
            archived,
            created_at,
            updated_at
        FROM products
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    tracing::debug!(row_count = rows.len(), "DB query finished: list products");

    rows.into_iter().map(Product::try_from).collect()
}
