//! Testy dodawania dawki i snapshotów składników

use super::fixture::Fixture;

/// Saving a dose must subtract stock and snapshot the serving's nutrients
#[tokio::test]
async fn creates_dose_with_stock_and_nutrients() {
    let db = Fixture::new().await;
    let dose = db
        .repos
        .dose_logs
        .create_dose_log(db.dose(2.0, 1.0, true))
        .await
        .expect("dose should be created");

    assert_eq!(dose.units_taken, 2.0);
    assert_eq!(dose.serving_multiplier, 1.0);
    assert_eq!(db.remaining().await, 7.0);

    let snapshots = db
        .repos
        .dose_log_nutrients
        .list_dose_log_nutrients(dose.id)
        .await
        .expect("snapshots should load");
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].amount, 100.0);
    assert_eq!(snapshots[0].nutrient_display_name, "SQLite Test Magnesium");

    let movements = db
        .repos
        .stock_movements
        .list_stock_movements_for_lot(db.lot_id)
        .await
        .expect("movements should load");
    assert_eq!(movements[0].quantity_after, 7.0);
    db.close().await;
}
