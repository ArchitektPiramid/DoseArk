//! Testy granic dnia, gdy raport zaczyna się po północy

use chrono::{DateTime, NaiveDate, Utc};
use doseark_db_sqlite::{create_sqlite_pool, run_migrations, SqliteStorage, SqliteStorageConfig};
use doseark_domain::NutrientTrendReportInput;
use uuid::Uuid;

fn utc(value: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(value).unwrap().with_timezone(&Utc)
}

fn date(value: &str) -> NaiveDate {
    NaiveDate::parse_from_str(value, "%Y-%m-%d").unwrap()
}

/// Every report section must use the same half-open UTC interval
#[tokio::test]
async fn shifted_day_boundary_matches_totals_days_and_events() {
    let path = std::env::temp_dir().join(format!("doseark-report-boundary-{}.db", Uuid::new_v4()));
    let pool = create_sqlite_pool(&SqliteStorageConfig::new(&path)).await.unwrap();
    run_migrations(&pool).await.unwrap();

    let product_id = Uuid::new_v4();
    let nutrient_id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query("INSERT INTO products (id, name, category, dosage_form, serving_unit, serving_size_units, created_at, updated_at) VALUES (?1, 'Boundary product', 'supplement', 'tablet', 'tablet', 1, ?2, ?3)")
        .bind(product_id.to_string()).bind(now).bind(now).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO nutrients (id, canonical_name, display_name, kind, default_unit, created_at, updated_at) VALUES (?1, 'boundary_nutrient', 'Boundary nutrient', 'mineral', 'mg', ?2, ?3)")
        .bind(nutrient_id.to_string()).bind(now).bind(now).execute(&pool).await.unwrap();

    // Reporting days begin at 04:00 (UTC+02:00); the first and last events fall outside
    for taken_at in [
        "2026-08-11T00:30:00Z", "2026-08-11T02:30:00Z",
        "2026-08-12T01:00:00Z", "2026-08-13T01:30:00Z",
        "2026-08-13T02:30:00Z",
    ] {
        let dose_id = Uuid::new_v4();
        sqlx::query("INSERT INTO dose_logs (id, product_id, taken_at, units_taken, unit, serving_multiplier, status, created_at, updated_at) VALUES (?1, ?2, ?3, 1, 'tablet', 1, 'active', ?4, ?5)")
            .bind(dose_id.to_string()).bind(product_id.to_string())
            .bind(utc(taken_at)).bind(now).bind(now).execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO dose_log_nutrients (id, dose_log_id, nutrient_id, amount, unit, created_at) VALUES (?1, ?2, ?3, 100, 'mg', ?4)")
            .bind(Uuid::new_v4().to_string()).bind(dose_id.to_string())
            .bind(nutrient_id.to_string()).bind(now).execute(&pool).await.unwrap();
    }

    let storage = SqliteStorage::new(pool.clone(), path.display().to_string());
    let report = storage.repositories().reports;
    let request = |product_id_filter, nutrient_id_filter, unit: &str| NutrientTrendReportInput {
        start_date: date("2026-08-11"),
        end_date: date("2026-08-12"),
        start_at: utc("2026-08-10T22:00:00Z"),
        end_at: utc("2026-08-12T22:00:00Z"),
        timezone_offset_minutes: -120,
        day_start_hour: 4,
        nutrient_id: nutrient_id_filter,
        product_id: product_id_filter,
        unit: Some(unit.into()),
    };

    let nutrients = report.get_nutrient_trend_report(request(None, Some(nutrient_id), "mg"))
        .await.unwrap();
    assert_eq!(nutrients.nutrients.len(), 1);
    assert_eq!(nutrients.nutrients[0].amount_total, 300.0);
    assert_eq!(nutrients.nutrients[0].recorded_day_count, 2);
    assert_eq!(nutrients.days.len(), 2);
    assert_eq!(nutrients.days[0].amount_total, 200.0);
    assert_eq!(nutrients.days[1].amount_total, 100.0);
    assert_eq!(nutrients.events.len(), 3);

    let products = report.get_nutrient_trend_report(request(Some(product_id), None, "tablet"))
        .await.unwrap();
    assert_eq!(products.days[0].amount_total, 2.0);
    assert_eq!(products.days[1].amount_total, 1.0);
    assert_eq!(products.events.len(), 3);

    pool.close().await;
    for file in [path.clone(), path.with_extension("db-shm"), path.with_extension("db-wal")] {
        let _ = std::fs::remove_file(file);
    }
}
