import type { Brand } from "@doseark/types";
import { useState } from "react";
import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("brands");

export type BrandsStatus = "idle" | "loading" | "saving" | "error";

export function useBrandsLoader() {
  const [brands, setBrands] = useState<Brand[]>([]);
  const [status, setStatus] = useState<BrandsStatus>("idle");
  const [error, setError] = useState<string | null>(null);

  async function loadBrands() {
    try {
      setStatus("loading");
      setError(null);

      logger.trace("Loading brands started");

      const rows = await apiClient.listBrands();
      setBrands(rows);

      logger.debug("Brands loaded", { count: rows.length });

      setStatus("idle");
      return rows;
    } catch (err) {
      logger.error("Failed to load brands", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatus("error");
      return null;
    }
  }

  return {
    brands,
    status,
    error,
    setStatus,
    setError,
    loadBrands,
  };
}
