// use chrono::Utc;
// use doseark_application::{ApplicationError, ApplicationResult};
// use sqlx::{Postgres, Transaction};
// use uuid::Uuid;

// pub(super) async fn insert_stock_movement(
//     tx: &mut Transaction<'_, Postgres>,
//     inventory_lot_id: Uuid,
//     product_id: Uuid,
//     movement_type: &str,
//     quantity_delta: f64,
//     quantity_after: f64,
//     reason: &str,
//     notes: Option<String>,
// ) -> ApplicationResult<()> {
//     let now = Utc::now();

//     sqlx::query(
//         r#"
//         INSERT INTO stock_movements (
//             id,
//             inventory_lot_id,
//             product_id,
//             dose_log_id,
//             occurred_at,
//             movement_type,
//             quantity_delta,
//             quantity_after,
//             reason,
//             notes,
//             created_at
//         )
//         VALUES ($1, $2, $3, NULL, $4, $5, $6, $7, $8, $9, $10)
//         "#,
//     )
//     .bind(Uuid::new_v4())
//     .bind(inventory_lot_id)
//     .bind(product_id)
//     .bind(now)
//     .bind(movement_type)
//     .bind(quantity_delta)
//     .bind(quantity_after)
//     .bind(reason)
//     .bind(notes)
//     .bind(now)
//     .execute(&mut **tx)
//     .await
//     .map_err(|err| ApplicationError::Repository(err.to_string()))?;

//     Ok(())
// }
