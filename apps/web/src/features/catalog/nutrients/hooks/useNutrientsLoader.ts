import type { Nutrient } from "@doseark/types";
import { useEffect, useRef, useState } from "react";

import { useApiProfiles } from "@/app/api_profiles";
import { apiClient } from "@/app/client";
import { createRequestGate } from "@/core/async/requestGate";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("nutrients");
export type NutrientsStatus = "idle" | "loading" | "saving" | "error";

export function useNutrientsLoader() {
  const { effectiveApiBaseUrl } = useApiProfiles();
  const [nutrients, setNutrients] = useState<Nutrient[]>([]);
  const [status, setStatusState] = useState<NutrientsStatus>("idle");
  const [error, setError] = useState<string | null>(null);
  const requestGate = useRef(createRequestGate());
  const activeProfileUrl = useRef(effectiveApiBaseUrl);

  useEffect(() => () => requestGate.current.invalidate(), []);

  function setStatus(nextStatus: NutrientsStatus) {
    if (nextStatus === "saving") requestGate.current.invalidate();
    setStatusState(nextStatus);
  }

  async function loadNutrients() {
    if (activeProfileUrl.current !== effectiveApiBaseUrl) {
      activeProfileUrl.current = effectiveApiBaseUrl;
      requestGate.current.invalidate();
      setNutrients([]);
    }
    const requestId = requestGate.current.begin();
    const requestBaseUrl = apiClient.getBaseUrl();

    try {
      setStatusState("loading");
      setError(null);
      logger.trace("Loading nutrients started");
      const rows = await apiClient.listNutrients();
      if (!requestGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl()) return null;

      setNutrients(rows);
      logger.debug("Nutrients loaded", { count: rows.length });
      setStatusState("idle");
      return rows;
    } catch (err) {
      if (!requestGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl()) return null;
      logger.error("Failed to load nutrients", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatusState("error");
      return null;
    }
  }

  return { nutrients, status, error, setStatus, setError, loadNutrients };
}
