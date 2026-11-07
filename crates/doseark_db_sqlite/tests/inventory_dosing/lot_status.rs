//! Testy pobierania dawek z aktywnych i wyłączonych partii

use doseark_application::ApplicationError;
use doseark_domain::InventoryLotStatus;

use super::fixture::Fixture;

/// Disabled lots must not be consumed, even with stock remaining
#[tokio::test]
async fn only_active_lots_allow_new_doses() {
    for status in [
        InventoryLotStatus::Reserve,
        InventoryLotStatus::OnHold,
        InventoryLotStatus::Expired,
        InventoryLotStatus::Discarded,
        InventoryLotStatus::Archived,
        InventoryLotStatus::Empty,
    ] {
        let db = Fixture::new().await;
        db.update_lot(db.product_id, status)
            .await
            .expect("status should update");
        let result = db
            .repos
            .dose_logs
            .create_dose_log(db.dose(1.0, 0.5, true))
            .await;
        assert!(matches!(result, Err(ApplicationError::Validation(_))));
        assert_eq!(db.remaining().await, 9.0);
        db.close().await;
    }
}

/// Historical cancellations can return stock to a currently held lot
#[tokio::test]
async fn cancelled_dose_returns_stock_to_held_lot() {
    let db = Fixture::new().await;
    let dose = db
        .repos
        .dose_logs
        .create_dose_log(db.dose(2.0, 1.0, true))
        .await
        .expect("dose should be created");
    db.update_lot(db.product_id, InventoryLotStatus::OnHold)
        .await
        .unwrap();

    db.repos.dose_logs.cancel_dose_log(dose.id).await.unwrap();
    assert_eq!(db.remaining().await, 9.0);
    let lot = db
        .repos
        .inventory_lots
        .list_inventory_lots()
        .await
        .unwrap()
        .into_iter()
        .find(|lot| lot.id == db.lot_id)
        .unwrap();
    assert!(matches!(lot.status, InventoryLotStatus::OnHold));
    db.close().await;
}
