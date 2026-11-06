import type { NutrientTrendOption, NutrientTrendReport, Product } from "@doseark/types";
import { useEffect, useMemo, useRef, useState } from "react";

import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";
import type { StatisticsDateRange } from "../utils/statisticsRange";
import {
  buildStatisticsNutrientKey,
  parseStatisticsNutrientKey,
} from "../utils/statisticsSelection";
import type { StatisticsDailyPoint } from "../utils/statisticsSeries";
import type { StatisticsSource } from "./useStatistics";

const logger = createLogger("statistics");

type Status = "idle" | "loading" | "error";

export type StatisticsComparisonOption = {
  key: string;
  label: string;
  unit: string;
};

export type StatisticsComparisonSeries = StatisticsComparisonOption & {
  rows: StatisticsDailyPoint[];
};

type UseStatisticsComparisonInput = {
  enabled: boolean;
  range: StatisticsDateRange;
  products: Product[];
};

export function useStatisticsComparison({
  enabled,
  range,
  products,
}: UseStatisticsComparisonInput) {
  const [source, setSource] = useState<StatisticsSource>("product");
  const [nutrients, setNutrients] = useState<NutrientTrendOption[]>([]);
  const [selectedProductIds, setSelectedProductIds] = useState<string[]>([]);
  const [selectedNutrientKeys, setSelectedNutrientKeys] = useState<string[]>([]);
  const [series, setSeries] = useState<StatisticsComparisonSeries[]>([]);
  const [status, setStatus] = useState<Status>("idle");
  const [error, setError] = useState<string | null>(null);
  const discoveryRequestIdRef = useRef(0);
  const seriesRequestIdRef = useRef(0);
  const productInitializedRef = useRef(false);
  const nutrientInitializedRef = useRef(false);

  const productOptions = useMemo<StatisticsComparisonOption[]>(
    () =>
      products.map((product) => ({
        key: product.id,
        label: product.name,
        unit: product.serving_unit,
      })),
    [products],
  );

  const nutrientOptions = useMemo<StatisticsComparisonOption[]>(
    () =>
      nutrients.map((nutrient) => ({
        key: buildStatisticsNutrientKey({
          nutrientId: nutrient.nutrient_id,
          unit: nutrient.unit,
        }),
        label: nutrient.nutrient_display_name,
        unit: nutrient.unit,
      })),
    [nutrients],
  );

  const options = source === "product" ? productOptions : nutrientOptions;
  const selectedKeys = source === "product" ? selectedProductIds : selectedNutrientKeys;
  const selectedOptions = useMemo(
    () => selectedKeys.map((key) => options.find((option) => option.key === key)).filter(isDefined),
    [options, selectedKeys],
  );
  const selectedUnit = selectedOptions[0]?.unit ?? null;

  useEffect(() => {
    if (!enabled || source !== "product" || productInitializedRef.current || productOptions.length === 0) {
      return;
    }
    productInitializedRef.current = true;
    setSelectedProductIds([productOptions[0].key]);
  }, [enabled, productOptions, source]);

  useEffect(() => {
    if (!enabled || source !== "nutrient") return;

    const requestId = ++discoveryRequestIdRef.current;

    async function discover() {
      setStatus("loading");
      setError(null);

      try {
        const report = await apiClient.getNutrientTrendReport({
          startDate: range.startDate,
          endDate: range.endDate,
          timezoneOffsetMinutes: new Date().getTimezoneOffset(),
          nutrientId: null,
          productId: null,
          unit: null,
          dayStartHour: 0,
        });
        if (requestId !== discoveryRequestIdRef.current) return;

        setNutrients(report.nutrients);
        setSelectedNutrientKeys((current) => {
          const availableKeys = new Set(
            report.nutrients.map((nutrient) =>
              buildStatisticsNutrientKey({ nutrientId: nutrient.nutrient_id, unit: nutrient.unit }),
            ),
          );
          const preserved = current.filter((key) => availableKeys.has(key));
          if (preserved.length > 0 || nutrientInitializedRef.current) return preserved;

          nutrientInitializedRef.current = true;
          const first = report.nutrients[0];
          return first
            ? [buildStatisticsNutrientKey({ nutrientId: first.nutrient_id, unit: first.unit })]
            : [];
        });
        setStatus("idle");
      } catch (loadError) {
        if (requestId !== discoveryRequestIdRef.current) return;
        logger.error("Failed to discover comparison nutrients", loadError);
        setStatus("error");
        setError(loadError instanceof Error ? loadError.message : "Unknown error");
      }
    }

    void discover();
    return () => {
      discoveryRequestIdRef.current += 1;
    };
  }, [enabled, range.endDate, range.startDate, source]);

  useEffect(() => {
    if (!enabled) return;
    if (source === "nutrient" && nutrients.length === 0) {
      setSeries([]);
      return;
    }

    const activeOptions = selectedOptions;
    if (activeOptions.length === 0) {
      setSeries([]);
      setStatus("idle");
      return;
    }

    const requestId = ++seriesRequestIdRef.current;

    async function loadSeries() {
      setStatus("loading");
      setError(null);

      try {
        const loaded = await Promise.all(
          activeOptions.map(async (option): Promise<StatisticsComparisonSeries> => {
            const report = await loadOptionReport(source, option, range);
            return {
              ...option,
              rows: report.days.map((day) => ({
                date: day.date,
                amount: day.amount_total,
                doseCount: day.dose_count,
              })),
            };
          }),
        );
        if (requestId !== seriesRequestIdRef.current) return;

        setSeries(loaded);
        setStatus("idle");
      } catch (loadError) {
        if (requestId !== seriesRequestIdRef.current) return;
        logger.error("Failed to load comparison series", loadError);
        setStatus("error");
        setError(loadError instanceof Error ? loadError.message : "Unknown error");
      }
    }

    void loadSeries();
    return () => {
      seriesRequestIdRef.current += 1;
    };
  }, [enabled, range.endDate, range.startDate, selectedOptions, source]);

  function changeSource(nextSource: StatisticsSource) {
    if (nextSource === source) return;
    setSource(nextSource);
    setSeries([]);
    setError(null);
  }

  function toggleSelection(key: string) {
    const option = options.find((item) => item.key === key);
    if (!option) return;

    const setter = source === "product" ? setSelectedProductIds : setSelectedNutrientKeys;
    setter((current) => {
      if (current.includes(key)) {
        return current.filter((item) => item !== key);
      }

      const currentOptions = current
        .map((item) => options.find((candidate) => candidate.key === item))
        .filter(isDefined);
      const activeUnit = currentOptions[0]?.unit ?? option.unit;
      if (option.unit !== activeUnit) return current;

      return [...current, key];
    });
  }

  function clearSelection() {
    if (source === "product") setSelectedProductIds([]);
    else setSelectedNutrientKeys([]);
  }

  return {
    source,
    options,
    selectedKeys,
    selectedOptions,
    selectedUnit,
    series,
    status,
    error,
    changeSource,
    toggleSelection,
    clearSelection,
  };
}

async function loadOptionReport(
  source: StatisticsSource,
  option: StatisticsComparisonOption,
  range: StatisticsDateRange,
): Promise<NutrientTrendReport> {
  if (source === "product") {
    return apiClient.getNutrientTrendReport({
      startDate: range.startDate,
      endDate: range.endDate,
      timezoneOffsetMinutes: new Date().getTimezoneOffset(),
      productId: option.key,
      nutrientId: null,
      unit: option.unit,
      dayStartHour: 0,
    });
  }

  const selection = parseStatisticsNutrientKey(option.key);
  if (!selection) {
    throw new Error("Invalid nutrient selection");
  }

  return apiClient.getNutrientTrendReport({
    startDate: range.startDate,
    endDate: range.endDate,
    timezoneOffsetMinutes: new Date().getTimezoneOffset(),
    productId: null,
    nutrientId: selection.nutrientId,
    unit: selection.unit,
    dayStartHour: 0,
  });
}

function isDefined<T>(value: T | null | undefined): value is T {
  return value !== null && value !== undefined;
}
