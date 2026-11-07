//! Sprawdza wstrzykiwanie repozytoriów oraz walidację żądań przed zapisem

#[path = "service_wiring/probe.rs"]
mod probe;

use std::sync::Arc;

use chrono::{Duration, Utc};
use doseark_application::{AppServices, ApplicationError};
use doseark_domain::{
    CreateBrandInput, CreateDoseLogInput, CreateInventoryLotInput, DailyIntakeReportInput,
    DoseLogListInput, InventoryLotStatus, SaveProductNutrientInput,
};
use uuid::Uuid;

use probe::RepositoryProbe;

/// Exercises the service graph without starting a database
#[tokio::test]
async fn forwards_reads_to_the_matching_repositories() {
    let repositories = Arc::new(RepositoryProbe::default());
    let services = AppServices::from_repositories(repositories.bundle());

    services.brands.list_brands().await.unwrap();
    services.products.list_products().await.unwrap();
    services.nutrients.list_nutrients().await.unwrap();
    services
        .product_composition
        .list_product_nutrients(Uuid::new_v4())
        .await
        .unwrap();
    services.inventory_lots.list_inventory_lots().await.unwrap();
    services
        .stock_movements
        .list_stock_movements()
        .await
        .unwrap();
    services
        .dose_log_nutrients
        .list_dose_log_nutrients(Uuid::new_v4())
        .await
        .unwrap();
    services
        .storage_locations
        .list_storage_locations(true)
        .await
        .unwrap();

    let product_id = Uuid::new_v4();
    let page = services
        .dose_logs
        .list_dose_logs(DoseLogListInput {
            limit: 500,
            offset: -4,
            product_id: Some(product_id),
        })
        .await
        .unwrap();
    assert_eq!((page.limit, page.offset), (100, 0));
    assert_eq!(repositories.last_page(), Some((100, 0, Some(product_id))));

    let start_at = Utc::now();
    let date = start_at.date_naive();
    let end_at = start_at.clone() + Duration::days(1);
    let report = services
        .reports
        .get_daily_intake_report(DailyIntakeReportInput {
            date,
            start_at,
            end_at,
        })
        .await
        .unwrap();
    assert_eq!(report.date, date);

    assert_eq!(
        repositories.calls(),
        vec![
            "brands",
            "products",
            "nutrients",
            "composition",
            "lots",
            "movements",
            "dose_nutrients",
            "locations",
            "doses",
            "reports",
        ]
    );
}

/// Invalid commands must never reach persistence adapters
#[tokio::test]
async fn rejects_invalid_inputs_before_repository_calls() {
    let repositories = Arc::new(RepositoryProbe::default());
    let services = AppServices::from_repositories(repositories.bundle());

    let dose = CreateDoseLogInput {
        product_id: Uuid::new_v4(),
        inventory_lot_id: None,
        taken_at: Utc::now(),
        units_taken: 0.0,
        unit: "tablet".into(),
        serving_multiplier: 1.0,
        reason: None,
        notes: None,
    };
    assert!(matches!(
        services.dose_logs.create_dose_log(dose).await,
        Err(ApplicationError::Validation(_))
    ));

    let lot = CreateInventoryLotInput {
        product_id: Uuid::new_v4(),
        lot_number: None,
        storage_location: None,
        storage_location_id: None,
        units_initial: 10.0,
        units_remaining: Some(11.0),
        purchase_date: None,
        opened_date: None,
        expires_at: None,
        price_amount: None,
        price_currency: None,
        status: Some(InventoryLotStatus::Active),
        notes: None,
    };
    assert!(matches!(
        services.inventory_lots.create_inventory_lot(lot).await,
        Err(ApplicationError::Validation(_))
    ));

    let nutrient_id = Uuid::new_v4();
    let ingredient = SaveProductNutrientInput {
        nutrient_id,
        amount_per_serving: 100.0,
        unit: "mg".into(),
        label_daily_value_percent: None,
        label_reference_amount: None,
        label_reference_unit: None,
        source: "manual".into(),
        notes: None,
    };
    assert!(matches!(
        services
            .product_composition
            .replace_product_nutrients(Uuid::new_v4(), vec![ingredient.clone(), ingredient])
            .await,
        Err(ApplicationError::Validation(_))
    ));

    let brand = CreateBrandInput {
        name: "   ".into(),
        country: None,
        website: None,
        notes: None,
    };
    assert!(matches!(
        services.brands.create_brand(brand).await,
        Err(ApplicationError::Validation(_))
    ));

    let now = Utc::now();
    let invalid_end = now.clone();
    assert!(matches!(
        services
            .reports
            .get_daily_intake_report(DailyIntakeReportInput {
                date: now.date_naive(),
                start_at: now,
                end_at: invalid_end,
            })
            .await,
        Err(ApplicationError::Validation(_))
    ));

    assert!(repositories.calls().is_empty());
}

#[tokio::test]
async fn clamps_small_pages_without_changing_the_filter() {
    let repositories = Arc::new(RepositoryProbe::default());
    let services = AppServices::from_repositories(repositories.bundle());
    let product_id = Uuid::new_v4();

    let page = services
        .dose_logs
        .list_dose_logs(DoseLogListInput {
            limit: 0,
            offset: -20,
            product_id: Some(product_id),
        })
        .await
        .unwrap();

    assert_eq!((page.limit, page.offset), (1, 0));
    assert_eq!(repositories.last_page(), Some((1, 0, Some(product_id))));
}
