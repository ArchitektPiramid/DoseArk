import type { Brand, InventoryLot, Product, StorageLocation } from "@doseark/types";
import { useEffect, useRef, useState } from "react";

import { apiClient } from "@/app/client";
import { useApiProfiles } from "@/app/api_profiles";
import { createRequestGate } from "@/core/async/requestGate";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("inventory");

export type InventoryStatus = "idle" | "loading" | "saving" | "error";

export function useInventoryLotsLoader() {
  const { effectiveApiBaseUrl } = useApiProfiles();
  const [lots, setLots] = useState<InventoryLot[]>([]);
  const [products, setProducts] = useState<Product[]>([]);
  const [brands, setBrands] = useState<Brand[]>([]);
  const [storageLocations, setStorageLocations] = useState<StorageLocation[]>([]);
  const [status, setStatusState] = useState<InventoryStatus>("idle");
  const [error, setError] = useState<string | null>(null);
  const requestGate = useRef(createRequestGate());
  const activeProfileUrl = useRef(effectiveApiBaseUrl);

  useEffect(() => () => requestGate.current.invalidate(), []);

  function setStatus(nextStatus: InventoryStatus) {
    if (nextStatus === "saving") requestGate.current.invalidate();
    setStatusState(nextStatus);
  }

  async function loadAll() {
    if (activeProfileUrl.current !== effectiveApiBaseUrl) {
      activeProfileUrl.current = effectiveApiBaseUrl;
      requestGate.current.invalidate();
      setLots([]);
      setProducts([]);
      setBrands([]);
      setStorageLocations([]);
    }
    const requestId = requestGate.current.begin();
    const requestBaseUrl = apiClient.getBaseUrl();
    try {
      setStatusState("loading");
      setError(null);
      logger.trace("Loading inventory page data started");

      const [lotRows, productRows, brandRows, storageLocationRows] = await Promise.all([
        apiClient.listInventoryLots(),
        apiClient.listProducts(),
        apiClient.listBrands(),
        apiClient.listStorageLocations(false),
      ]);
      if (!requestGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl()) return null;

      setLots(lotRows);
      setProducts(productRows);
      setBrands(brandRows);
      setStorageLocations(storageLocationRows);

      logger.debug("Inventory page data loaded", {
        lots: lotRows.length,
        products: productRows.length,
        brands: brandRows.length,
        storageLocations: storageLocationRows.length,
      });
      setStatusState("idle");
      return { lots: lotRows, products: productRows, brands: brandRows, storageLocations: storageLocationRows };
    } catch (err) {
      if (!requestGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl()) return null;
      logger.error("Failed to load inventory page data", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatusState("error");
      return null;
    }
  }

  return { lots, products, brands, storageLocations, status, error, setStatus, setError, loadAll };
}
