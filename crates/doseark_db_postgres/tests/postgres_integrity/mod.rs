//! Isolated test schemas and shared fixtures for PostgreSQL repository checks.

mod inventory;

use std::{env, error::Error};

use sqlx::{postgres::PgPoolOptions, PgPool};
use uuid::Uuid;

pub(super) type TestResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

/// Runs the inventory scenarios without exposing their private fixtures.
pub(super) async fn run_inventory_and_snapshot_regressions() -> TestResult<()> {
    inventory::run().await
}

pub(super) struct TestDatabase {
    pub pool: PgPool,
    admin: PgPool,
    schema: String,
}

impl TestDatabase {
    /// Runs the real migrations in a uniquely named schema, never in public.
    pub async fn new() -> TestResult<Self> {
        let url = env::var("DOSEARK_PG_TEST_URL")?;
        let schema = format!("doseark_test_{}", Uuid::new_v4().simple());
        let admin = PgPool::connect(&url).await?;
        sqlx::query(&format!("CREATE SCHEMA \"{schema}\""))
            .execute(&admin)
            .await?;

        let connection_schema = schema.clone();
        let pool = PgPoolOptions::new()
            .max_connections(3)
            .after_connect(move |connection, _| {
                let schema = connection_schema.clone();
                Box::pin(async move {
                    sqlx::query(&format!("SET search_path TO \"{schema}\""))
                        .execute(connection)
                        .await?;
                    Ok(())
                })
            })
            .connect(&url)
            .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self {
            pool,
            admin,
            schema,
        })
    }

    pub async fn cleanup(self) -> TestResult<()> {
        self.pool.close().await;
        sqlx::query(&format!("DROP SCHEMA \"{}\" CASCADE", self.schema))
            .execute(&self.admin)
            .await?;
        self.admin.close().await;
        Ok(())
    }
}

/// Creates a minimal supplement that can be used for inventory and intake.
pub(super) async fn seed_product(pool: &PgPool, name: &str) -> TestResult<Uuid> {
    let id = Uuid::new_v4();
    sqlx::query(
        r#"INSERT INTO products
        (id, name, category, dosage_form, serving_unit, serving_size_units, created_at, updated_at)
        VALUES ($1, $2, 'supplement', 'tablet', 'tablet', 1, NOW(), NOW())"#,
    )
    .bind(id)
    .bind(name)
    .execute(pool)
    .await?;
    Ok(id)
}

pub(super) async fn seed_lot(pool: &PgPool, product_id: Uuid) -> TestResult<Uuid> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO inventory_lots (id, product_id, units_initial, units_remaining, status, created_at, updated_at) VALUES ($1, $2, 10, 10, 'active', NOW(), NOW())",
    )
    .bind(id)
    .bind(product_id)
    .execute(pool)
    .await?;
    Ok(id)
}
