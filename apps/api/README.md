# doseark-api

Axum HTTP API for DoseArk. Routes delegate business rules to `doseark_application`.
Runtime settings and profile connectivity checks remain in the API host.

## Source layout

- `src/routes/catalog/` — brands, products and nutrients
- `src/routes/dosing/` — dose logs and nutrient snapshots
- `src/routes/inventory/` — inventory lots, movements and storage locations
- `src/routes/reports/` — daily and trend reports, query parsing
- `src/routes/system/` — health and administration endpoints
- `src/routes/error.rs` — shared HTTP error format

## Checks

Run from the repository root:

```powershell
cargo fmt --all -- --check
cargo check -p doseark-api
cargo test -p doseark-api
cargo test -p doseark_contracts
cargo check --workspace --locked
```
