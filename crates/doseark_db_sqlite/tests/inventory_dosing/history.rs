use doseark_domain::DoseLogListInput;
use uuid::Uuid;

use super::fixture::Fixture;

/// History keeps pagination totals and the projected before/after stock balances
#[tokio::test]
async fn lists_active_doses_with_stock_projection() {
    let db = Fixture::new().await;
    let first = db.repos.dose_logs.create_dose_log(db.dose(2.0, 1.0, true)).await.unwrap();

    let page = db.repos.dose_logs.list_dose_logs(DoseLogListInput {
        limit: 20, offset: 0, product_id: Some(db.product_id),
    }).await.unwrap();
    assert_eq!(page.total, 1);
    assert_eq!(page.items[0].id, first.id);
    assert_eq!(page.items[0].inventory_quantity_before, Some(9.0));
    assert_eq!(page.items[0].inventory_quantity_after, Some(7.0));

    let empty = db.repos.dose_logs.list_dose_logs(DoseLogListInput {
        limit: 20, offset: 0, product_id: Some(Uuid::new_v4()),
    }).await.unwrap();
    assert_eq!(empty.total, 0);
    db.close().await;
}
