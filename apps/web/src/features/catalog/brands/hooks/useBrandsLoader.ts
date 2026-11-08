import type { Brand } from "@doseark/types";
import { useEffect, useRef, useState } from "react";

import { useApiProfiles } from "@/app/api_profiles";
import { apiClient } from "@/app/client";
import { createRequestGate } from "@/core/async/requestGate";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("brands");
export type BrandsStatus = "idle" | "loading" | "saving" | "error";

export function useBrandsLoader() {
  const { effectiveApiBaseUrl } = useApiProfiles();
  const [brands, setBrands] = useState<Brand[]>([]);
  const [status, setStatusState] = useState<BrandsStatus>("idle");
  const [error, setError] = useState<string | null>(null);
  const requestGate = useRef(createRequestGate());
  const activeProfileUrl = useRef(effectiveApiBaseUrl);

  useEffect(() => () => requestGate.current.invalidate(), []);

  function setStatus(nextStatus: BrandsStatus) {
    if (nextStatus === "saving") requestGate.current.invalidate();
    setStatusState(nextStatus);
  }

  async function loadBrands() {
    if (activeProfileUrl.current !== effectiveApiBaseUrl) {
      activeProfileUrl.current = effectiveApiBaseUrl;
      requestGate.current.invalidate();
      setBrands([]);
    }
    const requestId = requestGate.current.begin();
    const requestBaseUrl = apiClient.getBaseUrl();

    try {
      setStatusState("loading");
      setError(null);
      logger.trace("Loading brands started");
      const rows = await apiClient.listBrands();
      if (!requestGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl()) return null;

      setBrands(rows);
      logger.debug("Brands loaded", { count: rows.length });
      setStatusState("idle");
      return rows;
    } catch (err) {
      if (!requestGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl()) return null;
      logger.error("Failed to load brands", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatusState("error");
      return null;
    }
  }

  return { brands, status, error, setStatus, setError, loadBrands };
}
