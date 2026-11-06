# DoseArk Rust crates

The workspace keeps domain logic, application use cases, transport contracts, and database adapters separated so the same Rust logic can be reused by the HTTP API and future Tauri local mode.

| Crate | Responsibility |
| --- | --- |
| `doseark_domain` | Pure domain models, enums, value objects, and domain rules. It must not depend on HTTP, Tauri, SQLx, PostgreSQL, or SQLite. |
| `doseark_application` | Application services/use cases and ports implemented by infrastructure adapters. It owns repository contracts, service composition, and storage diagnostics abstractions. |
| `doseark_contracts` | Serialization-friendly request/response contracts shared by transport boundaries such as Axum HTTP and future Tauri IPC. It must not contain persistence logic. |
| `doseark_db_postgres` | PostgreSQL adapter. Implements application repository ports, migrations, connection helpers, and PostgreSQL diagnostics. |
| `doseark_db_sqlite` | SQLite adapter for local/server SQLite modes. Provides migrations, diagnostics, managed backup support, catalog, inventory, stock movement, dosing, and report repositories. |
| `doseark_runtime_config` | File-backed profile metadata and Rust-only credential resolution shared by the HTTP API and future Tauri runtimes. It never exposes secret values to browser JavaScript. |


## Versioning

Cargo crate versions are internal package metadata. DoseArk exposes only Frontend, Backend and Database versions to users.

## Dependency direction

```text
doseark_domain
      ^
      |
doseark_application
      ^
      |
      +--------------------+
      |                    |
doseark_db_postgres   doseark_db_sqlite
```

Transport adapters live in the applications, not in separate core crates:

```text
apps/api              -> Axum HTTP -> doseark_application
apps/desktop (future) -> Tauri IPC -> doseark_application

doseark_runtime_config -> shared by Rust API / future Tauri host
```

The React frontend will use a transport-neutral client interface so feature code does not need to know whether requests use HTTP or Tauri IPC.

## SQLite transaction boundary

Inventory and dosing mutations use `BEGIN IMMEDIATE` transactions so stock reads, dose writes, stock adjustments, audit movements, and nutrient snapshots commit or roll back as one operation.
