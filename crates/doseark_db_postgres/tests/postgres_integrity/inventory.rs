//! Regression checks for lot identity, dosing status and historical snapshots.

use chrono::Utc;
use doseark_application::{DoseLogRepository, InventoryLotRepository};
use doseark_db_postgres::{PgDoseLogRepository, PgInventoryLotRepository};
use doseark_domain::{
    CreateDoseLogInput, InventoryLotStatus, UpdateDoseLogInput, UpdateInventoryLotInput,
};
use sqlx::PgPool;
use uuid::Uuid;

use super::{seed_lot, seed_product, TestDatabase, TestResult};

async fn seed_nutrients(pool: &PgPool, product_id: Uuid) -> TestResult<()> {
    let nutrient_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO nutrients (id, canonical_name, display_name, kind, default_unit, created_at, updated_at) VALUES ($1, 'vitamin-c', 'Vitamin C', 'vitamin', 'mg', NOW(), NOW())",
    )
    .bind(nutrient_id)
    .execute(pool)
    .await?;
    sqlx::query(
        "INSERT INTO product_nutrients (id, product_id, nutrient_id, amount_per_serving, unit, source, created_at, updated_at) VALUES ($1, $2, $3, 10, 'mg', 'label', NOW(), NOW())",
    )
    .bind(Uuid::new_v4())
    .bind(product_id)
    .bind(nutrient_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub(super) async fn run() -> TestResult<()> {
    let db = TestDatabase::new().await?;
    let product_id = seed_product(&db.pool, "Product A").await?;
    let other_product_id = seed_product(&db.pool, "Product B").await?;
    let lot_id = seed_lot(&db.pool, product_id).await?;
    seed_nutrients(&db.pool, product_id).await?;

    let inventory = PgInventoryLotRepository::new(db.pool.clone());
    let incompatible = inventory
        .update_inventory_lot(
            lot_id,
            UpdateInventoryLotInput {
                product_id: other_product_id,
                lot_number: None,
                storage_location: None,
                storage_location_id: None,
                units_initial: 10.0,
                units_remaining: 10.0,
                purchase_date: None,
                opened_date: None,
                expires_at: None,
                price_amount: None,
                price_currency: None,
                status: InventoryLotStatus::Active,
                notes: None,
            },
        )
        .await;
    assert!(incompatible.is_err(), "Changing a lot's product must fail");
    let stored_product: Uuid =
        sqlx::query_scalar("SELECT product_id FROM inventory_lots WHERE id = $1")
            .bind(lot_id)
            .fetch_one(&db.pool)
            .await?;
    assert_eq!(stored_product, product_id);

    let dosing = PgDoseLogRepository::new(db.pool.clone());
    let original = dosing
        .create_dose_log(CreateDoseLogInput {
            product_id,
            inventory_lot_id: Some(lot_id),
            taken_at: Utc::now(),
            units_taken: 2.0,
            unit: "tablet".into(),
            serving_multiplier: 2.0,
            reason: None,
            notes: Some("before".into()),
        })
        .await?;

    let (snapshot_id, original_amount): (Uuid, f64) =
        sqlx::query_as("SELECT id, amount FROM dose_log_nutrients WHERE dose_log_id = $1")
            .bind(original.id)
            .fetch_one(&db.pool)
            .await?;
    assert_eq!(original_amount, 20.0);
    sqlx::query("UPDATE product_nutrients SET amount_per_serving = 99 WHERE product_id = $1")
        .bind(product_id)
        .execute(&db.pool)
        .await?;
    sqlx::query("UPDATE inventory_lots SET status = 'on_hold' WHERE id = $1")
        .bind(lot_id)
        .execute(&db.pool)
        .await?;

    let rejected = dosing
        .create_dose_log(CreateDoseLogInput {
            product_id,
            inventory_lot_id: Some(lot_id),
            taken_at: Utc::now(),
            units_taken: 1.0,
            unit: "tablet".into(),
            serving_multiplier: 1.0,
            reason: None,
            notes: None,
        })
        .await;
    assert!(rejected.is_err(), "Held lots must not be consumed");

    let count_before: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM stock_movements WHERE dose_log_id = $1")
            .bind(original.id)
            .fetch_one(&db.pool)
            .await?;
    let edited = dosing
        .update_dose_log(
            original.id,
            UpdateDoseLogInput {
                product_id,
                inventory_lot_id: Some(lot_id),
                taken_at: original.taken_at,
                units_taken: 2.0,
                unit: "tablet".into(),
                serving_multiplier: 2.0,
                reason: None,
                notes: Some("after".into()),
            },
        )
        .await?;
    assert_eq!(edited.notes.as_deref(), Some("after"));
    let count_after: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM stock_movements WHERE dose_log_id = $1")
            .bind(original.id)
            .fetch_one(&db.pool)
            .await?;
    assert_eq!(
        count_before, count_after,
        "Metadata edits must not move stock"
    );
    let (same_id, amount): (Uuid, f64) =
        sqlx::query_as("SELECT id, amount FROM dose_log_nutrients WHERE dose_log_id = $1")
            .bind(original.id)
            .fetch_one(&db.pool)
            .await?;
    assert_eq!((same_id, amount), (snapshot_id, 20.0));

    sqlx::query("UPDATE inventory_lots SET status = 'active' WHERE id = $1")
        .bind(lot_id)
        .execute(&db.pool)
        .await?;
    dosing
        .update_dose_log(
            original.id,
            UpdateDoseLogInput {
                product_id,
                inventory_lot_id: Some(lot_id),
                taken_at: original.taken_at,
                units_taken: 3.0,
                unit: "tablet".into(),
                serving_multiplier: 3.0,
                reason: None,
                notes: Some("rescaled".into()),
            },
        )
        .await?;
    let (same_id, amount): (Uuid, f64) =
        sqlx::query_as("SELECT id, amount FROM dose_log_nutrients WHERE dose_log_id = $1")
            .bind(original.id)
            .fetch_one(&db.pool)
            .await?;
    assert_eq!((same_id, amount), (snapshot_id, 30.0));

    sqlx::query("UPDATE inventory_lots SET status = 'on_hold' WHERE id = $1")
        .bind(lot_id)
        .execute(&db.pool)
        .await?;
    dosing.cancel_dose_log(original.id).await?;
    let (remaining, status): (f64, String) =
        sqlx::query_as("SELECT units_remaining, status FROM inventory_lots WHERE id = $1")
            .bind(lot_id)
            .fetch_one(&db.pool)
            .await?;
    assert_eq!(remaining, 10.0);
    assert_eq!(
        status, "on_hold",
        "A correction must not reactivate held stock"
    );

    db.cleanup().await
}
