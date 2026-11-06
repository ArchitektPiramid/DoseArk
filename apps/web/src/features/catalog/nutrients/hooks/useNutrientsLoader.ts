import type { Nutrient } from "@doseark/types";
import { useState } from "react";
import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("nutrients");

export type NutrientsStatus = "idle" | "loading" | "saving" | "error";

export function useNutrientsLoader() {
  const [nutrients, setNutrients] = useState<Nutrient[]>([]);
  const [status, setStatus] = useState<NutrientsStatus>("idle");
  const [error, setError] = useState<string | null>(null);

  async function loadNutrients() {
    try {
      setStatus("loading");
      setError(null);

      logger.trace("Loading nutrients started");

      const rows = await apiClient.listNutrients();
      setNutrients(rows);

      logger.debug("Nutrients loaded", { count: rows.length });

      setStatus("idle");
      return rows;
    } catch (err) {
      logger.error("Failed to load nutrients", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatus("error");
      return null;
    }
  }

  return {
    nutrients,
    status,
    error,
    setStatus,
    setError,
    loadNutrients,
  };
}
