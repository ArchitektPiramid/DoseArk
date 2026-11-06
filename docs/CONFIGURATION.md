# Local configuration

DoseArk is currently intended to run **locally**, with two terminals and no Docker setup.

## 1. API environment (`.env` at repository root)

Copy `.env.example` to `.env` and review values:

```env
DOSEARK_STORAGE=sqlite
DOSEARK_SQLITE_PATH=./data/doseark.sqlite
DOSEARK_API_BIND=127.0.0.1:8080
```

- `DOSEARK_STORAGE`: `sqlite` or `postgres` (required).
- `DOSEARK_SQLITE_PATH`: SQLite database file; created/initialized by the API as needed.
- `DATABASE_URL`: only needed for PostgreSQL; never commit real credentials.
- `DOSEARK_API_BIND`: bind address; use `127.0.0.1:8080` for local-only operation.
- `RUST_LOG`: startup logging filters.
- `DOSEARK_RUNTIME_SETTINGS_PATH`: persisted local settings (under `data/` by default).
- `DOSEARK_BACKUP_DIR`: optional override for supported backups.
- `DOSEARK_PROFILES_FILE`: optional file used by the backend profile-validation endpoint.

The backend also understands some legacy `MEDTAKER_*` names. `.env` and all `data/` content should stay local and ignored by Git.

## 2. Web frontend

Default browser API URL: `http://localhost:8080`. To override it, create `apps/web/.env.local` from `apps/web/.env.example`:

```env
VITE_API_BASE_URL=http://localhost:8080
```

Vite's server runs at `http://localhost:5173`

## 3. Optional display profiles

The frontend Vite plugin reads the ignored `config/profiles.json`, or uses the tracked `config/profiles.example.json` if no local override exists. The JSON is served to the browser as `/config/profiles.json` and included in web builds. Therefore, do **not** store secrets or sensitive local infrastructure data there.

The profile selector changes which API URL the browser uses; it **does not** switch the database type for the running backend. The real database configuration is set by the API environment variables.

The backend's optional profile diagnostics accept configuration versions 1 and 2. The local SQLite example requires no credentials file.

## 4. PostgreSQL (optional local/manual configuration)

If you have your own PostgreSQL service, use `.env.postgres.example` as a guide and set a real `DATABASE_URL` on your machine. PostgreSQL itself must be started separately; Docker Compose is **not** part of the supported setup yet.

