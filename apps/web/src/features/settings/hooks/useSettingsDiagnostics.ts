import type { BackupList, DatabaseOverview, RuntimeConfiguration } from "@doseark/types";
import { useCallback, useEffect, useState } from "react";

import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("settings");

export function useSettingsDiagnostics() {
  const [backups, setBackups] = useState<BackupList | null>(null);
  const [database, setDatabase] = useState<DatabaseOverview | null>(null);
  const [runtimeConfiguration, setRuntimeConfiguration] = useState<RuntimeConfiguration | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const refresh = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    setBackups(null);
    setDatabase(null);
    setRuntimeConfiguration(null);

    try {
      const [overview, runtimeConfig] = await Promise.all([
        apiClient.getDatabaseOverview(),
        apiClient.getRuntimeConfiguration(),
      ]);
      setDatabase(overview);
      setRuntimeConfiguration(runtimeConfig);

      if (overview.capabilities.backups) {
        try {
          setBackups(await apiClient.listBackups());
        } catch (backupError) {
          const message = backupError instanceof Error ? backupError.message : "Backup information request failed.";
          setError(message);
          logger.warn("Backup information failed", { message });
        }
      }

      logger.debug("Storage settings loaded", {
        storageEngine: overview.engine,
        backupsSupported: overview.capabilities.backups,
      });
    } catch (overviewError) {
      const message = overviewError instanceof Error
        ? overviewError.message
        : "Storage information request failed.";
      setError(message);
      logger.warn("Storage overview failed", { message });
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  return { backups, database, runtimeConfiguration, error, isLoading, refresh };
}
