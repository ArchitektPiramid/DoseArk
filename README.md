# DoseArk

**DoseArk** is a local, actively developed app for tracking medicines and supplements, product composition, inventory and intake history. It currently runs as **two separate processes**: a **React + TypeScript web frontend** and a **Rust/Axum HTTP API**. Data is stored in **SQLite** (recommended for the first run) or **PostgreSQL**.

[Features](docs/FEATURES.md) · [Architecture](docs/ARCHITECTURE.md) · [Local configuration](docs/CONFIGURATION.md)

> **Development version / local use only.** There is no access control or user authentication in the current API. Do not expose the API or Vite dev server to a LAN or the Internet. DoseArk is not a medical device and does not provide medical advice.

## Local quick start (SQLite)

Requirements: Node.js compatible with Vite 8 (Node **20.19+ or 22.12+**), pnpm **10.34.3** (see `package.json`), and the **Rust** toolchain (see `rust-toolchain.toml`). SQLite does not require a database server.

From the repository root:

```bash
corepack enable
pnpm install --frozen-lockfile
```

Create your local API configuration:

```powershell
# PowerShell (Windows)
Copy-Item .env.example .env
```

Or on Linux/macOS: `cp .env.example .env`.

In **two terminals** opened at the repository root:

```bash
# terminal A: backend (automatically initializes SQLite and runs migrations)
pnpm dev:api
```

```bash
# terminal B: frontend
pnpm dev:web
```

Open **http://localhost:5173** in your browser. The API is at **http://localhost:8080**, with diagnostics at **http://localhost:8080/api/health**. The web frontend works without Docker or an API token.

If your existing `.env` sets `DOSEARK_API_BIND=0.0.0.0:8080`, change it to `127.0.0.1:8080` for local-only use. `.env` is not overwritten by updates.

The browser uses the tracked `config/profiles.example.json` if you have not created `config/profiles.json`. You can optionally copy it to the ignored local file to customize display profiles, but **never place passwords, tokens or private information there**: profile JSON is served to the browser by Vite.

## Available now

- Products, brands, nutrient definitions and product composition.
- Inventory lots, storage locations, stock changes and balances.
- Single and bulk intake entries, history, cancellation and nutrient snapshots.
- Dashboard, intake statistics and charts.
- Local API diagnostics, application settings and supported database backup functions.

See [Feature inventory](docs/current/FEATURES.md) for code locations and limitations. These capabilities exist in source code; this is not a claim that every flow has been end-to-end tested.

## Project structure

```text
apps/api/                 Rust / Axum HTTP API
apps/web/                 React / TypeScript / Vite UI
crates/doseark_domain/    Domain entities and rules
crates/doseark_application/ Use cases and repository interfaces
crates/doseark_contracts/  Rust HTTP contracts
crates/doseark_db_sqlite/  SQLite adapter and migrations
crates/doseark_db_postgres/ PostgreSQL adapter and migrations
crates/doseark_runtime_config/ Optional profile validation
packages/client/         Reusable TypeScript HTTP client
packages/types/          Shared frontend types
config/                  Local development profile examples
```

For a data-flow explanation, see [Architecture](docs/current/ARCHITECTURE.md).

## Checks

```bash
pnpm check
pnpm build:web
cargo test --workspace
cargo fmt --all -- --check
```

CI is intentionally not configured yet. See [Development](docs/current/DEVELOPMENT.md) and [Contributing](CONTRIBUTING.md).

## Future work

**Docker / Docker Compose deployment**, Android, Tauri and offline synchronization are future plans. The existing `docker/` directory is experimental and **not** a supported deployment method; see [TODO.md](TODO.md).

## Security and license

This repository is intended for local development. Do not upload your `.env`, database, backups or personal medicine records. See [SECURITY.md](SECURITY.md). Licensed under [MIT](LICENSE).
