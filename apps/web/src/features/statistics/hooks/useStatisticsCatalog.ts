import type { Product } from "@doseark/types";
import { useEffect, useRef, useState } from "react";

import { apiClient } from "@/app/client";
import { createRequestGate } from "@/core/async/requestGate";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("statistics");

/** Loads the active product catalog independently of report filters and selection. */
export function useStatisticsCatalog(profileUrl: string) {
  const [products, setProducts] = useState<Product[]>([]);
  const [readyProfileUrl, setReadyProfileUrl] = useState<string | null>(null);
  const [status, setStatus] = useState<"loading" | "idle" | "error">("loading");
  const [error, setError] = useState<string | null>(null);
  const [revision, setRevision] = useState(0);
  const gate = useRef(createRequestGate());

  useEffect(() => {
    const requestId = gate.current.begin();
    const requestBaseUrl = apiClient.getBaseUrl();
    setProducts([]);
    setReadyProfileUrl(null);
    setStatus("loading");
    setError(null);

    async function load() {
      try {
        const rows = (await apiClient.listProducts()).filter((product) => !product.archived);
        if (!gate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl()) return;
        setProducts(rows);
        setReadyProfileUrl(profileUrl);
        setStatus("idle");
      } catch (loadError) {
        if (!gate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl()) return;
        logger.error("Failed to load statistics products", loadError);
        setStatus("error");
        setError(loadError instanceof Error ? loadError.message : "Unknown error");
      }
    }

    void load();
    return () => gate.current.invalidate();
  }, [profileUrl, revision]);

  return {
    products,
    ready: readyProfileUrl === profileUrl,
    status,
    error,
    refresh: () => setRevision((current) => current + 1),
  };
}
