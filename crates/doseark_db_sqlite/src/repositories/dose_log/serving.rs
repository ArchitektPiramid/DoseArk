// Sprawdzanie mnożnika porcji
use doseark_application::{validated_serving_multiplier, ApplicationError, ApplicationResult};
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

/// Reads the persisted serving size and checks the client multiplier
/// returns the accepted fraction of a serving, not the number of units
pub(super) async fn validated_multiplier_for_product(
    tx: &mut Transaction<'_, Sqlite>,
    product_id: Uuid,
    units_taken: f64,
    unit: &str,
    requested_multiplier: f64,
) -> ApplicationResult<f64> {
    let product: Option<(f64, String)> = sqlx::query_as(
        "SELECT serving_size_units, serving_unit FROM products WHERE id = ?1",
    )
    .bind(product_id.to_string())
    .fetch_optional(&mut **tx)
    .await
    .map_err(|err| ApplicationError::Repository(err.to_string()))?;

    let (serving_size, serving_unit) = product
        .ok_or_else(|| ApplicationError::NotFound(format!("Product not found: {product_id}")))?;
    validated_serving_multiplier(
        units_taken,
        unit,
        requested_multiplier,
        serving_size,
        &serving_unit,
    )
}
