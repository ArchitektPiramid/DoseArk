# Current architecture

DoseArk currently consists of two independently started processes running on one developer's computer.

```text
Browser - React / TypeScript / Vite  (apps/web)
    |= Feature pages, forms, charts and PL/EN translations
    |= packages/types - shared frontend types
    |= packages/client - reusable HTTP transport
                  │ HTTP: http://localhost:8080/api/*
                  ▼
Rust Axum API (apps/api)
    |= routes/- request handlers for catalog, inventory, doses, reports, admin
    |= config.rs - local environment configuration
    |= state.rs / storage_runtime.rs - composition and startup
                  │
                  ▼
crates/doseark_application/ - use cases and repository ports
                  ↕
crates/doseark_domain/ - entities and domain rules
                  |
                  |= crates/doseark_db_sqlite/ - SQLite SQL and migrations
                  |= crates/doseark_db_postgres/ - PostgreSQL SQL and migrations

crates/doseark_contracts/ - Rust HTTP request/response DTOs
crates/doseark_runtime_config/ - optional profile validation and credentials references
```

## Where changes belong

- **Web UI:** page layout, user interactions, input hints, data visualization.
- **TypeScript client/types:** transport behavior and frontend contracts.
- **API:** HTTP routing, request extraction, status and serialization.
- **Application/domain:** dose and inventory rules that must hold independently of the UI.
- **Database adapters:** transactional persistence, queries, migrations and reporting.

## Local configuration and scope

Actual database selection comes from `DOSEARK_STORAGE`, `DOSEARK_SQLITE_PATH` or `DATABASE_URL` in the server's `.env`, **not** from the browser profile picker. Frontend `VITE_API_BASE_URL` points to the running API. `config/profiles.json`  contains display/validation metadata; Vite serves its contents to the browser, so it must **not** contain secrets.

