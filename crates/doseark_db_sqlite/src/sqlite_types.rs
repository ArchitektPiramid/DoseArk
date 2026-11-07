//! Konwersje typów UUID dla sterownika SQLite

use sqlx::types::Text;
use uuid::Uuid;

/// UUID persisted as canonical text in SQLite (`xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`).
///
/// SQLx maps `Uuid` directly to SQLite's binary representation. DoseArk stores UUIDs
/// in TEXT columns, so repository row types use the `Text<T>` adapter explicitly.
pub(crate) type SqliteUuid = Text<Uuid>;

pub(crate) fn into_uuid(value: SqliteUuid) -> Uuid {
    value.0
}

pub(crate) fn into_optional_uuid(value: Option<SqliteUuid>) -> Option<Uuid> {
    value.map(|value| value.0)
}
