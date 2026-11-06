use doseark_application::{ApplicationError, ApplicationResult};
use sqlx::{Sqlite, SqlitePool, Transaction};

/// Starts a SQLite write transaction that acquires the writer reservation up front.
///
/// Inventory and dosing flows read current stock before applying multiple dependent
/// writes, so using `BEGIN IMMEDIATE` prevents another writer from changing that
/// stock between the read and the final mutation.
pub(crate) async fn begin_immediate(
    pool: &SqlitePool,
) -> ApplicationResult<Transaction<'static, Sqlite>> {
    pool.begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|err| ApplicationError::Repository(err.to_string()))
}
