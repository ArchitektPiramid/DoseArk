import type { NutrientTrendOption, NutrientTrendReport, Product } from "@doseark/types";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";

import { useApiProfiles } from "@/app/api_profiles";
import { apiClient } from "@/app/client";
import { createRequestGate } from "@/core/async/requestGate";
import { createLogger } from "@/core/logger/logger";
import {
  createStatisticsReportRequest,
  type StatisticsReportSubject,
} from "../utils/statisticsReportRequest";
import { getStatisticsPresetRange, type StatisticsDateRange } from "../utils/statisticsRange";
import {
  buildStatisticsNutrientKey,
  parseStatisticsNutrientKey,
} from "../utils/statisticsSelection";

export type StatisticsSource = "product" | "nutrient";
type Status = "idle" | "loading" | "error";

const logger = createLogger("statistics");

export function useStatistics() {
  const { effectiveApiBaseUrl } = useApiProfiles();
  const [range, setRange] = useState<StatisticsDateRange>(() => getStatisticsPresetRange(30));
  const [source, setSource] = useState<StatisticsSource>("product");
  const [products, setProducts] = useState<Product[]>([]);
  const [nutrients, setNutrients] = useState<NutrientTrendOption[]>([]);
  const [selectedProductId, setSelectedProductId] = useState<string | null>(null);
  const [selectedNutrientKey, setSelectedNutrientKeyState] = useState<string | null>(null);
  const [report, setReport] = useState<NutrientTrendReport | null>(null);
  const [status, setStatus] = useState<Status>("loading");
  const [error, setError] = useState<string | null>(null);
  const [readyProfileUrl, setReadyProfileUrl] = useState<string | null>(null);
  const [catalogRevision, setCatalogRevision] = useState(0);
  const [reportRevision, setReportRevision] = useState(0);
  const selectedNutrientKeyRef = useRef<string | null>(null);
  const catalogGate = useRef(createRequestGate());
  const reportGate = useRef(createRequestGate());

  const selectedProduct = useMemo(
    () => products.find((product) => product.id === selectedProductId) ?? null,
    [products, selectedProductId],
  );
  const selectedNutrient = useMemo(
    () => parseStatisticsNutrientKey(selectedNutrientKey),
    [selectedNutrientKey],
  );

  const getReport = useCallback(
    (subject: StatisticsReportSubject, targetRange: StatisticsDateRange) =>
      apiClient.getNutrientTrendReport(createStatisticsReportRequest(targetRange, subject)),
    [],
  );

  useEffect(() => {
    const requestId = catalogGate.current.begin();
    const requestBaseUrl = apiClient.getBaseUrl();
    reportGate.current.invalidate();
    setReadyProfileUrl(null);
    setProducts([]);
    setNutrients([]);
    setReport(null);
    setSelectedProductId(null);
    setSelectedNutrientKeyState(null);
    selectedNutrientKeyRef.current = null;
    setStatus("loading");
    setError(null);

    async function loadProducts() {
      try {
        const productList = (await apiClient.listProducts()).filter((product) => !product.archived);
        if (!catalogGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl())
          return;

        setProducts(productList);
        setSelectedProductId(productList[0]?.id ?? null);
        setReadyProfileUrl(effectiveApiBaseUrl);
      } catch (loadError) {
        if (!catalogGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl())
          return;
        logger.error("Failed to load statistics products", loadError);
        setStatus("error");
        setError(loadError instanceof Error ? loadError.message : "Unknown error");
      }
    }

    void loadProducts();
    return () => {
      catalogGate.current.invalidate();
      reportGate.current.invalidate();
    };
  }, [effectiveApiBaseUrl, catalogRevision]);

  useEffect(() => {
    if (readyProfileUrl !== effectiveApiBaseUrl) return;
    const requestId = reportGate.current.begin();
    const requestBaseUrl = apiClient.getBaseUrl();
    setStatus("loading");
    setError(null);

    async function loadSelection() {
      try {
        let nextReport: NutrientTrendReport;

        if (source === "product") {
          if (!selectedProduct) {
            setReport(null);
            setStatus("idle");
            return;
          }
          nextReport = await getReport(
            { kind: "product", productId: selectedProduct.id, unit: selectedProduct.serving_unit },
            range,
          );
        } else {
          const discovery = await getReport({ kind: "all" }, range);
          if (!reportGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl())
            return;
          setNutrients(discovery.nutrients);

          const preferred = parseStatisticsNutrientKey(selectedNutrientKeyRef.current);
          const selected =
            discovery.nutrients.find(
              (item) => item.nutrient_id === preferred?.nutrientId && item.unit === preferred?.unit,
            ) ??
            discovery.nutrients[0] ??
            null;

          if (!selected) {
            selectedNutrientKeyRef.current = null;
            setSelectedNutrientKeyState(null);
            nextReport = discovery;
          } else {
            const key = buildStatisticsNutrientKey({
              nutrientId: selected.nutrient_id,
              unit: selected.unit,
            });
            selectedNutrientKeyRef.current = key;
            setSelectedNutrientKeyState(key);
            nextReport = await getReport(
              { kind: "nutrient", nutrientId: selected.nutrient_id, unit: selected.unit },
              range,
            );
          }
        }

        if (!reportGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl())
          return;
        setReport(nextReport);
        setStatus("idle");
        setError(null);
      } catch (loadError) {
        if (!reportGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl())
          return;
        logger.error("Failed to load statistics", loadError);
        setStatus("error");
        setError(loadError instanceof Error ? loadError.message : "Unknown error");
      }
    }

    void loadSelection();
    return () => reportGate.current.invalidate();
  }, [
    readyProfileUrl,
    effectiveApiBaseUrl,
    source,
    range,
    selectedProduct,
    reportRevision,
    getReport,
  ]);

  function changeSelectedNutrientKey(key: string) {
    selectedNutrientKeyRef.current = key;
    setSelectedNutrientKeyState(key);
    setReportRevision((current) => current + 1);
  }

  function refresh() {
    if (readyProfileUrl !== effectiveApiBaseUrl) {
      setCatalogRevision((current) => current + 1);
    } else {
      setReportRevision((current) => current + 1);
    }
  }

  return {
    range,
    source,
    products,
    nutrients,
    selectedProductId,
    selectedProduct,
    selectedNutrientKey,
    selectedNutrient,
    report,
    status,
    error,
    setRange,
    setSource,
    setSelectedProductId,
    setSelectedNutrientKey: changeSelectedNutrientKey,
    refresh,
  };
}
