//! Testy niezmienności produktu przypisanego do partii

use doseark_application::ApplicationError;
use doseark_domain::InventoryLotStatus;

use super::fixture::Fixture;

/// A lot with existing dose and stock history must retain its product ID
#[tokio::test]
async fn existing_lot_cannot_be_reassigned_to_another_product() {
    let db = Fixture::new().await;
    db.repos
        .dose_logs
        .create_dose_log(db.dose(2.0, 1.0, true))
        .await
        .expect("initial dose should be recorded");
    let other_id = db.another_product().await;

    let result = db.update_lot(other_id, InventoryLotStatus::Active).await;
    assert!(matches!(result, Err(ApplicationError::Validation(_))));

    let lot = db
        .repos
        .inventory_lots
        .list_inventory_lots()
        .await
        .unwrap()
        .into_iter()
        .find(|lot| lot.id == db.lot_id)
        .unwrap();
    assert_eq!(lot.product_id, db.product_id);
    assert_eq!(db.remaining().await, 7.0);
    db.close().await;
}

#[tokio::test]
async fn pristine_lot_also_keeps_its_original_product() {
    let db = Fixture::new().await;
    let other_id = db.another_product().await;
    let result = db.update_lot(other_id, InventoryLotStatus::Active).await;
    assert!(matches!(result, Err(ApplicationError::Validation(_))));
    db.close().await;
}

/// Changing a dose to another product cannot reuse the original lot
#[tokio::test]
async fn dose_cannot_switch_product_while_reusing_another_products_lot() {
    let db = Fixture::new().await;
    let dose = db
        .repos
        .dose_logs
        .create_dose_log(db.dose(2.0, 1.0, true))
        .await
        .unwrap();
    let other = db.another_product().await;
    let mut edit = db.edit(&dose, 2.0, 1.0);
    edit.product_id = other;
    let result = db.repos.dose_logs.update_dose_log(dose.id, edit).await;
    assert!(matches!(result, Err(ApplicationError::Validation(_))));
    assert_eq!(db.remaining().await, 7.0);
    db.close().await;
}
