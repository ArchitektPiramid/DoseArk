# doseark_domain

Shared Rust entities and input models for DoseArk. Transport DTOs belong to `doseark_contracts`; persistence rows and validation at database boundaries belong to the adapters. This crate keeps existing JSON field names and enum representations stable.

## Layout

- `src/catalog/` — brands, products, nutrients and composition
- `src/dosing/` — dose logs and historical nutrient snapshots
- `src/inventory/` — lots, stock movements and locations
- `src/reports/` — daily intake and trend reports

Existing paths such as `doseark_domain::Product` and
`doseark_domain::product::Product` remain available alongside
`doseark_domain::catalog::product::Product`.

## Tests

From the repository root:

```powershell
# Validate formatting without modifying files
cargo fmt --all -- --check

# First check after the version bump refreshes Cargo.lock
cargo check -p doseark_domain

# Run the isolated crate tests with the updated lockfile
cargo test -p doseark_domain --locked

# Check module compatibility and JSON snapshots independently
cargo test -p doseark_domain --test public_paths --locked
cargo test -p doseark_domain --test serialization --locked

# Check integration with dependent crates
cargo check --workspace --locked
```

Tests cover stable import paths, JSON names and record shapes, required fields and older history records. They do not need a database. `serde` models represent data; business rules are validated by application services and persistence adapters.
