# doseark_contracts

Serialization-only DTOs shared by the HTTP API and future Tauri IPC layer.
They do not contain domain rules, database access, or credential resolution.

`system` groups responses for health, database diagnostics, runtime configuration,
and profile validation. Existing crate-root and `system::*` imports remain valid.
Outgoing responses intentionally derive `Serialize`, not `Deserialize`.

The health response includes `channel`, `api` and `backend_components` metadata
alongside its existing status and database fields. Component versions reflect
the Cargo packages compiled into the running API.

The API must redact credentials and sensitive environment values **before**
constructing runtime configuration responses.

## Validation

Run from the workspace root:

```powershell
cargo fmt --all -- --check
cargo check -p doseark_contracts --locked
cargo test -p doseark_contracts --locked
cargo check --workspace --locked
```

Tests cover public import paths and exact JSON response shapes; no database is needed.
After changing the crate version, first run `cargo check -p doseark_contracts`
without `--locked` to refresh `Cargo.lock`, then repeat the locked checks.
