use doseark_domain::{
    CreateBrandInput, CreateNutrientInput, CreateProductInput, CreateStorageLocationInput,
    DosageForm, NutrientKind, ProductCategory, SaveProductNutrientInput,
};
use doseark_db_sqlite::{
    create_sqlite_pool, run_migrations, SqliteStorage, SqliteStorageConfig,
};
use uuid::Uuid;

#[tokio::test]
async fn sqlite_catalog_repositories_round_trip() {
    let database_path = std::env::temp_dir().join(format!(
        "doseark-sqlite-catalog-{}.db",
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
    let repositories = storage.catalog_repositories();

    let brand = repositories
        .brands
        .create_brand(CreateBrandInput {
            name: "DoseArk Test Brand".to_string(),
            country: Some("PL".to_string()),
            website: None,
            notes: None,
        })
        .await
        .expect("brand should be created");

    let stored_brand_id: String = sqlx::query_scalar("SELECT id FROM brands WHERE name = ?1")
        .bind("DoseArk Test Brand")
        .fetch_one(&pool)
        .await
        .expect("brand UUID should be stored as SQLite TEXT");
    assert_eq!(stored_brand_id, brand.id.to_string());
    assert_eq!(stored_brand_id.len(), 36);

    let nutrient = repositories
        .nutrients
        .create_nutrient(CreateNutrientInput {
            canonical_name: "test_magnesium".to_string(),
            display_name: "Test Magnesium".to_string(),
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
            name: "DoseArk Test Product".to_string(),
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

    let composition = repositories
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

    let location = repositories
        .storage_locations
        .create_storage_location(CreateStorageLocationInput {
            name: "Kitchen".to_string(),
            description: None,
            color: Some("#6f91b8".to_string()),
            icon: Some("box".to_string()),
            sort_order: Some(10),
        })
        .await
        .expect("storage location should be created");

    assert_eq!(repositories.brands.list_brands().await.unwrap().len(), 1);
    assert_eq!(repositories.nutrients.list_nutrients().await.unwrap().len(), 1);
    assert_eq!(repositories.products.list_products().await.unwrap().len(), 1);
    assert_eq!(composition.len(), 1);
    assert_eq!(location.name, "Kitchen");

    let runtime = storage
        .diagnostics()
        .runtime_info()
        .await
        .expect("SQLite diagnostics should load");
    assert_eq!(runtime.applied_migration_revision, 5);

    pool.close().await;
    let _ = std::fs::remove_file(&database_path);
    let _ = std::fs::remove_file(database_path.with_extension("db-shm"));
    let _ = std::fs::remove_file(database_path.with_extension("db-wal"));
}
