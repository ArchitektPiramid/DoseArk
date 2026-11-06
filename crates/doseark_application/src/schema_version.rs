//! Expected database schema version for the API and storage adapters.
//! Update only when introducing new SQLx migrations; never edit applied migrations.

pub const DATABASE_SCHEMA_VERSION: &str = "0.1.0";
pub const EXPECTED_MIGRATION_REVISION: i64 = 5;
