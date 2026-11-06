//! PostgreSQL persistence adapter for DoseArk.
//!
//! Implements application repository ports, migrations, connection helpers,
//! and PostgreSQL-specific storage diagnostics.

pub mod diagnostics;
pub mod postgres;
pub mod repositories;
pub mod storage;

pub use diagnostics::*;
pub use postgres::*;
pub use repositories::*;
pub use storage::*;

pub use sqlx::PgPool;
