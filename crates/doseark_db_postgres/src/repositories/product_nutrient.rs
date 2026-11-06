use async_trait::async_trait;
use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult, ProductNutrientRepository};
use doseark_domain::{ProductNutrient, SaveProductNutrientInput};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgProductNutrientRepository {
    pool: PgPool,
}

impl PgProductNutrientRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductNutrientRepository for PgProductNutrientRepository {
    async fn list_product_nutrients(&self, product_id: Uuid) -> ApplicationResult<Vec<ProductNutrient>> {
        tracing::trace!(
            product_id = %product_id,
            "DB query started: list product nutrients",
        );

        let rows = sqlx::query_as::<_, ProductNutrientRow>(
            r#"
            SELECT
                id,
                product_id,
                nutrient_id,
                amount_per_serving,
                unit,
                label_daily_value_percent,
                label_reference_amount,
                label_reference_unit,
                source,
                notes,
                created_at,
                updated_at
            FROM product_nutrients
            WHERE product_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(product_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::debug!(
            product_id = %product_id,
            row_count = rows.len(),
            "DB query finished: list product nutrients",
        );

        Ok(rows.into_iter().map(ProductNutrient::from).collect())
    }

    async fn replace_product_nutrients(
        &self,
        product_id: Uuid,
        items: Vec<SaveProductNutrientInput>,
    ) -> ApplicationResult<Vec<ProductNutrient>> {
        tracing::info!(
            product_id = %product_id,
            item_count = items.len(),
            "Replacing product composition",
        );

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        sqlx::query("DELETE FROM product_nutrients WHERE product_id = $1")
            .bind(product_id)
            .execute(&mut *tx)
            .await
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        let now = Utc::now();

        for item in items {
            sqlx::query(
                r#"
                INSERT INTO product_nutrients (
                    id,
                    product_id,
                    nutrient_id,
                    amount_per_serving,
                    unit,
                    label_daily_value_percent,
                    label_reference_amount,
                    label_reference_unit,
                    source,
                    notes,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(product_id)
            .bind(item.nutrient_id)
            .bind(item.amount_per_serving)
            .bind(item.unit)
            .bind(item.label_daily_value_percent)
            .bind(item.label_reference_amount)
            .bind(item.label_reference_unit)
            .bind(item.source)
            .bind(item.notes)
            .bind(now)
            .bind(now)
            .execute(&mut *tx)
            .await
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        }

        tx.commit()
            .await
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;

        tracing::info!(product_id = %product_id, "Product composition replaced");

        self.list_product_nutrients(product_id).await
    }
}

#[derive(Debug, sqlx::FromRow)]
struct ProductNutrientRow {
    id: Uuid,
    product_id: Uuid,
    nutrient_id: Uuid,
    amount_per_serving: f64,
    unit: String,
    label_daily_value_percent: Option<f64>,
    label_reference_amount: Option<f64>,
    label_reference_unit: Option<String>,
    source: String,
    notes: Option<String>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl From<ProductNutrientRow> for ProductNutrient {
    fn from(row: ProductNutrientRow) -> Self {
        Self {
            id: row.id,
            product_id: row.product_id,
            nutrient_id: row.nutrient_id,
            amount_per_serving: row.amount_per_serving,
            unit: row.unit,
            label_daily_value_percent: row.label_daily_value_percent,
            label_reference_amount: row.label_reference_amount,
            label_reference_unit: row.label_reference_unit,
            source: row.source,
            notes: row.notes,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
