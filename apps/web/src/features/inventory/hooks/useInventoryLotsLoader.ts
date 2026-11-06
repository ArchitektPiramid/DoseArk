import type { Brand, InventoryLot, Product, StorageLocation } from "@doseark/types";
import { useState } from "react";

import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("inventory");

export type InventoryStatus = "idle" | "loading" | "saving" | "error";

export function useInventoryLotsLoader() {
  const [lots, setLots] = useState<InventoryLot[]>([]);
  const [products, setProducts] = useState<Product[]>([]);
  const [brands, setBrands] = useState<Brand[]>([]);
  const [storageLocations, setStorageLocations] = useState<StorageLocation[]>([]);
  const [status, setStatus] = useState<InventoryStatus>("idle");
  const [error, setError] = useState<string | null>(null);

  async function loadAll() {
    try {
      setStatus("loading");
      setError(null);

      logger.trace("Loading inventory page data started");

      const [lotRows, productRows, brandRows, storageLocationRows] = await Promise.all([
        apiClient.listInventoryLots(),
        apiClient.listProducts(),
        apiClient.listBrands(),
        apiClient.listStorageLocations(false),
      ]);

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

      setStatus("idle");

      return {
        lots: lotRows,
        products: productRows,
        brands: brandRows,
        storageLocations: storageLocationRows,
      };
    } catch (err) {
      logger.error("Failed to load inventory page data", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatus("error");
      return null;
    }
  }

  return {
    lots,
    products,
    brands,
    storageLocations,
    status,
    error,
    setStatus,
    setError,
    loadAll,
  };
}
