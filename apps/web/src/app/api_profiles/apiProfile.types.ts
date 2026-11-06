export type ApiProfileSource = "file" | "fallback";
export type RuntimeSourceMode = "env" | "profile";
export type ApiProfileMode = "web-remote" | "tauri-remote" | "tauri-local";
export type ApiProfileDatabaseType = "auto" | "sqlite" | "postgres";

export type ApiProfileDatabase = {
  type: ApiProfileDatabaseType;
  name: string | null;
  path: string | null;
  host: string | null;
  port: number | null;
  sslMode: string | null;
};

export type ApiProfile = {
  id: string;
  name: string;
  description: string | null;
  mode: ApiProfileMode;
  baseUrl: string;
  healthPath: string;
  requestTimeoutMs: number;
  database: ApiProfileDatabase;
  backupPath: string | null;
  attachmentsPath: string | null;
  credentialsRef: string | null;
};

export type RuntimeApiProfileConfig = {
  id: string;
  name: string;
  description?: string | null;
  mode?: ApiProfileMode;
  baseUrl?: string;
  healthPath?: string;
  requestTimeoutMs?: number;
  database?: {
    type?: ApiProfileDatabaseType;
    name?: string | null;
    path?: string | null;
    host?: string | null;
    port?: number | null;
    sslMode?: string | null;
  };
  credentialsRef?: string | null;
  backupPath?: string | null;
  attachmentsPath?: string | null;
};

export type RuntimeApiConfig = {
  configVersion: number;
  defaultApiProfileId: string;
  apiProfiles: RuntimeApiProfileConfig[];
};

export type RuntimeConfigSource = {
  kind: ApiProfileSource;
  runtimeUrl: string;
  projectPath: string;
};

export type LoadedRuntimeApiConfig = {
  config: RuntimeApiConfig;
  source: RuntimeConfigSource;
};
