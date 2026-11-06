import type { DoseLog, DoseLogNutrient } from "@doseark/types";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";

import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("dosing");

type NutrientStatus = "idle" | "loading" | "error";

export function useDoseLogSelection(doseLogs: DoseLog[]) {
  const [editingDoseLogId, setEditingDoseLogId] = useState<string | null>(null);
  const [selectedDoseLogId, setSelectedDoseLogId] = useState<string | null>(null);
  const [selectedDoseNutrients, setSelectedDoseNutrients] = useState<DoseLogNutrient[]>([]);
  const [selectedDoseNutrientsStatus, setSelectedDoseNutrientsStatus] =
    useState<NutrientStatus>("idle");
  const selectionRequestId = useRef(0);

  const editingDoseLog = useMemo(() => {
    if (!editingDoseLogId) {
      return undefined;
    }

    return doseLogs.find((log) => log.id === editingDoseLogId);
  }, [doseLogs, editingDoseLogId]);

  const clearSelection = useCallback(() => {
    selectionRequestId.current += 1;
    setSelectedDoseLogId(null);
    setSelectedDoseNutrients([]);
    setSelectedDoseNutrientsStatus("idle");
  }, []);

  const selectDoseLog = useCallback(async (doseLogId: string) => {
    const requestId = selectionRequestId.current + 1;
    selectionRequestId.current = requestId;

    try {
      setSelectedDoseLogId(doseLogId);
      setSelectedDoseNutrientsStatus("loading");

      logger.trace("Loading dose nutrient snapshots", { doseLogId });

      const rows = await apiClient.listDoseLogNutrients(doseLogId);
      if (selectionRequestId.current !== requestId) {
        return;
      }

      setSelectedDoseNutrients(rows);
      setSelectedDoseNutrientsStatus("idle");

      logger.debug("Dose nutrient snapshots loaded", {
        doseLogId,
        rows: rows.length,
      });
    } catch (err) {
      if (selectionRequestId.current !== requestId) {
        return;
      }

      logger.error("Failed to load dose nutrient snapshots", err);
      setSelectedDoseNutrients([]);
      setSelectedDoseNutrientsStatus("error");
    }
  }, []);

  const startEditDoseLog = useCallback(
    (doseLogId: string) => {
      setEditingDoseLogId(doseLogId);
      void selectDoseLog(doseLogId);
    },
    [selectDoseLog],
  );

  const cancelEditDoseLog = useCallback(() => {
    setEditingDoseLogId(null);
  }, []);

  const clearDoseLogState = useCallback(
    (doseLogId: string) => {
      setEditingDoseLogId((current) => (current === doseLogId ? null : current));
      if (selectedDoseLogId === doseLogId) {
        clearSelection();
      }
    },
    [clearSelection, selectedDoseLogId],
  );

  useEffect(() => {
    if (editingDoseLogId && !doseLogs.some((row) => row.id === editingDoseLogId)) {
      setEditingDoseLogId(null);
    }

    if (selectedDoseLogId && !doseLogs.some((row) => row.id === selectedDoseLogId)) {
      clearSelection();
    }
  }, [clearSelection, doseLogs, editingDoseLogId, selectedDoseLogId]);

  return {
    editingDoseLog,
    editingDoseLogId,
    startEditDoseLog,
    cancelEditDoseLog,
    clearDoseLogState,

    selectedDoseLogId,
    selectedDoseNutrients,
    selectedDoseNutrientsStatus,
    selectDoseLog,
    clearSelection,
  };
}
