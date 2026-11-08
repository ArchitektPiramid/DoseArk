import type { Brand, DailyIntakeReport, DoseLog, InventoryLot, Product } from "@doseark/types";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";

import { useApiProfiles } from "@/app/api_profiles";
import { apiClient } from "@/app/client";
import { createRequestGate } from "@/core/async/requestGate";
import { createLogger } from "@/core/logger/logger";
import { getActivityStart, loadActivityDoses } from "../utils/dashboardActivity";

const logger = createLogger("dashboard");
const PAGE_SIZE = 100;
const RECENT_DOSE_LIMIT = 5;

/** Loads the dashboard datasets and the recent activity window. */
export function useDashboardData() {
  const { effectiveApiBaseUrl } = useApiProfiles();
  const [products, setProducts] = useState<Product[]>([]);
  const [brands, setBrands] = useState<Brand[]>([]);
  const [lots, setLots] = useState<InventoryLot[]>([]);
  const [activityDoses, setActivityDoses] = useState<DoseLog[]>([]);
  const [activityTruncated, setActivityTruncated] = useState(false);
  const [recentDoses, setRecentDoses] = useState<DoseLog[]>([]);
  const [insightDoses, setInsightDoses] = useState<DoseLog[]>([]);
  const [insightTotal, setInsightTotal] = useState(0);
  const [todayReport, setTodayReport] = useState<DailyIntakeReport | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [hasLoaded, setHasLoaded] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const requestGate = useRef(createRequestGate());
  const activeProfileUrl = useRef(effectiveApiBaseUrl);

  const load = useCallback(async () => {
    if (activeProfileUrl.current !== effectiveApiBaseUrl) {
      activeProfileUrl.current = effectiveApiBaseUrl;
      requestGate.current.invalidate();
      setProducts([]);
      setBrands([]);
      setLots([]);
      setActivityDoses([]);
      setActivityTruncated(false);
      setRecentDoses([]);
      setInsightDoses([]);
      setInsightTotal(0);
      setTodayReport(null);
      setHasLoaded(false);
    }
    const requestId = requestGate.current.begin();
    const requestBaseUrl = apiClient.getBaseUrl();
    setIsLoading(true);
    setError(null);

    try {
      const now = new Date();
      const today = toLocalDateKey(now);
      const since = getActivityStart(now);
      const todayReportRequest = apiClient
        .getDailyIntakeReport(today, now.getTimezoneOffset())
        .catch((reportError) => {
          logger.warn("Dashboard today report unavailable", {
            message: reportError instanceof Error ? reportError.message : String(reportError),
          });
          return null;
        });

      const [productRows, brandRows, lotRows, firstPage, dailyReport] = await Promise.all([
        apiClient.listProducts(),
        apiClient.listBrands(),
        apiClient.listInventoryLots(),
        apiClient.listDoseLogs({ limit: PAGE_SIZE, offset: 0 }),
        todayReportRequest,
      ]);
      const activity = await loadActivityDoses(firstPage, since,
        (offset) => apiClient.listDoseLogs({ limit: PAGE_SIZE, offset }),
      );
      if (!requestGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl()) return;

      setProducts(productRows);
      setBrands(brandRows);
      setLots(lotRows);
      setRecentDoses(firstPage.items.slice(0, RECENT_DOSE_LIMIT));
      // The usage panel describes this bounded recent sample, not lifetime totals
      setInsightDoses(firstPage.items);
      setInsightTotal(firstPage.total);
      setActivityDoses(activity.doses);
      setActivityTruncated(activity.truncated);
      setTodayReport(dailyReport);
      setHasLoaded(true);

      logger.debug("Dashboard overview loaded", {
        products: productRows.length,
        lots: lotRows.length,
        recentDoses: firstPage.items.length,
        activityDoses: activity.doses.length,
        activityTruncated: activity.truncated,
      });
    } catch (loadError) {
      if (!requestGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl()) return;
      const message = loadError instanceof Error ? loadError.message : "Dashboard data request failed.";
      setError(message);
      logger.warn("Dashboard data load failed", { message });
    } finally {
      if (requestGate.current.isCurrent(requestId)) setIsLoading(false);
    }
  }, [effectiveApiBaseUrl]);

  useEffect(() => {
    void load();
    return () => requestGate.current.invalidate();
  }, [load]);

  const brandNameById = useMemo(() => new Map(brands.map((brand) => [brand.id, brand.name])), [brands]);
  const productNameById = useMemo(() => {
    const names = new Map<string, string>();
    for (const product of products) {
      const brandName = product.brand_id ? brandNameById.get(product.brand_id) : undefined;
      names.set(product.id, brandName ? `${brandName} / ${product.name}` : product.name);
    }
    return names;
  }, [brandNameById, products]);

  return {
    products,
    lots,
    activityDoses,
    activityTruncated,
    recentDoses,
    insightDoses,
    insightTotal,
    todayReport,
    productNameById,
    isLoading,
    hasLoaded,
    error,
    load,
  };
}

function toLocalDateKey(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
}
