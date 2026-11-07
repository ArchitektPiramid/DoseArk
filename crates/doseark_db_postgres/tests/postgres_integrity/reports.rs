//! Confirms that trend events and daily totals use identical UTC boundaries.

use chrono::{TimeZone, Utc};
use doseark_application::{DailyIntakeReportRepository, DoseLogRepository};
use doseark_db_postgres::{PgDailyIntakeReportRepository, PgDoseLogRepository};
use doseark_domain::{CreateDoseLogInput, NutrientTrendReportInput};

use super::{seed_product, TestDatabase, TestResult};

pub(super) async fn run() -> TestResult<()> {
    let db = TestDatabase::new().await?;
    let product_id = seed_product(&db.pool, "Boundary Product").await?;
    let dosing = PgDoseLogRepository::new(db.pool.clone());
    let start = Utc.with_ymd_and_hms(2026, 11, 7, 4, 0, 0).single().unwrap();
    let end = Utc.with_ymd_and_hms(2026, 11, 8, 4, 0, 0).single().unwrap();

    for timestamp in [
        Utc.with_ymd_and_hms(2026, 11, 7, 3, 59, 59).single().unwrap(),
        Utc.with_ymd_and_hms(2026, 11, 7, 4, 0, 0).single().unwrap(),
        Utc.with_ymd_and_hms(2026, 11, 8, 3, 59, 59).single().unwrap(),
        Utc.with_ymd_and_hms(2026, 11, 8, 4, 0, 0).single().unwrap(),
    ] {
        dosing.create_dose_log(CreateDoseLogInput {
            product_id,
            inventory_lot_id: None,
            taken_at: timestamp,
            units_taken: 1.0,
            unit: "tablet".into(),
            serving_multiplier: 1.0,
            reason: None,
            notes: None,
        }).await?;
    }

    let report = PgDailyIntakeReportRepository::new(db.pool.clone())
        .get_nutrient_trend_report(NutrientTrendReportInput {
            start_date: start.date_naive(),
            end_date: start.date_naive(),
            start_at: start,
            end_at: end,
            timezone_offset_minutes: 0,
            day_start_hour: 4,
            nutrient_id: None,
            product_id: Some(product_id),
            unit: Some("tablet".into()),
        }).await?;

    assert_eq!(report.days.len(), 1);
    assert_eq!(report.days[0].amount_total, 2.0);
    assert_eq!(report.events.len(), 2);
    assert!(report.events.iter().all(|e| e.taken_at >= start && e.taken_at < end));
    assert_eq!(report.events.iter().map(|e| e.amount).sum::<f64>(), 2.0);

    db.cleanup().await
}
