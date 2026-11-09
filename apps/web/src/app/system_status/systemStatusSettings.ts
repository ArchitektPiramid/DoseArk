import { createLogger } from "@/core/logger/logger";

const STORAGE_KEY = "doseark.connection-monitoring.v1";
const LEGACY_STORAGE_KEY = "medtaker.connection-monitoring.v1";
const logger = createLogger("system");

export type ConnectionMonitoringSettings = {
  refreshIntervalSeconds: number;
  requestTimeoutMs: number;
};

export const DEFAULT_SETTINGS: ConnectionMonitoringSettings = {
  refreshIntervalSeconds: 30,
  requestTimeoutMs: 5_000,
};

export function normalizeSettings(
  settings: ConnectionMonitoringSettings,
): ConnectionMonitoringSettings {
  const refreshIntervalSeconds = [0, 15, 30, 60, 120].includes(settings.refreshIntervalSeconds)
    ? settings.refreshIntervalSeconds
    : DEFAULT_SETTINGS.refreshIntervalSeconds;
  const requestTimeoutMs = [3_000, 5_000, 10_000, 15_000].includes(settings.requestTimeoutMs)
    ? settings.requestTimeoutMs
    : DEFAULT_SETTINGS.requestTimeoutMs;
  return { refreshIntervalSeconds, requestTimeoutMs };
}

/** Loads persisted monitoring preferences and migrates the former storage key. */
export function loadSettings(): ConnectionMonitoringSettings {
  try {
    const storage = globalThis.localStorage;
    let stored = storage?.getItem(STORAGE_KEY);
    if (!stored) {
      stored = storage?.getItem(LEGACY_STORAGE_KEY);
      if (stored) {
        storage?.setItem(STORAGE_KEY, stored);
        storage?.removeItem(LEGACY_STORAGE_KEY);
        logger.info("Migrated connection monitoring settings to DoseArk storage key");
      }
    }
    if (!stored) return DEFAULT_SETTINGS;
    return normalizeSettings({
      ...DEFAULT_SETTINGS,
      ...(JSON.parse(stored) as Partial<ConnectionMonitoringSettings>),
    });
  } catch (error) {
    logger.warn("Could not load connection monitoring settings", error);
    return DEFAULT_SETTINGS;
  }
}

export function saveSettings(settings: ConnectionMonitoringSettings): void {
  try {
    globalThis.localStorage?.setItem(STORAGE_KEY, JSON.stringify(settings));
  } catch (error) {
    logger.warn("Could not persist connection monitoring settings", error);
  }
}

/** Profile-specific health options override defaults only in profile mode. */
export function getHealthRequestOptions(
  settings: ConnectionMonitoringSettings,
  runtimeSource: string,
  profile: { requestTimeoutMs?: number | null; healthPath?: string | null } | null,
) {
  const useProfile = runtimeSource === "profile" && Boolean(profile);
  return {
    timeoutMs: useProfile
      ? (profile?.requestTimeoutMs ?? settings.requestTimeoutMs) || 5_000
      : settings.requestTimeoutMs || 5_000,
    healthPath: useProfile ? profile?.healthPath || "/api/health" : "/api/health",
  };
}
