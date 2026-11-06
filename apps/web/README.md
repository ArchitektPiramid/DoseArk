# DoseArk Web UI

React + TypeScript + Vite frontend for DoseArk.

## Commands

Run from the repository root:

```bash
pnpm dev:web
pnpm build:web
pnpm typecheck
pnpm lint
```

## Runtime modes

The Web UI talks to the DoseArk HTTP API. Tauri/local storage adapters are intentionally kept outside browser storage so the same frontend can be used in remote deployments.

See the main [README](../../README.md) for setup and project-level commands.
