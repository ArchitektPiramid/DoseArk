import { getFallbackApiBaseUrl } from "@/app/client";
import type {
  ApiProfile,
  ApiProfileDatabaseType,
  ApiProfileMode,
  LoadedRuntimeApiConfig,
  RuntimeApiConfig,
  RuntimeApiProfileConfig,
} from "./apiProfile.types";
import { normalizeApiBaseUrl } from "./apiProfileUrl";

export const RUNTIME_CONFIG_FILE = "config/profiles.json";
export const RUNTIME_CONFIG_PROJECT_PATH = "config/profiles.json";

export async function loadRuntimeApiConfig(): Promise<LoadedRuntimeApiConfig> {
  const runtimeUrl = runtimeConfigLocation();

  try {
    const response = await fetch(runtimeUrl, { cache: "no-store" });
    if (!response.ok) throw new Error(`profiles.json returned ${response.status}`);

    const config = (await response.json()) as RuntimeApiConfig;
    if (!Array.isArray(config.apiProfiles) || config.apiProfiles.length === 0) {
      throw new Error("profiles.json contains no profiles");
    }

    return {
      config,
      source: {
        kind: "file",
        runtimeUrl,
        projectPath: RUNTIME_CONFIG_PROJECT_PATH,
      },
    };
  } catch {
    return {
      config: fallbackRuntimeConfig(),
      source: {
        kind: "fallback",
        runtimeUrl,
        projectPath: RUNTIME_CONFIG_PROJECT_PATH,
      },
    };
  }
}

export function normalizeRuntimeProfile(profile: RuntimeApiProfileConfig): ApiProfile {
  return {
    id: profile.id.trim(),
    name: profile.name.trim(),
    description: profile.description?.trim() || null,
    mode: normalizeProfileMode(profile.mode),
    baseUrl: normalizeOptionalUrl(profile.baseUrl),
    healthPath: profile.healthPath?.trim() || "/api/health",
    requestTimeoutMs: normalizePositiveInteger(profile.requestTimeoutMs, 5000),
    database: {
      type: normalizeDatabaseType(profile.database?.type),
      name: normalizeOptionalString(profile.database?.name),
      path: normalizeOptionalString(profile.database?.path),
      host: normalizeOptionalString(profile.database?.host),
      port: normalizeNullablePositiveInteger(profile.database?.port),
      sslMode: normalizeOptionalString(profile.database?.sslMode),
    },
    backupPath: normalizeOptionalString(profile.backupPath),
    attachmentsPath: normalizeOptionalString(profile.attachmentsPath),
    credentialsRef: normalizeOptionalString(profile.credentialsRef),
  };
}

export function runtimeConfigLocation(): string {
  if (typeof document === "undefined") return `/${RUNTIME_CONFIG_FILE}`;
  return new URL(RUNTIME_CONFIG_FILE, document.baseURI).toString();
}

function normalizeProfileMode(value: ApiProfileMode | undefined): ApiProfileMode {
  return value === "tauri-local" || value === "tauri-remote" ? value : "web-remote";
}

function normalizeDatabaseType(value: ApiProfileDatabaseType | undefined): ApiProfileDatabaseType {
  return value === "sqlite" || value === "postgres" ? value : "auto";
}

function normalizeOptionalUrl(value: string | undefined): string {
  if (!value?.trim()) return "";
  return normalizeApiBaseUrl(value);
}

function normalizeOptionalString(value: string | null | undefined): string | null {
  return value?.trim() || null;
}

function normalizePositiveInteger(value: number | undefined, fallback: number): number {
  return Number.isFinite(value) && Number(value) > 0 ? Math.trunc(Number(value)) : fallback;
}

function normalizeNullablePositiveInteger(value: number | null | undefined): number | null {
  return Number.isFinite(value) && Number(value) > 0 ? Math.trunc(Number(value)) : null;
}

function fallbackRuntimeConfig(): RuntimeApiConfig {
  return {
    configVersion: 1,
    defaultApiProfileId: "build-default",
    apiProfiles: [
      {
        id: "build-default",
        name: "Build default preview",
        description: "Fallback display profile generated from VITE_API_BASE_URL.",
        mode: "web-remote",
        baseUrl: getFallbackApiBaseUrl(),
        healthPath: "/api/health",
        requestTimeoutMs: 5000,
        database: { type: "auto" },
      },
    ],
  };
}
