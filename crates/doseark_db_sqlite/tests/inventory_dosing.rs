use chrono::Utc;
use doseark_application::ApplicationError;
use doseark_domain::{
    CreateBrandInput, CreateDoseLogInput, CreateInventoryLotInput, CreateNutrientInput,
    CreateProductInput, CreateStorageLocationInput, DoseLogListInput, DoseLogStatus, DosageForm,
    InventoryLotStatus, NutrientKind, ProductCategory, SaveProductNutrientInput,
    UpdateDoseLogInput, UpdateInventoryLotInput,
};
use doseark_db_sqlite::{
    create_sqlite_pool, run_migrations, SqliteStorage, SqliteStorageConfig,
};
use uuid::Uuid;

#[tokio::test]
async fn sqlite_inventory_and_dosing_round_trip_is_atomic() {
    let database_path = std::env::temp_dir().join(format!(
        "doseark-sqlite-dosing-{}.db",
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
    let catalog = storage.catalog_repositories();
    let operational = storage.inventory_dosing_repositories();

    let brand = catalog
        .brands
        .create_brand(CreateBrandInput {
            name: "DoseArk Dosing Test".to_string(),
            country: Some("PL".to_string()),
            website: None,
            notes: None,
        })
        .await
        .expect("brand should be created");

    let nutrient = catalog
        .nutrients
        .create_nutrient(CreateNutrientInput {
            canonical_name: "sqlite_test_magnesium".to_string(),
            display_name: "SQLite Test Magnesium".to_string(),
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

    let product = catalog
        .products
        .create_product(CreateProductInput {
            brand_id: Some(brand.id),
            name: "DoseArk Dosing Product".to_string(),
            category: ProductCategory::Supplement,
            dosage_form: DosageForm::Tablet,
            serving_unit: "tablet".to_string(),
            serving_size_units: 2.0,
            unit_weight_mg: Some(500.0),
            barcode: None,
            description: None,
            ui_color: None,
            ui_icon: None,
            user_importance: None,
        })
        .await
        .expect("product should be created");

    catalog
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

    let location = catalog
        .storage_locations
        .create_storage_location(CreateStorageLocationInput {
            name: "Kitchen".to_string(),
            description: None,
            color: None,
            icon: None,
            sort_order: None,
        })
        .await
        .expect("storage location should be created");

    let lot = operational
        .inventory_lots
        .create_inventory_lot(CreateInventoryLotInput {
            product_id: product.id,
            lot_number: Some("LOT-001".to_string()),
            storage_location: None,
            storage_location_id: Some(location.id),
            units_initial: 10.0,
            units_remaining: None,
            purchase_date: None,
            opened_date: None,
            expires_at: None,
            price_amount: Some(20.0),
            price_currency: Some("PLN".to_string()),
            status: Some(InventoryLotStatus::Active),
            notes: None,
        })
        .await
        .expect("inventory lot should be created");

    assert_eq!(lot.storage_location.as_deref(), Some("Kitchen"));
    assert_eq!(lot.units_remaining, 10.0);

    let lot = operational
        .inventory_lots
        .update_inventory_lot(
            lot.id,
            UpdateInventoryLotInput {
                product_id: product.id,
                lot_number: lot.lot_number.clone(),
                storage_location: None,
                storage_location_id: Some(location.id),
                units_initial: 10.0,
                units_remaining: 9.0,
                purchase_date: None,
                opened_date: None,
                expires_at: None,
                price_amount: Some(20.0),
                price_currency: Some("PLN".to_string()),
                status: InventoryLotStatus::Active,
                notes: None,
            },
        )
        .await
        .expect("inventory lot should be updated");

    assert_eq!(lot.units_remaining, 9.0);

    // A conflicting client multiplier must not modify dose history or stock.
    let mismatched_create = operational
        .dose_logs
        .create_dose_log(CreateDoseLogInput {
            product_id: product.id,
            inventory_lot_id: Some(lot.id),
            taken_at: Utc::now(),
            units_taken: 1.0,
            unit: "tablet".to_string(),
            serving_multiplier: 100.0,
            reason: None,
            notes: None,
        })
        .await;
    assert!(matches!(mismatched_create, Err(ApplicationError::Validation(_))));
    assert_eq!(
        operational.inventory_lots.list_inventory_lots().await.unwrap()[0].units_remaining,
        9.0,
    );
    assert_eq!(
        operational.dose_logs.list_dose_logs(DoseLogListInput {
            limit: 20,
            offset: 0,
            product_id: Some(product.id),
        }).await.unwrap().total,
        0,
    );

    let dose = operational
        .dose_logs
        .create_dose_log(CreateDoseLogInput {
            product_id: product.id,
            inventory_lot_id: Some(lot.id),
            taken_at: Utc::now(),
            units_taken: 2.0,
            unit: "tablet".to_string(),
            serving_multiplier: 1.0,
            reason: None,
            notes: Some("initial test dose".to_string()),
        })
        .await
        .expect("dose should be created");

    let lots = operational
        .inventory_lots
        .list_inventory_lots()
        .await
        .expect("lots should load");
    assert_eq!(lots[0].units_remaining, 7.0);

    let snapshots = operational
        .dose_log_nutrients
        .list_dose_log_nutrients(dose.id)
        .await
        .expect("dose nutrient snapshot should load");
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].amount, 100.0);
    assert_eq!(snapshots[0].nutrient_display_name, "SQLite Test Magnesium");

    let page = operational
        .dose_logs
        .list_dose_logs(DoseLogListInput {
            limit: 20,
            offset: 0,
            product_id: Some(product.id),
        })
        .await
        .expect("dose history should load");
    assert_eq!(page.total, 1);
    assert_eq!(page.items[0].inventory_quantity_before, Some(9.0));
    assert_eq!(page.items[0].inventory_quantity_after, Some(7.0));

    // Editing a dose with a fabricated multiplier also leaves prior snapshots intact.
    let mismatched_update = operational
        .dose_logs
        .update_dose_log(
            dose.id,
            UpdateDoseLogInput {
                product_id: product.id,
                inventory_lot_id: Some(lot.id),
                taken_at: dose.taken_at.clone(),
                units_taken: 3.0,
                unit: "tablet".to_string(),
                serving_multiplier: 100.0,
                reason: None,
                notes: None,
            },
        )
        .await;
    assert!(matches!(mismatched_update, Err(ApplicationError::Validation(_))));
    assert_eq!(
        operational.inventory_lots.list_inventory_lots().await.unwrap()[0].units_remaining,
        7.0,
    );
    assert_eq!(
        operational.dose_log_nutrients.list_dose_log_nutrients(dose.id).await.unwrap()[0].amount,
        100.0,
    );

    let failed_update = operational
        .dose_logs
        .update_dose_log(
            dose.id,
            UpdateDoseLogInput {
                product_id: product.id,
                inventory_lot_id: Some(lot.id),
                taken_at: dose.taken_at.clone(),
                units_taken: 20.0,
                unit: "tablet".to_string(),
                serving_multiplier: 10.0,
                reason: None,
                notes: Some("must roll back".to_string()),
            },
        )
        .await;
    assert!(matches!(failed_update, Err(ApplicationError::Validation(_))));

    let lots_after_failed_update = operational
        .inventory_lots
        .list_inventory_lots()
        .await
        .unwrap();
    assert_eq!(lots_after_failed_update[0].units_remaining, 7.0);

    let page_after_failed_update = operational
        .dose_logs
        .list_dose_logs(DoseLogListInput {
            limit: 20,
            offset: 0,
            product_id: Some(product.id),
        })
        .await
        .unwrap();
    assert_eq!(page_after_failed_update.items[0].units_taken, 2.0);

    let updated = operational
        .dose_logs
        .update_dose_log(
            dose.id,
            UpdateDoseLogInput {
                product_id: product.id,
                inventory_lot_id: Some(lot.id),
                taken_at: dose.taken_at.clone(),
                units_taken: 3.0,
                unit: "tablet".to_string(),
                serving_multiplier: 1.5,
                reason: None,
                notes: Some("updated test dose".to_string()),
            },
        )
        .await
        .expect("dose should update atomically");

    assert_eq!(updated.units_taken, 3.0);

    let lots = operational.inventory_lots.list_inventory_lots().await.unwrap();
    assert_eq!(lots[0].units_remaining, 6.0);

    let snapshots = operational
        .dose_log_nutrients
        .list_dose_log_nutrients(dose.id)
        .await
        .unwrap();
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].amount, 150.0);

    let cancelled = operational
        .dose_logs
        .cancel_dose_log(dose.id)
        .await
        .expect("dose should cancel atomically");
    assert!(matches!(cancelled.status, DoseLogStatus::Cancelled));

    let lots = operational.inventory_lots.list_inventory_lots().await.unwrap();
    assert_eq!(lots[0].units_remaining, 9.0);

    let page = operational
        .dose_logs
        .list_dose_logs(DoseLogListInput {
            limit: 20,
            offset: 0,
            product_id: Some(product.id),
        })
        .await
        .unwrap();
    assert_eq!(page.total, 0);

    let movements = operational
        .stock_movements
        .list_stock_movements_for_lot(lot.id)
        .await
        .expect("stock audit should load");
    assert_eq!(movements.len(), 6);
    assert_eq!(movements[0].quantity_after, 9.0);

    let insufficient = operational
        .dose_logs
        .create_dose_log(CreateDoseLogInput {
            product_id: product.id,
            inventory_lot_id: Some(lot.id),
            taken_at: Utc::now(),
            units_taken: 10.0,
            unit: "tablet".to_string(),
            serving_multiplier: 5.0,
            reason: None,
            notes: None,
        })
        .await;
    assert!(matches!(insufficient, Err(ApplicationError::Validation(_))));

    let lots = operational.inventory_lots.list_inventory_lots().await.unwrap();
    assert_eq!(lots[0].units_remaining, 9.0);

    // One tablet from a two-tablet serving creates half a serving's snapshot.
    let half_serving = operational
        .dose_logs
        .create_dose_log(CreateDoseLogInput {
            product_id: product.id,
            inventory_lot_id: None,
            taken_at: Utc::now(),
            units_taken: 1.0,
            unit: "tablet".to_string(),
            serving_multiplier: 0.5,
            reason: None,
            notes: None,
        })
        .await
        .expect("fractional serving should save");
    assert_eq!(half_serving.serving_multiplier, 0.5);
    let half_snapshot = operational
        .dose_log_nutrients
        .list_dose_log_nutrients(half_serving.id)
        .await
        .unwrap();
    assert_eq!(half_snapshot[0].amount, 50.0);
    assert_eq!(
        operational.inventory_lots.list_inventory_lots().await.unwrap()[0].units_remaining,
        9.0,
    );

    // Unit names must also match the product definition.
    let wrong_unit = operational
        .dose_logs
        .create_dose_log(CreateDoseLogInput {
            product_id: product.id,
            inventory_lot_id: None,
            taken_at: Utc::now(),
            units_taken: 1.0,
            unit: "capsule".to_string(),
            serving_multiplier: 0.5,
            reason: None,
            notes: None,
        })
        .await;
    assert!(matches!(wrong_unit, Err(ApplicationError::Validation(_))));

    pool.close().await;
    let _ = std::fs::remove_file(&database_path);
    let _ = std::fs::remove_file(database_path.with_extension("db-shm"));
    let _ = std::fs::remove_file(database_path.with_extension("db-wal"));
}
