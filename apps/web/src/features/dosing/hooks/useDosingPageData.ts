import { useCallback } from "react";

import { useDoseLogMutations } from "./useDoseLogMutations";
import { useDoseLogSelection } from "./useDoseLogSelection";
import { useDosingData } from "./useDosingData";
import { useDosingToast } from "./useDosingToast";

type Status = "idle" | "loading" | "saving" | "error";

export function useDosingPageData() {
  const data = useDosingData();
  const selection = useDoseLogSelection(data.doseLogs);
  const notifications = useDosingToast();

  const mutations = useDoseLogMutations({
    reload: data.loadAll,
    cancelEditing: selection.cancelEditDoseLog,
    clearDoseLogState: selection.clearDoseLogState,
    showToast: notifications.showToast,
  });

  const loadAll = useCallback(async () => {
    mutations.clearError();
    await data.loadAll();
  }, [data.loadAll, mutations.clearError]);

  const error = mutations.error ?? data.error;
  const status: Status = mutations.isSaving
    ? "saving"
    : data.isLoading
      ? "loading"
      : error
        ? "error"
        : "idle";

  return {
    doseLogs: data.doseLogs,
    filteredDoseLogs: data.doseLogs,
    doseLogTotalCount: data.doseLogTotalCount,
    doseLogPageSize: data.doseLogPageSize,
    doseLogPageIndex: data.doseLogPageIndex,
    setDoseLogPageSize: data.setDoseLogPageSize,
    setDoseLogPageIndex: data.setDoseLogPageIndex,

    lots: data.lots,
    products: data.products,
    status,
    error,
    toast: notifications.toast,
    dismissToast: notifications.dismissToast,

    historyProductId: data.historyProductId,
    setHistoryProductId: data.setHistoryProductId,

    editingDoseLog: selection.editingDoseLog,
    editingDoseLogId: selection.editingDoseLogId,
    startEditDoseLog: selection.startEditDoseLog,
    cancelEditDoseLog: selection.cancelEditDoseLog,

    selectedDoseLogId: selection.selectedDoseLogId,
    selectedDoseNutrients: selection.selectedDoseNutrients,
    selectedDoseNutrientsStatus: selection.selectedDoseNutrientsStatus,

    productById: data.productById,
    productNameById: data.productNameById,
    lotNumberById: data.lotNumberById,

    loadAll,
    saveDose: mutations.saveDose,
    saveDoseBatch: mutations.saveDoseBatch,
    saveDoseUpdate: mutations.saveDoseUpdate,
    cancelDose: mutations.cancelDose,
    selectDoseLog: selection.selectDoseLog,
    clearDoseLogSelection: selection.clearSelection,
  };
}
