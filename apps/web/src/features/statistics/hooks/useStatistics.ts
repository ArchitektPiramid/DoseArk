import type { NutrientTrendOption, NutrientTrendReport, Product } from "@doseark/types";
import { useEffect, useMemo, useRef, useState } from "react";

import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";
import {
  getStatisticsPresetRange,
  type StatisticsDateRange,
} from "../utils/statisticsRange";
import {
  buildStatisticsNutrientKey,
  parseStatisticsNutrientKey,
} from "../utils/statisticsSelection";

export type StatisticsSource = "product" | "nutrient";
type Status = "idle" | "loading" | "error";

const logger = createLogger("statistics");

export function useStatistics() {
  const [range, setRangeState] = useState<StatisticsDateRange>(() => getStatisticsPresetRange(30));
  const [source, setSourceState] = useState<StatisticsSource>("product");
  const [products, setProducts] = useState<Product[]>([]);
  const [nutrients, setNutrients] = useState<NutrientTrendOption[]>([]);
  const [selectedProductId, setSelectedProductIdState] = useState<string | null>(null);
  const [selectedNutrientKey, setSelectedNutrientKeyState] = useState<string | null>(null);
  const [report, setReport] = useState<NutrientTrendReport | null>(null);
  const [status, setStatus] = useState<Status>("idle");
  const [error, setError] = useState<string | null>(null);
  const requestIdRef = useRef(0);

  const selectedProduct = useMemo(
    () => products.find((product) => product.id === selectedProductId) ?? null,
    [products, selectedProductId],
  );

  const selectedNutrient = useMemo(
    () => parseStatisticsNutrientKey(selectedNutrientKey),
    [selectedNutrientKey],
  );

  async function loadProduct(productId: string, nextRange: StatisticsDateRange = range) {
    const product = products.find((item) => item.id === productId);
    if (!product) {
      setReport(null);
      return;
    }

    const requestId = ++requestIdRef.current;
    beginLoad();

    try {
      const nextReport = await apiClient.getNutrientTrendReport({
        startDate: nextRange.startDate,
        endDate: nextRange.endDate,
        timezoneOffsetMinutes: new Date().getTimezoneOffset(),
        productId,
        nutrientId: null,
        unit: product.serving_unit,
        dayStartHour: 0,
      });
      if (requestId !== requestIdRef.current) return;
      setReport(nextReport);
      finishLoad();
    } catch (loadError) {
      if (requestId !== requestIdRef.current) return;
      failLoad(loadError);
    }
  }

  async function loadNutrient(
    key: string,
    nextRange: StatisticsDateRange = range,
    knownNutrients: NutrientTrendOption[] = nutrients,
  ) {
    const selection = parseStatisticsNutrientKey(key);
    if (!selection) {
      setReport(null);
      return;
    }

    const option = knownNutrients.find(
      (item) => item.nutrient_id === selection.nutrientId && item.unit === selection.unit,
    );
    if (!option) {
      await discoverNutrients(nextRange, key);
      return;
    }

    const requestId = ++requestIdRef.current;
    beginLoad();

    try {
      const nextReport = await apiClient.getNutrientTrendReport({
        startDate: nextRange.startDate,
        endDate: nextRange.endDate,
        timezoneOffsetMinutes: new Date().getTimezoneOffset(),
        nutrientId: selection.nutrientId,
        productId: null,
        unit: selection.unit,
        dayStartHour: 0,
      });
      if (requestId !== requestIdRef.current) return;
      setReport(nextReport);
      finishLoad();
    } catch (loadError) {
      if (requestId !== requestIdRef.current) return;
      failLoad(loadError);
    }
  }

  async function discoverNutrients(nextRange: StatisticsDateRange = range, preferredKey?: string | null) {
    const requestId = ++requestIdRef.current;
    beginLoad();

    try {
      const discovery = await apiClient.getNutrientTrendReport({
        startDate: nextRange.startDate,
        endDate: nextRange.endDate,
        timezoneOffsetMinutes: new Date().getTimezoneOffset(),
        nutrientId: null,
        productId: null,
        unit: null,
        dayStartHour: 0,
      });
      if (requestId !== requestIdRef.current) return;

      setNutrients(discovery.nutrients);
      const preferred = preferredKey ? parseStatisticsNutrientKey(preferredKey) : null;
      const match = preferred
        ? discovery.nutrients.find(
            (item) => item.nutrient_id === preferred.nutrientId && item.unit === preferred.unit,
          )
        : null;
      const selected = match ?? discovery.nutrients[0] ?? null;

      if (!selected) {
        setSelectedNutrientKeyState(null);
        setReport(discovery);
        finishLoad();
        return;
      }

      const key = buildStatisticsNutrientKey({
        nutrientId: selected.nutrient_id,
        unit: selected.unit,
      });
      setSelectedNutrientKeyState(key);

      const detail = await apiClient.getNutrientTrendReport({
        startDate: nextRange.startDate,
        endDate: nextRange.endDate,
        timezoneOffsetMinutes: new Date().getTimezoneOffset(),
        nutrientId: selected.nutrient_id,
        productId: null,
        unit: selected.unit,
        dayStartHour: 0,
      });
      if (requestId !== requestIdRef.current) return;
      setReport(detail);
      finishLoad();
    } catch (loadError) {
      if (requestId !== requestIdRef.current) return;
      failLoad(loadError);
    }
  }

  function setRange(nextRange: StatisticsDateRange) {
    setRangeState(nextRange);
    if (source === "product" && selectedProductId) {
      void loadProduct(selectedProductId, nextRange);
      return;
    }
    void discoverNutrients(nextRange, selectedNutrientKey);
  }

  function setSource(nextSource: StatisticsSource) {
    if (nextSource === source) return;
    setSourceState(nextSource);

    if (nextSource === "product") {
      const productId = selectedProductId ?? products[0]?.id ?? null;
      setSelectedProductIdState(productId);
      if (productId) void loadProduct(productId);
      else setReport(null);
      return;
    }

    void discoverNutrients(range, selectedNutrientKey);
  }

  function setSelectedProductId(productId: string) {
    setSelectedProductIdState(productId);
    void loadProduct(productId);
  }

  function setSelectedNutrientKey(key: string) {
    setSelectedNutrientKeyState(key);
    void loadNutrient(key);
  }

  function refresh() {
    if (source === "product" && selectedProductId) {
      void loadProduct(selectedProductId);
      return;
    }
    void discoverNutrients(range, selectedNutrientKey);
  }

  function beginLoad() {
    setStatus("loading");
    setError(null);
  }

  function finishLoad() {
    setStatus("idle");
    setError(null);
  }

  function failLoad(loadError: unknown) {
    logger.error("Failed to load statistics", loadError);
    setStatus("error");
    setError(loadError instanceof Error ? loadError.message : "Unknown error");
  }

  useEffect(() => {
    let cancelled = false;

    async function start() {
      beginLoad();
      try {
        const productList = (await apiClient.listProducts()).filter((product) => !product.archived);
        if (cancelled) return;
        setProducts(productList);
        const firstProduct = productList[0] ?? null;
        setSelectedProductIdState(firstProduct?.id ?? null);

        if (!firstProduct) {
          setReport(null);
          finishLoad();
          return;
        }

        const initialReport = await apiClient.getNutrientTrendReport({
          startDate: range.startDate,
          endDate: range.endDate,
          timezoneOffsetMinutes: new Date().getTimezoneOffset(),
          productId: firstProduct.id,
          nutrientId: null,
          unit: firstProduct.serving_unit,
          dayStartHour: 0,
        });
        if (cancelled) return;
        setReport(initialReport);
        finishLoad();
      } catch (loadError) {
        if (cancelled) return;
        failLoad(loadError);
      }
    }

    void start();
    return () => {
      cancelled = true;
      requestIdRef.current += 1;
    };
  }, []);

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
    setSelectedNutrientKey,
    refresh,
  };
}
