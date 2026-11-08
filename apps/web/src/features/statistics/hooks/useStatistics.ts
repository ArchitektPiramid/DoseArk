import { useEffect, useMemo, useState } from "react";

import { useApiProfiles } from "@/app/api_profiles";
import { getStatisticsPresetRange, type StatisticsDateRange } from "../utils/statisticsRange";
import { parseStatisticsNutrientKey } from "../utils/statisticsSelection";
import { useStatisticsCatalog } from "./useStatisticsCatalog";
import { useStatisticsReport } from "./useStatisticsReport";

export type StatisticsSource = "product" | "nutrient";

/** Composes catalog, user selection and report state without coupling their request lifecycles. */
export function useStatistics() {
  const { effectiveApiBaseUrl } = useApiProfiles();
  const [range, setRange] = useState<StatisticsDateRange>(() => getStatisticsPresetRange(30));
  const [source, setSource] = useState<StatisticsSource>("product");
  const [selectedProductId, setSelectedProductId] = useState<string | null>(null);
  const catalog = useStatisticsCatalog(effectiveApiBaseUrl);

  useEffect(() => {
    setSelectedProductId(null);
  }, [effectiveApiBaseUrl]);

  useEffect(() => {
    if (!catalog.ready) return;
    setSelectedProductId((current) =>
      catalog.products.some((product) => product.id === current)
        ? current
        : (catalog.products[0]?.id ?? null),
    );
  }, [catalog.ready, catalog.products]);

  const selectedProduct = useMemo(
    () => catalog.products.find((product) => product.id === selectedProductId) ?? null,
    [catalog.products, selectedProductId],
  );
  const selection = useStatisticsReport({
    enabled: catalog.ready,
    profileUrl: effectiveApiBaseUrl,
    range,
    source,
    selectedProduct,
  });
  const selectedNutrient = useMemo(
    () => parseStatisticsNutrientKey(selection.selectedNutrientKey),
    [selection.selectedNutrientKey],
  );

  return {
    range,
    source,
    products: catalog.products,
    nutrients: selection.nutrients,
    selectedProductId,
    selectedProduct,
    selectedNutrientKey: selection.selectedNutrientKey,
    selectedNutrient,
    report: catalog.ready ? selection.report : null,
    status: catalog.ready ? selection.status : catalog.status,
    error: catalog.ready ? selection.error : catalog.error,
    setRange,
    setSource,
    setSelectedProductId,
    setSelectedNutrientKey: selection.setSelectedNutrientKey,
    refresh: () => (catalog.ready ? selection.refresh() : catalog.refresh()),
  };
}
