# Changelog

## [Unreleased]

### Changed
- Refactored Rust domain, application, contracts, runtime config, database adapters and API into focused modules
- Grouped HTTP routes and tests by responsibility; aligned `/api/health` with TypeScript contracts

### Fixed
- Ignored stale API responses during profile switches
- Protected inventory lot ownership and active-lot dosing in SQLite and PostgreSQL
- Preserved historical nutrient snapshots and aligned nutrient report date ranges
- Redacted credential and internal API error details; rejected symlinked backup files

### Tests
- Covered web forms, bulk doses and chart aggregation
- Added regression coverage for persistence, service validation, JSON contracts and runtime configuration
