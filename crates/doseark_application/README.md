# doseark_application

Use-case services, validation and repository ports for DoseArk. This crate does not depend on HTTP frameworks or database drivers.

- `ports/` — persistence interfaces for SQLite and PostgreSQL adapters
- `services/` — application use cases and input validation
- `bundles.rs` — dependency injection without a container framework
- `storage.rs` — storage diagnostics and backup interfaces

## Checks

Run from the repository root:

```powershell
# Check formatting without modifying files
cargo fmt --all -- --check

# First check refreshes Cargo.lock after the version bump
cargo check -p doseark_application

# Run all application tests, including service wiring
cargo test -p doseark_application --locked

# Check the entire workspace
cargo check --workspace --locked
```

Run the wiring test alone with `cargo test -p doseark_application --test service_wiring --locked`.
Tests use in-memory repository doubles; no SQLite or PostgreSQL database is needed.
