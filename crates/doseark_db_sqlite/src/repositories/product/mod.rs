mod mapping;

use async_trait::async_trait;
use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult, ProductRepository};
use doseark_domain::{
    CreateProductInput, Product, ProductUserImportance, UpdateProductInput,
};
use sqlx::SqlitePool;
use uuid::Uuid;

use self::mapping::{
    dosage_form_to_db, product_category_to_db, product_user_importance_to_db, ProductRow,
};

#[derive(Clone)]
pub struct SqliteProductRepository {
    pool: SqlitePool,
}

impl SqliteProductRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepository for SqliteProductRepository {
    async fn list_products(&self) -> ApplicationResult<Vec<Product>> {
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
        .fetch_all(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::debug!(row_count = rows.len(), "DB query finished: list products");

        rows.into_iter().map(Product::try_from).collect()
    }

    async fn create_product(&self, input: CreateProductInput) -> ApplicationResult<Product> {
        tracing::info!(
            product_name = %input.name,
            category = ?input.category,
            dosage_form = ?input.dosage_form,
            "Creating product",
        );

        let now = Utc::now();
        let id = Uuid::new_v4();
        let user_importance = input
            .user_importance
            .unwrap_or(ProductUserImportance::Normal);

        let row = sqlx::query_as::<_, ProductRow>(
            r#"
            INSERT INTO products (
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
            )
            VALUES (
                ?1,
                ?2,
                ?3,
                ?4,
                ?5,
                ?6,
                ?7,
                ?8,
                ?9,
                ?10,
                ?11,
                ?12,
                ?13,
                FALSE,
                ?14,
                ?15
            )
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
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::info!(product_id = %row.id.0, product_name = %row.name, "Product created");

        Product::try_from(row)
    }

    async fn update_product(&self, id: Uuid, input: UpdateProductInput) -> ApplicationResult<Product> {
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
                updated_at = ?14
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
        .bind(now)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?
        .ok_or_else(|| ApplicationError::NotFound(format!("Product not found: {id}")))?;

        tracing::info!(product_id = %row.id.0, product_name = %row.name, "Product updated");

        Product::try_from(row)
    }
}
