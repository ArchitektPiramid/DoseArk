//! Testy edycji dawki oraz korekty zapasu

use doseark_application::ApplicationError;

use super::fixture::Fixture;

/// Failed edits roll back both the stock correction and snapshot changes
#[tokio::test]
async fn edit_rolls_back_when_new_amount_exceeds_stock() {
    let db = Fixture::new().await;
    let dose = db
        .repos
        .dose_logs
        .create_dose_log(db.dose(2.0, 1.0, true))
        .await
        .unwrap();

    let invalid = db
        .repos
        .dose_logs
        .update_dose_log(dose.id, db.edit(&dose, 20.0, 10.0))
        .await;
    assert!(matches!(invalid, Err(ApplicationError::Validation(_))));
    assert_eq!(db.remaining().await, 7.0);

    let snapshots = db
        .repos
        .dose_log_nutrients
        .list_dose_log_nutrients(dose.id)
        .await
        .unwrap();
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].amount, 100.0);

    let logs = db
        .repos
        .dose_logs
        .list_dose_logs(doseark_domain::DoseLogListInput {
            limit: 20,
            offset: 0,
            product_id: Some(db.product_id),
        })
        .await
        .unwrap();
    assert_eq!(logs.items[0].units_taken, 2.0);
    db.close().await;
}

#[tokio::test]
async fn edit_recalculates_stock_and_nutrients() {
    let db = Fixture::new().await;
    let dose = db
        .repos
        .dose_logs
        .create_dose_log(db.dose(2.0, 1.0, true))
        .await
        .unwrap();

    let updated = db
        .repos
        .dose_logs
        .update_dose_log(dose.id, db.edit(&dose, 3.0, 1.5))
        .await
        .expect("dose should update");
    assert_eq!(updated.units_taken, 3.0);
    assert_eq!(db.remaining().await, 6.0);

    let nutrients = db
        .repos
        .dose_log_nutrients
        .list_dose_log_nutrients(dose.id)
        .await
        .unwrap();
    assert_eq!(nutrients.len(), 1);
    assert_eq!(nutrients[0].amount, 150.0);

    let movements = db
        .repos
        .stock_movements
        .list_stock_movements_for_lot(db.lot_id)
        .await
        .unwrap();
    assert_eq!(movements[0].quantity_after, 6.0);
    db.close().await;
}
