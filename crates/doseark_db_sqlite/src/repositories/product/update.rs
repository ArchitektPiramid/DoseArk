//! Aktualizacja definicji produktu bez usuwania powiązanych danych

use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{Product, ProductUserImportance, UpdateProductInput};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::mapping::{
    dosage_form_to_db, product_category_to_db, product_user_importance_to_db, ProductRow,
};

/// Updates an existing product or returns NotFound when its ID is absent
pub(super) async fn update_product(
    pool: &SqlitePool,
    id: Uuid,
    input: UpdateProductInput,
) -> ApplicationResult<Product> {
    tracing::info!(
        product_id = %id,
        product_name = %input.name,
        category = ?input.category,
        dosage_form = ?input.dosage_form,
        "Updating product",
    );

    let now = Utc::now();
    let user_importance = input
        .user_importance
        .unwrap_or(ProductUserImportance::Normal);

    let row = sqlx::query_as::<_, ProductRow>(
        r#"
        UPDATE products
        SET
            brand_id = ?2,
            name = ?3,
            category = ?4,
            dosage_form = ?5,
            serving_unit = ?6,
            serving_size_units = ?7,
            unit_weight_mg = ?8,
            barcode = ?9,
            description = ?10,
            ui_color = ?11,
            ui_icon = ?12,
            user_importance = ?13,
            archived = COALESCE(?14, archived),
            updated_at = ?15
        WHERE id = ?1
        RETURNING
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
        "#,
    )
    .bind(id.to_string())
    .bind(input.brand_id.map(|value| value.to_string()))
    .bind(input.name)
    .bind(product_category_to_db(&input.category))
    .bind(dosage_form_to_db(&input.dosage_form))
    .bind(input.serving_unit)
    .bind(input.serving_size_units)
    .bind(input.unit_weight_mg)
    .bind(input.barcode)
    .bind(input.description)
    .bind(input.ui_color)
    .bind(input.ui_icon)
    .bind(product_user_importance_to_db(&user_importance))
    .bind(input.archived)
    .bind(now)
    .fetch_optional(pool)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?
    .ok_or_else(|| ApplicationError::NotFound(format!("Product not found: {id}")))?;

    tracing::info!(product_id = %row.id.0, product_name = %row.name, "Product updated");

    Product::try_from(row)
}
