# TESTS:

```powershell
cargo fmt --all -- --check
cargo test -p doseark_db_sqlite --test inventory_dosing
cargo test -p doseark_db_sqlite
cargo check --workspace --locked
```

All tests use temporary SQLite databases, never the user's local database
