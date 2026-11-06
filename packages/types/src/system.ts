import type { IsoDateTime } from "./common";

export type SystemHealthStatus = "ok" | "degraded";
export type DatabaseHealthStatus = "ok" | "error";

export type ComponentVersion = {
  version: string;
  channel: string;
};

export type DatabaseHealth = {
  status: DatabaseHealthStatus;
  engine: string;
  database_name: string | null;
  server_version: string | null;
  schema_version: string;
  expected_migration_revision: number;
  applied_migrations: number | null;
  applied_migration_revision: number | null;
  compatible: boolean;
  latency_ms: number;
  error: string | null;
};

export type BackendComponentVersions = {
  domain: string;
  application: string;
  contracts: string;
  db_postgres: string;
  db_sqlite: string;
};

export type SystemHealth = {
  status: SystemHealthStatus;
  service: string;
  version: string;
  channel: string;
  api: ComponentVersion;
  backend_components: BackendComponentVersions;
  storage_mode: string;
  checked_at: IsoDateTime;
  started_at: IsoDateTime;
  uptime_seconds: number;
  database: DatabaseHealth;
};

export type BackupFile = {
  name: string;
  size_bytes: number;
  modified_at: IsoDateTime;
  format: string;
  download_path: string;
};

export type BackupList = {
  status: "ok" | "unavailable" | "unsupported";
  directory: string;
  files: BackupFile[];
  can_create: boolean;
  error: string | null;
};

export type StorageCapabilities = {
  connection_stats: boolean;
  table_stats: boolean;
  backups: boolean;
  managed_backups: boolean;
};

export type DatabaseOverview = {
  engine: string;
  database_name: string;
  server_version: string;
  schema_version: string;
  expected_migration_revision: number;
  applied_migrations: number;
  applied_migration_revision: number;
  compatible: boolean;
  database_size_bytes: number;
  active_connections: number;
  max_connections: number;
  pool_size: number;
  pool_idle: number;
  capabilities: StorageCapabilities;
};

export type DatabaseTableStats = {
  schema_name: string;
  table_name: string;
  live_rows: number;
  dead_rows: number;
  sequential_scans: number;
  index_scans: number;
  table_bytes: number;
  index_bytes: number;
  total_bytes: number;
  last_autovacuum: IsoDateTime | null;
  last_autoanalyze: IsoDateTime | null;
};


export type RuntimeConfigSourceKind = "env" | "default";

export type RuntimeConfigValue = {
  value: string;
  source: RuntimeConfigSourceKind;
  variable: string | null;
};

export type RuntimeEnvironmentVariable = {
  name: string;
  value: string;
};

export type RuntimeConfiguration = {
  env_file: string | null;
  environment_variables: RuntimeEnvironmentVariable[];
  profiles_file: string;
  credentials_file: string;
  credentials_file_exists: boolean;
  storage_backend: RuntimeConfigValue;
  database_target: RuntimeConfigValue;
  backup_dir: RuntimeConfigValue;
  bind_addr: RuntimeConfigValue;
};

export type ProfileValidationStatus = "ok" | "warning" | "error" | "skipped";

export type ProfileValidationCheck = {
  key: string;
  status: ProfileValidationStatus;
  message: string;
};

export type ProfileValidation = {
  profile_id: string;
  profile_file: string;
  credentials_file: string;
  ok: boolean;
  checks: ProfileValidationCheck[];
};
