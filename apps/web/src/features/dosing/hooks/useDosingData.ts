import type { Brand, DoseLog, InventoryLot, Product } from "@doseark/types";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";

import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("dosing");

const DEFAULT_DOSE_LOG_PAGE_SIZE = 25;

type LoadOptions = {
  pageIndex?: number;
  pageSize?: number;
  productId?: string;
};

export function useDosingData() {
  const [doseLogs, setDoseLogs] = useState<DoseLog[]>([]);
  const [doseLogTotalCount, setDoseLogTotalCount] = useState(0);
  const [doseLogPageSize, setDoseLogPageSizeState] = useState(DEFAULT_DOSE_LOG_PAGE_SIZE);
  const [doseLogPageIndex, setDoseLogPageIndexState] = useState(0);

  const [lots, setLots] = useState<InventoryLot[]>([]);
  const [products, setProducts] = useState<Product[]>([]);
  const [brands, setBrands] = useState<Brand[]>([]);

  const [historyProductId, setHistoryProductIdState] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const loadRequestId = useRef(0);

  const productById = useMemo(() => {
    return new Map(products.map((product) => [product.id, product]));
  }, [products]);

  const brandNameById = useMemo(() => {
    return new Map(brands.map((brand) => [brand.id, brand.name]));
  }, [brands]);

  const productNameById = useMemo(() => {
    const map = new Map<string, string>();

    for (const product of products) {
      const brandName = product.brand_id ? brandNameById.get(product.brand_id) : undefined;
      map.set(product.id, brandName ? `${brandName} / ${product.name}` : product.name);
    }

    return map;
  }, [products, brandNameById]);

  const lotNumberById = useMemo(() => {
    const map = new Map<string, string>();

    for (const lot of lots) {
      map.set(lot.id, lot.lot_number?.trim() || "—");
    }

    return map;
  }, [lots]);

  const loadAll = useCallback(
    async (options?: LoadOptions) => {
      const requestId = loadRequestId.current + 1;
      loadRequestId.current = requestId;

      const nextPageIndex = options?.pageIndex ?? doseLogPageIndex;
      const nextPageSize = options?.pageSize ?? doseLogPageSize;
      const nextProductId = options?.productId ?? historyProductId;

      try {
        setIsLoading(true);
        setError(null);

        logger.trace("Loading dosing page data", {
          doseLogPageIndex: nextPageIndex,
          doseLogPageSize: nextPageSize,
          historyProductId: nextProductId || null,
        });

        const [dosePage, lotRows, productRows, brandRows] = await Promise.all([
          apiClient.listDoseLogs({
            limit: nextPageSize,
            offset: nextPageIndex * nextPageSize,
            product_id: nextProductId || null,
          }),
          apiClient.listInventoryLots(),
          apiClient.listProducts(),
          apiClient.listBrands(),
        ]);

        if (loadRequestId.current !== requestId) {
          logger.trace("Ignoring stale dosing page response", { requestId });
          return;
        }

        setDoseLogs(dosePage.items);
        setDoseLogTotalCount(dosePage.total);
        setLots(lotRows);
        setProducts(productRows);
        setBrands(brandRows);

        const lastPageIndex = Math.max(0, Math.ceil(dosePage.total / nextPageSize) - 1);
        if (nextPageIndex > lastPageIndex) {
          setDoseLogPageIndexState(lastPageIndex);
        }

        logger.debug("Dosing page data loaded", {
          doseLogs: dosePage.items.length,
          totalDoseLogs: dosePage.total,
          lots: lotRows.length,
          products: productRows.length,
          brands: brandRows.length,
        });
      } catch (err) {
        if (loadRequestId.current !== requestId) {
          return;
        }

        logger.error("Failed to load dosing page data", err);
        setError(err instanceof Error ? err.message : "Unknown error");
      } finally {
        if (loadRequestId.current === requestId) {
          setIsLoading(false);
        }
      }
    },
    [doseLogPageIndex, doseLogPageSize, historyProductId],
  );

  const setHistoryProductId = useCallback((productId: string) => {
    setHistoryProductIdState(productId);
    setDoseLogPageIndexState(0);
  }, []);

  const setDoseLogPageSize = useCallback((pageSize: number) => {
    setDoseLogPageSizeState(pageSize);
    setDoseLogPageIndexState(0);
  }, []);

  const setDoseLogPageIndex = useCallback((pageIndex: number) => {
    setDoseLogPageIndexState(Math.max(0, pageIndex));
  }, []);

  useEffect(() => {
    void loadAll({
      pageIndex: doseLogPageIndex,
      pageSize: doseLogPageSize,
      productId: historyProductId,
    });
  }, [doseLogPageIndex, doseLogPageSize, historyProductId, loadAll]);

  return {
    doseLogs,
    doseLogTotalCount,
    doseLogPageSize,
    doseLogPageIndex,
    setDoseLogPageSize,
    setDoseLogPageIndex,

    lots,
    products,
    productById,
    productNameById,
    lotNumberById,

    historyProductId,
    setHistoryProductId,

    isLoading,
    error,
    loadAll,
  };
}
