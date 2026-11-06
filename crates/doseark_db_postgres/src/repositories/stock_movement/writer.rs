use chrono::Utc;
use doseark_application::{ApplicationError, ApplicationResult};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub(crate) struct InsertStockMovementInput<'a> {
    pub inventory_lot_id: Uuid,
    pub product_id: Uuid,
    pub dose_log_id: Option<Uuid>,
    pub movement_type: &'a str,
    pub quantity_delta: f64,
    pub quantity_after: f64,
    pub reason: &'a str,
    pub notes: Option<String>,
}

pub(crate) async fn insert_stock_movement(
    tx: &mut Transaction<'_, Postgres>,
    input: InsertStockMovementInput<'_>,
) -> ApplicationResult<()> {
    let now = Utc::now();

    sqlx::query(
        r#"
        INSERT INTO stock_movements (
            id,
            inventory_lot_id,
            product_id,
            dose_log_id,
            occurred_at,
            movement_type,
            quantity_delta,
            quantity_after,
            reason,
            notes,
            created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(input.inventory_lot_id)
    .bind(input.product_id)
    .bind(input.dose_log_id)
    .bind(now)
    .bind(input.movement_type)
    .bind(input.quantity_delta)
    .bind(input.quantity_after)
    .bind(input.reason)
    .bind(input.notes)
    .bind(now)
    .execute(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    Ok(())
}
