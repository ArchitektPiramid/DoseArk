# doseark_application

Use-case services, validation and repository ports for DoseArk. The crate does not depend on HTTP frameworks or database drivers.

- `ports/` — repository interfaces implemented by SQLite and PostgreSQL adapters
- `services/` — application use cases and their validation
- `bundles.rs` — constructor wiring repositories into services
- `storage.rs` — storage diagnostics and backup interfaces

## Checks

```powershell
cargo fmt --all -- --check
cargo check -p doseark_application --locked
cargo test -p doseark_application --locked
cargo check --workspace --locked
```

Tests of pure validation run without a database. For persistence behavior, use the adapter crate integration tests.
