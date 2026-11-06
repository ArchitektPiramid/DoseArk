import type { StockMovement } from "@doseark/types";
import { useEffect, useState } from "react";

import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("inventory");

type Status = "idle" | "loading" | "error";

export function useInventoryStockMovements(lotId?: string) {
  const [rows, setRows] = useState<StockMovement[]>([]);
  const [status, setStatus] = useState<Status>("idle");
  const [error, setError] = useState<string | null>(null);

  async function loadRows(nextLotId: string) {
    try {
      setStatus("loading");
      setError(null);

      logger.trace("Loading stock movements for lot", { lotId: nextLotId });

      const result = await apiClient.listStockMovementsForLot(nextLotId);
      setRows(result);

      logger.debug("Stock movements loaded", {
        lotId: nextLotId,
        rows: result.length,
      });

      setStatus("idle");
    } catch (err) {
      logger.error("Failed to load stock movements", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatus("error");
    }
  }

  useEffect(() => {
    if (!lotId) {
      setRows([]);
      setStatus("idle");
      setError(null);
      return;
    }

    void loadRows(lotId);
  }, [lotId]);

  return {
    rows,
    status,
    error,
    reload: lotId ? () => loadRows(lotId) : undefined,
  };
}
