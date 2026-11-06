# Local development

This repository uses two workspaces:

- Rust/Cargo (`Cargo.toml`): API `apps/api` and libraries `crates/`.
- pnpm (`pnpm-workspace.yaml`): UI `apps/web` and frontend packages `packages/client`, `packages/types`.

## Start

From the repository root:

```powershell
corepack enable
pnpm install --frozen-lockfile
Copy-Item .env.example .env
```

Run in separate terminals:

```powershell
pnpm dev:api
```

```powershell
pnpm dev:web
```

## Local checks

```bash
pnpm check
pnpm build:web
cargo fmt --all -- --check
cargo test --workspace
```

