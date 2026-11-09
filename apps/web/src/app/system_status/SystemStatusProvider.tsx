import type { SystemHealth } from "@doseark/types";
import { createContext, type ReactNode, useCallback, useContext, useMemo, useState } from "react";

import { useApiProfiles } from "@/app/api_profiles";
import { createLogger } from "@/core/logger/logger";
import type { ConnectionStatus } from "./systemHealth";
import {
  getHealthRequestOptions,
  loadSettings,
  normalizeSettings,
  saveSettings,
  type ConnectionMonitoringSettings,
} from "./systemStatusSettings";
import { useSystemHealthPolling } from "./useSystemHealthPolling";

export type { ConnectionMonitoringSettings } from "./systemStatusSettings";
export type { ConnectionStatus } from "./systemHealth";

const logger = createLogger("system");

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

const SystemStatusContext = createContext<SystemStatusContextValue | null>(null);

/** Provides monitoring state while polling and storage remain isolated in focused modules. */
export function SystemStatusProvider({ children }: { children: ReactNode }) {
  const profiles = useApiProfiles();
  const [settings, setSettings] = useState<ConnectionMonitoringSettings>(loadSettings);
  const { timeoutMs, healthPath } = getHealthRequestOptions(
    settings,
    profiles.runtimeSource,
    profiles.selectedProfile,
  );
  const polling = useSystemHealthPolling({
    timeoutMs,
    healthPath,
    refreshIntervalSeconds: settings.refreshIntervalSeconds,
    profileKey: `${profiles.runtimeSource}:${profiles.selectedProfile?.id ?? ""}:${profiles.effectiveApiBaseUrl}`,
  });

  const updateSettings = useCallback((patch: Partial<ConnectionMonitoringSettings>) => {
    setSettings((current) => {
      const next = normalizeSettings({ ...current, ...patch });
      saveSettings(next);
      logger.info("Connection monitoring settings changed", next);
      return next;
    });
  }, []);

  const value = useMemo<SystemStatusContextValue>(
    () => ({ ...polling, settings, updateSettings }),
    [
      polling.status,
      polling.health,
      polling.error,
      polling.checkedAt,
      polling.roundTripMs,
      polling.isRefreshing,
      polling.refresh,
      settings,
      updateSettings,
    ],
  );

  return <SystemStatusContext.Provider value={value}>{children}</SystemStatusContext.Provider>;
}

export function useSystemStatus(): SystemStatusContextValue {
  const value = useContext(SystemStatusContext);
  if (!value) throw new Error("useSystemStatus must be used inside SystemStatusProvider");
  return value;
}
