//! Isolated PostgreSQL inventory and dose-history regression tests.

#[path = "postgres_integrity/mod.rs"]
mod integrity;

#[tokio::test]
#[ignore = "requires DOSEARK_PG_TEST_URL with permission to create temporary schemas"]
async fn inventory_and_snapshot_regressions() {
    integrity::run_inventory_and_snapshot_regressions().await.unwrap();
}
