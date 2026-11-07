# doseark_db_sqlite

SQLite storage adapter for DoseArk

## Data integrity

- Inventory lots keep their original product
- New doses can use only active lots; cancellation still restores stock
- Editing dose metadata keeps historical nutrient snapshots and stock movements
- Quantity changes scale the original nutrient amounts
- Trend totals, daily buckets and events use consistent report-day bounds

## Tests

Run from the **workspace root**

**Formatting** — check without changes, then optionally format files:

```powershell
cargo fmt --all -- --check
cargo fmt --all
```

**Compilation and full SQLite tests:**

```powershell
cargo check -p doseark_db_sqlite
cargo test -p doseark_db_sqlite
```

**Run an individual integration suite:**

```powershell
cargo test -p doseark_db_sqlite --test inventory_dosing
cargo test -p doseark_db_sqlite --test reports
cargo test -p doseark_db_sqlite --test backup
```

**Workspace compilation** (after updating `Cargo.lock` when a version changes):

```powershell
cargo check --workspace --locked
```

`cargo fmt --all` modifies files; `--check` does not. Tests use temporary SQLite databases, not your local DoseArk database
