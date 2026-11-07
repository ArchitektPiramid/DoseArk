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
