use chrono::{DateTime, NaiveDate, Utc};
use doseark_domain::{
    CreateBrandInput, CreateDoseLogInput, CreateInventoryLotInput, CreateNutrientInput,
    CreateProductInput, DailyIntakeReportInput, DosageForm, InventoryLotStatus, NutrientKind,
    NutrientTrendReportInput, ProductCategory, SaveProductNutrientInput,
};
use doseark_db_sqlite::{
    create_sqlite_pool, run_migrations, SqliteStorage, SqliteStorageConfig,
};
use uuid::Uuid;

#[tokio::test]
async fn sqlite_reports_match_daily_and_trend_boundaries() {
    let database_path = std::env::temp_dir().join(format!(
        "doseark-sqlite-reports-{}.db",
        Uuid::new_v4()
    ));
    let config = SqliteStorageConfig::new(&database_path);
    let pool = create_sqlite_pool(&config)
        .await
        .expect("SQLite pool should open");

    run_migrations(&pool)
        .await
        .expect("SQLite migrations should apply");

    let storage = SqliteStorage::new(pool.clone(), database_path.display().to_string());
    let repositories = storage.repositories();

    let brand = repositories
        .brands
        .create_brand(CreateBrandInput {
            name: "DoseArk Reports Test".to_string(),
            country: Some("PL".to_string()),
            website: None,
            notes: None,
        })
        .await
        .expect("brand should be created");

    let nutrient = repositories
        .nutrients
        .create_nutrient(CreateNutrientInput {
            canonical_name: "sqlite_reports_magnesium".to_string(),
            display_name: "SQLite Reports Magnesium".to_string(),
            kind: NutrientKind::Mineral,
            default_unit: "mg".to_string(),
            notes: None,
            reference_daily_amount: Some(375.0),
            reference_daily_unit: Some("mg".to_string()),
            reference_source: Some("test".to_string()),
            reference_notes: None,
        })
        .await
        .expect("nutrient should be created");

    let product = repositories
        .products
        .create_product(CreateProductInput {
            brand_id: Some(brand.id),
            name: "DoseArk Reports Product".to_string(),
            category: ProductCategory::Supplement,
            dosage_form: DosageForm::Tablet,
            serving_unit: "tablet".to_string(),
            serving_size_units: 1.0,
            unit_weight_mg: Some(500.0),
            barcode: None,
            description: None,
            ui_color: None,
            ui_icon: None,
            user_importance: None,
        })
        .await
        .expect("product should be created");

    repositories
        .product_nutrients
        .replace_product_nutrients(
            product.id,
            vec![SaveProductNutrientInput {
                nutrient_id: nutrient.id,
                amount_per_serving: 100.0,
                unit: "mg".to_string(),
                label_daily_value_percent: None,
                label_reference_amount: None,
                label_reference_unit: None,
                source: "manual".to_string(),
                notes: None,
            }],
        )
        .await
        .expect("composition should be saved");

    let lot = repositories
        .inventory_lots
        .create_inventory_lot(CreateInventoryLotInput {
            product_id: product.id,
            lot_number: Some("REPORT-LOT".to_string()),
            storage_location: None,
            storage_location_id: None,
            units_initial: 20.0,
            units_remaining: None,
            purchase_date: None,
            opened_date: None,
            expires_at: None,
            price_amount: None,
            price_currency: None,
            status: Some(InventoryLotStatus::Active),
            notes: None,
        })
        .await
        .expect("inventory lot should be created");

    repositories
        .dose_logs
        .create_dose_log(CreateDoseLogInput {
            product_id: product.id,
            inventory_lot_id: Some(lot.id),
            taken_at: utc("2026-08-10T23:30:00Z"),
            units_taken: 1.0,
            unit: "tablet".to_string(),
            serving_multiplier: 1.0,
            reason: None,
            notes: Some("first report dose".to_string()),
        })
        .await
        .expect("first dose should be created");

    repositories
        .dose_logs
        .create_dose_log(CreateDoseLogInput {
            product_id: product.id,
            inventory_lot_id: Some(lot.id),
            taken_at: utc("2026-08-12T10:00:00Z"),
            units_taken: 2.0,
            unit: "tablet".to_string(),
            serving_multiplier: 2.0,
            reason: None,
            notes: Some("second report dose".to_string()),
        })
        .await
        .expect("second dose should be created");

    let daily = repositories
        .reports
        .get_daily_intake_report(DailyIntakeReportInput {
            date: date("2026-08-11"),
            start_at: utc("2026-08-10T22:00:00Z"),
            end_at: utc("2026-08-11T22:00:00Z"),
        })
        .await
        .expect("daily report should load");

    assert_eq!(daily.rows.len(), 1);
    assert_eq!(daily.rows[0].nutrient_id, nutrient.id);
    assert_eq!(daily.rows[0].amount_total, 100.0);
    assert_eq!(daily.rows[0].dose_count, 1);
    assert_eq!(daily.dose_logs.len(), 1);
    assert_eq!(daily.dose_logs[0].taken_at, utc("2026-08-10T23:30:00Z"));

    let trend = repositories
        .reports
        .get_nutrient_trend_report(NutrientTrendReportInput {
            start_date: date("2026-08-11"),
            end_date: date("2026-08-13"),
            start_at: utc("2026-08-10T22:00:00Z"),
            end_at: utc("2026-08-13T22:00:00Z"),
            timezone_offset_minutes: -120,
            day_start_hour: 0,
            nutrient_id: Some(nutrient.id),
            product_id: None,
            unit: Some("mg".to_string()),
        })
        .await
        .expect("trend report should load");

    assert_eq!(trend.nutrients.len(), 1);
    assert_eq!(trend.nutrients[0].amount_total, 300.0);
    assert_eq!(trend.nutrients[0].dose_count, 2);
    assert_eq!(trend.nutrients[0].recorded_day_count, 2);

    assert_eq!(trend.days.len(), 3);
    assert_eq!(trend.days[0].date, date("2026-08-11"));
    assert_eq!(trend.days[0].amount_total, 100.0);
    assert_eq!(trend.days[1].date, date("2026-08-12"));
    assert_eq!(trend.days[1].amount_total, 200.0);
    assert_eq!(trend.days[2].date, date("2026-08-13"));
    assert_eq!(trend.days[2].amount_total, 0.0);
    assert_eq!(trend.events.len(), 2);
    assert_eq!(trend.events[0].amount, 100.0);
    assert_eq!(trend.events[0].unit, "mg");
    assert_eq!(trend.events[1].amount, 200.0);

    let product_trend = repositories
        .reports
        .get_nutrient_trend_report(NutrientTrendReportInput {
            start_date: date("2026-08-11"),
            end_date: date("2026-08-13"),
            start_at: utc("2026-08-10T22:00:00Z"),
            end_at: utc("2026-08-13T22:00:00Z"),
            timezone_offset_minutes: -120,
            day_start_hour: 0,
            nutrient_id: None,
            product_id: Some(product.id),
            unit: Some("tablet".to_string()),
        })
        .await
        .expect("product trend report should load");

    assert_eq!(product_trend.selected_product_id, Some(product.id));
    assert_eq!(product_trend.selected_unit.as_deref(), Some("tablet"));
    assert!(product_trend.nutrients.is_empty());
    assert_eq!(product_trend.days.len(), 3);
    assert_eq!(product_trend.days[0].amount_total, 1.0);
    assert_eq!(product_trend.days[1].amount_total, 2.0);
    assert_eq!(product_trend.days[2].amount_total, 0.0);
    assert_eq!(product_trend.events.len(), 2);
    assert_eq!(product_trend.events[0].amount, 1.0);
    assert_eq!(product_trend.events[1].amount, 2.0);
    assert_eq!(product_trend.events[0].product_id, product.id);
    assert_eq!(product_trend.events[0].product_name, "DoseArk Reports Product");

    pool.close().await;
    remove_sqlite_files(&database_path);
}

fn utc(value: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(value)
        .expect("test timestamp should be valid")
        .with_timezone(&Utc)
}

fn date(value: &str) -> NaiveDate {
    NaiveDate::parse_from_str(value, "%Y-%m-%d").expect("test date should be valid")
}

fn remove_sqlite_files(database_path: &std::path::Path) {
    let _ = std::fs::remove_file(database_path);
    let _ = std::fs::remove_file(database_path.with_extension("db-shm"));
    let _ = std::fs::remove_file(database_path.with_extension("db-wal"));
}
