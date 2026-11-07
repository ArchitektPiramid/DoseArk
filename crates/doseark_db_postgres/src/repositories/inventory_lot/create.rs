//! Creates inventory lots and records their initial stock movements atomically.

use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use doseark_domain::{CreateInventoryLotInput, InventoryLot, InventoryLotStatus};
use sqlx::PgPool;
use uuid::Uuid;

use super::mapping::{inventory_lot_status_to_db, InventoryLotRow};
use crate::repositories::stock_movement::writer::{
    insert_stock_movement, InsertStockMovementInput,
};

pub(super) async fn create_inventory_lot(
    pool: &PgPool,
    input: CreateInventoryLotInput,
) -> ApplicationResult<InventoryLot> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let units_remaining = input.units_remaining.unwrap_or(input.units_initial);
    let status = input.status.unwrap_or(InventoryLotStatus::Active);

    tracing::info!(
        lot_id = %id,
        product_id = %input.product_id,
        storage_location_id = ?input.storage_location_id,
        units_initial = input.units_initial,
        units_remaining = units_remaining,
        "Creating inventory lot",
    );

    let mut tx = pool
        .begin()
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    let row = sqlx::query_as::<_, InventoryLotRow>(
        r#"
        WITH inserted AS (
            INSERT INTO inventory_lots (
                id,
                product_id,
                lot_number,
                storage_location_id,
                storage_location,
                units_initial,
                units_remaining,
                purchase_date,
                opened_date,
                expires_at,
                price_amount,
                price_currency,
                status,
                notes,
                created_at,
                updated_at
            )
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7,
                $8,
                $9,
                $10,
                $11,
                $12,
                $13,
                $14,
                $15,
                $16
            )
            RETURNING
                id,
                product_id,
                lot_number,
                storage_location_id,
                storage_location,
                units_initial,
                units_remaining,
                purchase_date,
                opened_date,
                expires_at,
                price_amount,
                price_currency,
                status,
                notes,
                created_at,
                updated_at
        )
        SELECT
            inserted.id,
            inserted.product_id,
            inserted.lot_number,
            inserted.storage_location_id,
            COALESCE(sl.name, inserted.storage_location) AS storage_location,
            inserted.units_initial,
            inserted.units_remaining,
            inserted.purchase_date,
            inserted.opened_date,
            inserted.expires_at,
            inserted.price_amount,
            inserted.price_currency,
            inserted.status,
            inserted.notes,
            inserted.created_at,
            inserted.updated_at
        FROM inserted
        LEFT JOIN storage_locations sl ON sl.id = inserted.storage_location_id
        "#,
    )
    .bind(id)
    .bind(input.product_id)
    .bind(input.lot_number)
    .bind(input.storage_location_id)
    .bind(input.storage_location)
    .bind(input.units_initial)
    .bind(units_remaining)
    .bind(input.purchase_date)
    .bind(input.opened_date)
    .bind(input.expires_at)
    .bind(input.price_amount)
    .bind(input.price_currency)
    .bind(inventory_lot_status_to_db(&status))
    .bind(input.notes)
    .bind(now)
    .bind(now)
    .fetch_one(&mut *tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    if row.units_remaining > f64::EPSILON {
        insert_stock_movement(
            &mut tx,
            InsertStockMovementInput {
                inventory_lot_id: row.id,
                product_id: row.product_id,
                dose_log_id: None,
                movement_type: "manual_add",
                quantity_delta: row.units_remaining,
                quantity_after: row.units_remaining,
                reason: "initial_lot_create",
                notes: None,
            },
        )
        .await?;
    }

    tx.commit()
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    tracing::info!(lot_id = %row.id, "Inventory lot created");

    InventoryLot::try_from(row)
}
