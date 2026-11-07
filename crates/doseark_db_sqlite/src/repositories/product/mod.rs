//! Repozytorium produktów katalogowych i operacji ich edycji

mod create;
mod mapping;
mod read;
mod update;

use async_trait::async_trait;
use doseark_application::{ApplicationResult, ProductRepository};
use doseark_domain::{CreateProductInput, Product, UpdateProductInput};
use sqlx::SqlitePool;
use uuid::Uuid;

/// SQLite-backed product catalog
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
        read::list_products(&self.pool).await
    }

    async fn create_product(&self, input: CreateProductInput) -> ApplicationResult<Product> {
        create::create_product(&self.pool, input).await
    }

    async fn update_product(
        &self,
        id: Uuid,
        input: UpdateProductInput,
    ) -> ApplicationResult<Product> {
        update::update_product(&self.pool, id, input).await
    }
}
