# TESTS:

```powershell
# for preview only
cargo fmt --all -- --check
# !! auto update formating all
cargo fmt --all

### TEST ALL
cargo test -p doseark_db_sqlite
# invidual
cargo test -p doseark_db_sqlite --test inventory_dosing
cargo test -p doseark_db_sqlite --test backup


cargo check --workspace --locked
```

All tests use temporary SQLite databases, never the user's local database
