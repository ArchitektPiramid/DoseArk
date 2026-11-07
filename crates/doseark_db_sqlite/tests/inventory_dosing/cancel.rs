//! Testy anulowania dawki i przywrócenia zapasu

use doseark_application::ApplicationError;
use doseark_domain::{DoseLogListInput, DoseLogStatus};

use super::fixture::Fixture;

/// Cancelling restores stock and removes the dose from active history
#[tokio::test]
async fn cancellation_restores_stock_only_once() {
    let db = Fixture::new().await;
    let dose = db
        .repos
        .dose_logs
        .create_dose_log(db.dose(2.0, 1.0, true))
        .await
        .unwrap();
    assert_eq!(db.remaining().await, 7.0);

    let cancelled = db.repos.dose_logs.cancel_dose_log(dose.id).await.unwrap();
    assert!(matches!(cancelled.status, DoseLogStatus::Cancelled));
    assert_eq!(db.remaining().await, 9.0);

    let repeat = db.repos.dose_logs.cancel_dose_log(dose.id).await;
    assert!(matches!(repeat, Err(ApplicationError::Validation(_))));
    assert_eq!(db.remaining().await, 9.0);

    let page = db
        .repos
        .dose_logs
        .list_dose_logs(DoseLogListInput {
            limit: 20,
            offset: 0,
            product_id: Some(db.product_id),
        })
        .await
        .unwrap();
    assert_eq!(page.total, 0);
    db.close().await;
}
