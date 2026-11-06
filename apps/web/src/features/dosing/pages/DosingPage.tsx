import type { CreateDoseLogInput, UpdateDoseLogInput } from "@doseark/types";
import { Plus, RefreshCw } from "lucide-react";
import { useState } from "react";
import { useTranslation } from "react-i18next";

import { ApiErrorNotice } from "@/app/system_status";
import { Button, PageHeader } from "@/core/ui";

import { DoseLogEditorPanel } from "../components/DoseLogEditorPanel";
import { DoseLogNutrientsPanel } from "../components/DoseLogNutrientsPanel";
import { DoseLogsTable } from "../components/DoseLogsTable";
import { DosingToast } from "../components/DosingToast";
import { useDosingPageData } from "../hooks/useDosingPageData";

/** Keeps dose entry visible beside history, without changing the dose API. */
export function DosingPage() {
  const { t } = useTranslation("dosing");
  const data = useDosingPageData();
  const [formRevision, setFormRevision] = useState(0);
  const [keepValues, setKeepValues] = useState(false);
  const isSaving = data.status === "saving";
  const editingDoseLog = data.editingDoseLog;

  /** Opens a fresh entry and clears any history selection. */
  function newDose() {
    if (isSaving) return;
    data.cancelEditDoseLog();
    data.clearDoseLogSelection();
    setFormRevision((revision) => revision + 1);
  }

  /** Selecting a history row loads that dose into the left editor. */
  function editDose(doseLogId: string) {
    if (isSaving) return;
    data.startEditDoseLog(doseLogId);
  }

  /** Retains entered values only when the rapid-entry option is enabled. */
  async function createDose(input: CreateDoseLogInput) {
    const saved = await data.saveDose(input);
    if (saved && !keepValues) setFormRevision((revision) => revision + 1);
  }

  async function updateDose(id: string, input: UpdateDoseLogInput) {
    await data.saveDoseUpdate(id, input);
  }

  function cancelDoseLog(doseLogId: string) {
    if (isSaving) return;
    if (window.confirm(t("table.deleteConfirm"))) void data.cancelDose(doseLogId);
  }

  return (
    <>
      <DosingToast toast={data.toast} onDismiss={data.dismissToast} />

      <PageHeader
        title={t("page.title")}
        description={t("page.description")}
        action={
          <>
            <Button variant="primary" disabled={isSaving} onClick={newDose}>
              <Plus size={16} />
              {t("page.addDose")}
            </Button>
            <Button onClick={() => void data.loadAll()} disabled={data.status === "loading" || isSaving}>
              <RefreshCw size={15} />
              {t("page.reload")}
            </Button>
          </>
        }
      />

      <div className="mt-page">
        {data.error ? (
          <ApiErrorNotice error={data.error} onRetry={() => void data.loadAll()} />
        ) : null}

        <div className="mt-grid mt-dose-workspace">
          <DoseLogEditorPanel
            formRevision={formRevision}
            keepValues={keepValues}
            onKeepValuesChange={setKeepValues}
            lots={data.lots}
            products={data.products}
            productById={data.productById}
            productNameById={data.productNameById}
            editingDoseLog={editingDoseLog}
            disabled={isSaving}
            onCreate={createDose}
            onUpdate={updateDose}
            onCancelEdit={data.cancelEditDoseLog}
            onClose={newDose}
          />

          <div className="mt-stack mt-dose-workspace__history">
            <DoseLogsTable
              doseLogs={data.filteredDoseLogs}
              totalCount={data.doseLogTotalCount}
              pageSize={data.doseLogPageSize}
              pageIndex={data.doseLogPageIndex}
              products={data.products}
              productNameById={data.productNameById}
              lotNumberById={data.lotNumberById}
              selectedDoseLogId={data.selectedDoseLogId}
              historyProductId={data.historyProductId}
              onHistoryProductChange={data.setHistoryProductId}
              onPageSizeChange={data.setDoseLogPageSize}
              onPageIndexChange={data.setDoseLogPageIndex}
              onSelectDoseLog={editDose}
              onEditDoseLog={editDose}
              onCancelDoseLog={cancelDoseLog}
            />

            {data.selectedDoseLogId ? (
              <DoseLogNutrientsPanel
                selectedDoseLogId={data.selectedDoseLogId}
                rows={data.selectedDoseNutrients}
                loading={data.selectedDoseNutrientsStatus === "loading"}
              />
            ) : null}
          </div>
        </div>
      </div>
    </>
  );
}
