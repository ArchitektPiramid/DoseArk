import type { SystemHealth } from "@doseark/types";
import {
  createContext,
  type ReactNode,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";

import { apiClient } from "@/app/client";
import { useApiProfiles } from "@/app/api_profiles";
import { createLogger } from "@/core/logger/logger";

const STORAGE_KEY = "doseark.connection-monitoring.v1";
const LEGACY_STORAGE_KEY = "medtaker.connection-monitoring.v1";
const logger = createLogger("system");

export type ConnectionStatus = "checking" | "ready" | "degraded" | "offline";

export type ConnectionMonitoringSettings = {
  refreshIntervalSeconds: number;
  requestTimeoutMs: number;
};

type SystemStatusContextValue = {
  status: ConnectionStatus;
  health: SystemHealth | null;
  error: string | null;
  checkedAt: Date | null;
  roundTripMs: number | null;
  isRefreshing: boolean;
  settings: ConnectionMonitoringSettings;
  refresh: () => Promise<void>;
  updateSettings: (patch: Partial<ConnectionMonitoringSettings>) => void;
};

const DEFAULT_SETTINGS: ConnectionMonitoringSettings = {
  refreshIntervalSeconds: 30,
  requestTimeoutMs: 5_000,
};

const SystemStatusContext = createContext<SystemStatusContextValue | null>(null);

export function SystemStatusProvider({ children }: { children: ReactNode }) {
  const profiles = useApiProfiles();
  const [status, setStatus] = useState<ConnectionStatus>("checking");
  const [health, setHealth] = useState<SystemHealth | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [checkedAt, setCheckedAt] = useState<Date | null>(null);
  const [roundTripMs, setRoundTripMs] = useState<number | null>(null);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [settings, setSettings] = useState<ConnectionMonitoringSettings>(loadSettings);
  const requestSequence = useRef(0);
  const lastLoggedStatus = useRef<ConnectionStatus | null>(null);

  const refresh = useCallback(async () => {
    const requestId = ++requestSequence.current;
    const startedAt = performance.now();

    setIsRefreshing(true);

    try {
      const useProfileSettings = profiles.runtimeSource === "profile" && Boolean(profiles.selectedProfile);
      const timeoutMs = useProfileSettings
        ? (profiles.selectedProfile?.requestTimeoutMs ?? settings.requestTimeoutMs) || 5_000
        : settings.requestTimeoutMs || 5_000;
      const healthPath = useProfileSettings
        ? profiles.selectedProfile?.healthPath || "/api/health"
        : "/api/health";
      const response = await apiClient.health(timeoutMs, healthPath);

      if (requestId !== requestSequence.current) return;

      if (!isCompatibleHealthResponse(response)) {
        setHealth(null);
        setStatus("degraded");
        setError(
          "The API is reachable, but its health response is from an older or incompatible version.",
        );
        setCheckedAt(new Date());
        setRoundTripMs(Math.max(0, Math.round(performance.now() - startedAt)));
        logStatusTransition(lastLoggedStatus, "degraded", {
          reason: "incompatible-health-response",
        });
        return;
      }

      const nextStatus: ConnectionStatus =
        response.status === "ok" && response.database.status === "ok" ? "ready" : "degraded";

      setHealth(response);
      setStatus(nextStatus);
      setError(response.database.error);
      setCheckedAt(new Date());
      setRoundTripMs(Math.max(0, Math.round(performance.now() - startedAt)));
      logStatusTransition(lastLoggedStatus, nextStatus, {
        apiStatus: response.status,
        databaseStatus: response.database.status,
      });
    } catch (requestError) {
      if (requestId !== requestSequence.current) return;

      setHealth(null);
      setStatus("offline");
      setError(formatConnectionError(requestError, settings.requestTimeoutMs));
      setCheckedAt(new Date());
      setRoundTripMs(null);
      logStatusTransition(lastLoggedStatus, "offline", {
        error: requestError,
      });
    } finally {
      if (requestId === requestSequence.current) {
        setIsRefreshing(false);
      }
    }
  }, [profiles.runtimeSource, profiles.selectedProfile, settings.requestTimeoutMs]);

  const updateSettings = useCallback((patch: Partial<ConnectionMonitoringSettings>) => {
    setSettings((current) => {
      const next = normalizeSettings({ ...current, ...patch });
      saveSettings(next);
      logger.info("Connection monitoring settings changed", next);
      return next;
    });
  }, []);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  useEffect(() => {
    if (settings.refreshIntervalSeconds <= 0) return;

    const interval = globalThis.setInterval(
      () => void refresh(),
      settings.refreshIntervalSeconds * 1_000,
    );

    return () => globalThis.clearInterval(interval);
  }, [refresh, settings.refreshIntervalSeconds]);

  useEffect(() => {
    function handleOnline() {
      void refresh();
    }

    function handleVisibilityChange() {
      if (document.visibilityState === "visible") {
        void refresh();
      }
    }

    globalThis.addEventListener("online", handleOnline);
    document.addEventListener("visibilitychange", handleVisibilityChange);

    return () => {
      globalThis.removeEventListener("online", handleOnline);
      document.removeEventListener("visibilitychange", handleVisibilityChange);
    };
  }, [refresh]);

  const value = useMemo<SystemStatusContextValue>(
    () => ({
      status,
      health,
      error,
      checkedAt,
      roundTripMs,
      isRefreshing,
      settings,
      refresh,
      updateSettings,
    }),
    [checkedAt, error, health, isRefreshing, refresh, roundTripMs, settings, status, updateSettings],
  );

  return <SystemStatusContext.Provider value={value}>{children}</SystemStatusContext.Provider>;
}

export function useSystemStatus(): SystemStatusContextValue {
  const value = useContext(SystemStatusContext);

  if (!value) {
    throw new Error("useSystemStatus must be used inside SystemStatusProvider");
  }

  return value;
}

function logStatusTransition(
  lastStatusRef: { current: ConnectionStatus | null },
  nextStatus: ConnectionStatus,
  details: unknown,
): void {
  if (lastStatusRef.current === nextStatus) {
    logger.trace("System health poll completed", { status: nextStatus });
    return;
  }

  const previousStatus = lastStatusRef.current;
  lastStatusRef.current = nextStatus;
  const transitionDetails = { previousStatus, nextStatus, details };

  if (nextStatus === "offline" || nextStatus === "degraded") {
    logger.warn("System connection status changed", transitionDetails);
  } else {
    logger.info("System connection status changed", transitionDetails);
  }
}

function loadSettings(): ConnectionMonitoringSettings {
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

function saveSettings(settings: ConnectionMonitoringSettings): void {
  try {
    globalThis.localStorage?.setItem(STORAGE_KEY, JSON.stringify(settings));
  } catch (error) {
    logger.warn("Could not persist connection monitoring settings", error);
  }
}

function normalizeSettings(settings: ConnectionMonitoringSettings): ConnectionMonitoringSettings {
  const refreshIntervalSeconds = [0, 15, 30, 60, 120].includes(settings.refreshIntervalSeconds)
    ? settings.refreshIntervalSeconds
    : DEFAULT_SETTINGS.refreshIntervalSeconds;

  const requestTimeoutMs = [3_000, 5_000, 10_000, 15_000].includes(settings.requestTimeoutMs)
    ? settings.requestTimeoutMs
    : DEFAULT_SETTINGS.requestTimeoutMs;

  return {
    refreshIntervalSeconds,
    requestTimeoutMs,
  };
}

function formatConnectionError(error: unknown, timeoutMs: number): string {
  if (error instanceof DOMException && error.name === "AbortError") {
    return `API health check timed out after ${Math.round(timeoutMs / 1_000)} seconds.`;
  }

  if (globalThis.navigator?.onLine === false) {
    return "The browser is offline. Check the network connection.";
  }

  if (error instanceof TypeError) {
    return "The API server could not be reached. Check whether it is running and whether the URL and CORS configuration are correct.";
  }

  return error instanceof Error ? error.message : "Unknown API connection error.";
}

function isCompatibleHealthResponse(value: SystemHealth): boolean {
  return Boolean(
    value &&
      typeof value.version === "string" &&
      value.database &&
      (value.database.status === "ok" || value.database.status === "error"),
  );
}
