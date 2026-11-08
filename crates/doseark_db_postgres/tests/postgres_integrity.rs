//! PostgreSQL inventory and report integration regression tests.
//!
//! Tests are ignored by default and use temporary schemas in the explicitly
//! configured test database when `DOSEARK_PG_TEST_URL` is provided.

#[path = "postgres_integrity/mod.rs"]
mod test_support;

#[tokio::test]
#[ignore = "requires DOSEARK_PG_TEST_URL and permission to create temporary schemas"]
async fn inventory_and_snapshot_regressions() {
    test_support::run_inventory_regressions()
        .await
        .expect("inventory and snapshot regressions failed");
}

#[tokio::test]
#[ignore = "requires DOSEARK_PG_TEST_URL and permission to create temporary schemas"]
async fn report_range_regressions() {
    test_support::run_report_regressions()
        .await
        .expect("report range regressions failed");
}

#[tokio::test]
#[ignore = "requires DOSEARK_PG_TEST_URL and permission to create temporary schemas"]
async fn product_archive_regressions() {
    use doseark_application::ProductRepository;
    use doseark_db_postgres::PgProductRepository;
    use doseark_domain::{CreateProductInput, DosageForm, ProductCategory, UpdateProductInput};

    let db = test_support::TestDatabase::new()
        .await
        .expect("isolated test DB");
    let products = PgProductRepository::new(db.pool.clone());
    let create_input = |archived: Option<bool>| CreateProductInput {
        brand_id: None,
        name: "Archive test".into(),
        category: ProductCategory::Supplement,
        dosage_form: DosageForm::Tablet,
        serving_unit: "tablet".into(),
        serving_size_units: 1.0,
        unit_weight_mg: None,
        barcode: None,
        description: None,
        ui_color: None,
        ui_icon: None,
        user_importance: None,
        archived,
    };
    let active = products.create_product(create_input(None)).await.unwrap();
    let archived_on_create = products
        .create_product(create_input(Some(true)))
        .await
        .unwrap();
    assert!(!active.archived);
    assert!(archived_on_create.archived);

    let update_input = |archived: Option<bool>| UpdateProductInput {
        brand_id: None,
        name: "Archive test".into(),
        category: ProductCategory::Supplement,
        dosage_form: DosageForm::Tablet,
        serving_unit: "tablet".into(),
        serving_size_units: 1.0,
        unit_weight_mg: None,
        barcode: None,
        description: None,
        ui_color: None,
        ui_icon: None,
        user_importance: None,
        archived,
    };
    let archived = products
        .update_product(active.id, update_input(Some(true)))
        .await
        .unwrap();
    assert!(archived.archived);
    let preserved = products
        .update_product(active.id, update_input(None))
        .await
        .unwrap();
    assert!(
        preserved.archived,
        "Omitted archive state must preserve existing status"
    );
    let restored = products
        .update_product(active.id, update_input(Some(false)))
        .await
        .unwrap();
    assert!(!restored.archived);
    let persisted: bool = sqlx::query_scalar("SELECT archived FROM products WHERE id = $1")
        .bind(active.id)
        .fetch_one(&db.pool)
        .await
        .unwrap();
    assert!(!persisted);
    db.cleanup().await.unwrap();
}
