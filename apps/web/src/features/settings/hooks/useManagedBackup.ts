import { useCallback, useState } from "react";

import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("settings-backup");

export function useManagedBackup(onRefresh: () => Promise<void>) {
  const [creating, setCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const createBackup = useCallback(async () => {
    setCreating(true);
    setError(null);

    try {
      const backup = await apiClient.createBackup();
      logger.info("Managed storage backup created", {
        sizeBytes: backup.size_bytes,
        format: backup.format,
      });
      await onRefresh();
    } catch (err) {
      const message = err instanceof Error ? err.message : "Unknown backup error";
      setError(message);
      logger.error("Managed storage backup failed", err);
    } finally {
      setCreating(false);
    }
  }, [onRefresh]);

  return { creating, error, createBackup };
}
