import type { Brand, Product } from "@doseark/types";
import { useState } from "react";

import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("products");

export type LoadStatus = "idle" | "loading" | "saving" | "error";

export function useProductsLoader() {
  const [products, setProducts] = useState<Product[]>([]);
  const [brands, setBrands] = useState<Brand[]>([]);
  const [status, setStatus] = useState<LoadStatus>("idle");
  const [error, setError] = useState<string | null>(null);

  async function loadAll() {
    try {
      setStatus("loading");
      setError(null);

      logger.trace("Loading products and brands started");

      const [productRows, brandRows] = await Promise.all([
        apiClient.listProducts(),
        apiClient.listBrands(),
      ]);

      setProducts(productRows);
      setBrands(brandRows);

      logger.debug("Products and brands loaded", {
        products: productRows.length,
        brands: brandRows.length,
      });

      setStatus("idle");

      return {
        products: productRows,
        brands: brandRows,
      };
    } catch (err) {
      logger.error("Failed to load products and brands", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatus("error");

      return null;
    }
  }

  return {
    products,
    brands,
    status,
    error,
    setStatus,
    setError,
    loadAll,
  };
}
