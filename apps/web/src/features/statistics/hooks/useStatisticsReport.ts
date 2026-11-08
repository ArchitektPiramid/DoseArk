import type { NutrientTrendOption, NutrientTrendReport, Product } from "@doseark/types";
import { useEffect, useRef, useState } from "react";

import { apiClient } from "@/app/client";
import { createRequestGate } from "@/core/async/requestGate";
import { createLogger } from "@/core/logger/logger";
import {
  createStatisticsReportRequest,
  type StatisticsReportSubject,
} from "../utils/statisticsReportRequest";
import type { StatisticsDateRange } from "../utils/statisticsRange";
import { resolveStatisticsNutrient } from "../utils/statisticsSelection";
import type { StatisticsSource } from "./useStatistics";

const logger = createLogger("statistics");
type Status = "idle" | "loading" | "error";

type Input = {
  enabled: boolean;
  profileUrl: string;
  range: StatisticsDateRange;
  source: StatisticsSource;
  selectedProduct: Product | null;
};

function getReport(subject: StatisticsReportSubject, range: StatisticsDateRange) {
  return apiClient.getNutrientTrendReport(createStatisticsReportRequest(range, subject));
}

/** Owns report discovery, selected nutrient and stale-response protection. */
export function useStatisticsReport({
  enabled,
  profileUrl,
  range,
  source,
  selectedProduct,
}: Input) {
  const [nutrients, setNutrients] = useState<NutrientTrendOption[]>([]);
  const [selectedNutrientKey, setSelectedNutrientKeyState] = useState<string | null>(null);
  const [report, setReport] = useState<NutrientTrendReport | null>(null);
  const [status, setStatus] = useState<Status>("loading");
  const [error, setError] = useState<string | null>(null);
  const [revision, setRevision] = useState(0);
  const selectedKeyRef = useRef<string | null>(null);
  const gate = useRef(createRequestGate());

  useEffect(() => {
    gate.current.invalidate();
    selectedKeyRef.current = null;
    setSelectedNutrientKeyState(null);
    setNutrients([]);
    setReport(null);
  }, [profileUrl]);

  useEffect(() => {
    if (!enabled) {
      gate.current.invalidate();
      setReport(null);
      return;
    }

    const requestId = gate.current.begin();
    const requestBaseUrl = apiClient.getBaseUrl();
    const isCurrent = () =>
      gate.current.isCurrent(requestId) && requestBaseUrl === apiClient.getBaseUrl();
    setStatus("loading");
    setError(null);

    async function load() {
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
          if (!isCurrent()) return;
          setNutrients(discovery.nutrients);

          const selection = resolveStatisticsNutrient(discovery.nutrients, selectedKeyRef.current);
          selectedKeyRef.current = selection.key;
          setSelectedNutrientKeyState(selection.key);
          nextReport = selection.nutrient
            ? await getReport(
                {
                  kind: "nutrient",
                  nutrientId: selection.nutrient.nutrient_id,
                  unit: selection.nutrient.unit,
                },
                range,
              )
            : discovery;
        }

        if (!isCurrent()) return;
        setReport(nextReport);
        setStatus("idle");
        setError(null);
      } catch (loadError) {
        if (!isCurrent()) return;
        logger.error("Failed to load statistics", loadError);
        setStatus("error");
        setError(loadError instanceof Error ? loadError.message : "Unknown error");
      }
    }

    void load();
    return () => gate.current.invalidate();
  }, [enabled, profileUrl, source, range, selectedProduct, revision]);

  function changeSelectedNutrientKey(key: string) {
    selectedKeyRef.current = key;
    setSelectedNutrientKeyState(key);
    setRevision((current) => current + 1);
  }

  return {
    nutrients,
    selectedNutrientKey,
    report,
    status,
    error,
    setSelectedNutrientKey: changeSelectedNutrientKey,
    refresh: () => setRevision((current) => current + 1),
  };
}
