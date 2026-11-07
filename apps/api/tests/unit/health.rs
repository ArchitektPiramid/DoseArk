//! Unit tests for system health response helpers.

use super::*;

#[test]
fn storage_mode_names_match_existing_clients() {
    assert_eq!(storage_mode(StorageEngine::Postgres), "remote-postgresql");
    assert_eq!(storage_mode(StorageEngine::Sqlite), "remote-sqlite");
}

#[test]
fn compiled_component_versions_are_present() {
    for version in [
        version::BACKEND_VERSION,
        version::DOMAIN_VERSION,
        version::APPLICATION_VERSION,
        version::CONTRACTS_VERSION,
        version::DB_POSTGRES_VERSION,
        version::DB_SQLITE_VERSION,
    ] {
        assert!(!version.is_empty());
    }
}
