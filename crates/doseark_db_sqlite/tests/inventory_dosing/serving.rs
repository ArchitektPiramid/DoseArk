use doseark_application::ApplicationError;

use super::fixture::Fixture;

/// Rejects a fabricated serving multiplier without touching the lot
#[tokio::test]
async fn conflicting_multiplier_does_not_create_a_dose() {
    let db = Fixture::new().await;
    let result = db.repos.dose_logs.create_dose_log(db.dose(1.0, 100.0, true)).await;
    assert!(matches!(result, Err(ApplicationError::Validation(_))));
    assert_eq!(db.remaining().await, 9.0);
    db.close().await;
}

#[tokio::test]
async fn invalid_edit_keeps_original_nutrients() {
    let db = Fixture::new().await;
    let dose = db.repos.dose_logs.create_dose_log(db.dose(2.0, 1.0, true)).await.unwrap();
    let error = db.repos.dose_logs.update_dose_log(dose.id, db.edit(&dose, 3.0, 100.0)).await;
    assert!(matches!(error, Err(ApplicationError::Validation(_))));
    assert_eq!(db.remaining().await, 7.0);
    let nutrients = db.repos.dose_log_nutrients.list_dose_log_nutrients(dose.id).await.unwrap();
    assert_eq!(nutrients[0].amount, 100.0);
    db.close().await;
}

#[tokio::test]
async fn half_serving_without_lot_uses_product_definition() {
    let db = Fixture::new().await;
    let dose = db.repos.dose_logs.create_dose_log(db.dose(1.0, 0.5, false)).await.unwrap();
    assert_eq!(dose.serving_multiplier, 0.5);
    let nutrients = db.repos.dose_log_nutrients.list_dose_log_nutrients(dose.id).await.unwrap();
    assert_eq!(nutrients[0].amount, 50.0);
    assert_eq!(db.remaining().await, 9.0);
    db.close().await;
}

#[tokio::test]
async fn wrong_product_unit_is_rejected() {
    let db = Fixture::new().await;
    let mut input = db.dose(1.0, 0.5, false);
    input.unit = "capsule".into();
    let result = db.repos.dose_logs.create_dose_log(input).await;
    assert!(matches!(result, Err(ApplicationError::Validation(_))));
    db.close().await;
}
