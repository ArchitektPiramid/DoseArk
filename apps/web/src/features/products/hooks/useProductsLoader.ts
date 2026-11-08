import type { Brand, Product } from "@doseark/types";
import { useEffect, useRef, useState } from "react";

import { apiClient } from "@/app/client";
import { useApiProfiles } from "@/app/api_profiles";
import { createRequestGate } from "@/core/async/requestGate";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("products");

export type LoadStatus = "idle" | "loading" | "saving" | "error";

export function useProductsLoader() {
  const { effectiveApiBaseUrl } = useApiProfiles();
  const [products, setProducts] = useState<Product[]>([]);
  const [brands, setBrands] = useState<Brand[]>([]);
  const [status, setStatusState] = useState<LoadStatus>("idle");
  const [error, setError] = useState<string | null>(null);
  const requestGate = useRef(createRequestGate());
  const activeProfileUrl = useRef(effectiveApiBaseUrl);

  useEffect(() => () => requestGate.current.invalidate(), []);

  function setStatus(nextStatus: LoadStatus) {
    if (nextStatus === "saving") requestGate.current.invalidate();
    setStatusState(nextStatus);
  }

  async function loadAll() {
    if (activeProfileUrl.current !== effectiveApiBaseUrl) {
      activeProfileUrl.current = effectiveApiBaseUrl;
      requestGate.current.invalidate();
      setProducts([]);
      setBrands([]);
    }
    const requestId = requestGate.current.begin();
    const requestBaseUrl = apiClient.getBaseUrl();
    try {
      setStatusState("loading");
      setError(null);
      logger.trace("Loading products and brands started");

      const [productRows, brandRows] = await Promise.all([
        apiClient.listProducts(),
        apiClient.listBrands(),
      ]);
      if (!requestGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl())
        return null;

      setProducts(productRows);
      setBrands(brandRows);
      logger.debug("Products and brands loaded", {
        products: productRows.length,
        brands: brandRows.length,
      });
      setStatusState("idle");
      return { products: productRows, brands: brandRows };
    } catch (err) {
      if (!requestGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl())
        return null;
      logger.error("Failed to load products and brands", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatusState("error");
      return null;
    }
  }

  return { products, brands, status, error, setStatus, setError, loadAll };
}
