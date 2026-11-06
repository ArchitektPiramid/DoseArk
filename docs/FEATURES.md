# Current functionality


| Area       | Code present                                                             | Main paths                                                                                        |
| ---------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------- |
| Dashboard  | Overview and intake/inventory summaries                                  | `apps/web/src/features/dashboard/`                                                                |
| Products   | Medicine/supplement catalog, composition and associations                | `apps/web/src/features/products/`, `apps/api/src/routes/products.rs`, `product_nutrients.rs`      |
| Catalog    | Brands and nutrients                                                     | `apps/web/src/features/catalog/`, `apps/api/src/routes/brands.rs`, `nutrients.rs`                 |
| Inventory  | Lots, stock levels and movements                                         | `apps/web/src/features/inventory/`, `apps/api/src/routes/inventory_lots.rs`, `stock_movements.rs` |
| Storage    | Storage locations                                                        | `apps/web/src/features/storage-locations/`, `apps/api/src/routes/storage_locations.rs`            |
| Dosing     | Single/bulk intake, history, cancellations, nutrient snapshots           | `apps/web/src/features/dosing/`, `apps/api/src/routes/dose_logs.rs`, `dose_log_nutrients.rs`      |
| Statistics | Intake reports, charts and trend analyses                                | `apps/web/src/features/statistics/`, `apps/api/src/routes/reports.rs`                             |
| Settings   | Local API source, runtime diagnostics, logs and supported backup actions | `apps/web/src/features/settings/`, `apps/api/src/routes/profile_admin.rs`, `backups.rs`           |
| Languages  | PL and EN translations                                                   | `apps/web/src/core/i18n/locales/`                                                                 |
| Storage    | SQLite and PostgreSQL implementations and migrations                     | `crates/doseark_db_sqlite/`, `crates/doseark_db_postgres/`                                        |

## Currently supported setup

Web browser + separate local Axum backend, typically with a local SQLite file. PostgreSQL is also implemented, but it requires a separately managed database server. The frontend communicates with the API over local HTTP.

## Not currently delivered

- docker / docker compose deployment
- mobile apps
- offline synchronization and multi-user account system
- internet-facing authenticated service



