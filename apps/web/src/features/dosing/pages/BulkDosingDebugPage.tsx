import type { CreateDoseLogInput, UpdateDoseLogInput } from "@doseark/types";
import { RefreshCw } from "lucide-react";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Navigate } from "react-router";

import { useDebugMode } from "@/app/debug/DebugModeProvider";
import { ApiErrorNotice } from "@/app/system_status";
import { Button, PageHeader } from "@/core/ui";

import { BulkDoseEntryPanel } from "../components/BulkDoseEntryPanel";
import { BulkDoseHistoryTable } from "../components/BulkDoseHistoryTable";
import { DoseLogDrawer } from "../components/DoseLogDrawer";
import { DoseLogNutrientsPanel } from "../components/DoseLogNutrientsPanel";
import { DosingToast } from "../components/DosingToast";
import { useDosingPageData } from "../hooks/useDosingPageData";

export function BulkDosingDebugPage() {
  const debug = useDebugMode();
  if (!debug.enabled) return <Navigate to="/dosing" replace />;
  return <BulkDosingDebugContent />;
}

function BulkDosingDebugContent() {
  const { t } = useTranslation("dosing");
  const data = useDosingPageData();
  const [isEditDrawerOpen, setEditDrawerOpen] = useState(false);
  const isSaving = data.status === "saving";

  function openEditDrawer(doseLogId: string) {
    data.startEditDoseLog(doseLogId);
    setEditDrawerOpen(true);
  }

  function closeEditDrawer() {
    data.cancelEditDoseLog();
    setEditDrawerOpen(false);
  }

  async function updateDose(id: string, input: UpdateDoseLogInput) {
    const saved = await data.saveDoseUpdate(id, input);
    if (saved) setEditDrawerOpen(false);
  }

  async function createDose(input: CreateDoseLogInput) {
    await data.saveDose(input);
  }

  function cancelDoseLog(doseLogId: string) {
    const confirmed = window.confirm(t("table.deleteConfirm"));
    if (confirmed) void data.cancelDose(doseLogId);
  }

  return (
    <>
      <DosingToast toast={data.toast} onDismiss={data.dismissToast} />

      <PageHeader
        title={t("bulk.pageTitle")}
        description={t("bulk.pageDescription")}
        action={
          <Button onClick={() => void data.loadAll()} disabled={data.status === "loading"}>
            <RefreshCw size={15} />
            {t("page.reload")}
          </Button>
        }
      />

      <div className="mt-page">
        {data.error ? <ApiErrorNotice error={data.error} onRetry={() => void data.loadAll()} /> : null}

        <div className="mt-bulk-dosing-layout">
          <div className="mt-stack">
            <BulkDoseHistoryTable
              doseLogs={data.filteredDoseLogs}
              totalCount={data.doseLogTotalCount}
              pageSize={data.doseLogPageSize}
              pageIndex={data.doseLogPageIndex}
              products={data.products}
              productNameById={data.productNameById}
              selectedDoseLogId={data.selectedDoseLogId}
              historyProductId={data.historyProductId}
              onHistoryProductChange={data.setHistoryProductId}
              onPageSizeChange={data.setDoseLogPageSize}
              onPageIndexChange={data.setDoseLogPageIndex}
              onSelectDoseLog={(doseLogId) => void data.selectDoseLog(doseLogId)}
              onEditDoseLog={openEditDrawer}
              onCancelDoseLog={cancelDoseLog}
            />

            <DoseLogNutrientsPanel
              selectedDoseLogId={data.selectedDoseLogId}
              rows={data.selectedDoseNutrients}
              loading={data.selectedDoseNutrientsStatus === "loading"}
            />
          </div>

          <BulkDoseEntryPanel
            lots={data.lots}
            products={data.products}
            productById={data.productById}
            productNameById={data.productNameById}
            disabled={isSaving}
            onSave={data.saveDoseBatch}
          />
        </div>
      </div>

      <DoseLogDrawer
        open={isEditDrawerOpen}
        lots={data.lots}
        products={data.products}
        productById={data.productById}
        productNameById={data.productNameById}
        editingDoseLog={data.editingDoseLog}
        disabled={isSaving}
        onCreate={createDose}
        onUpdate={updateDose}
        onCancelEdit={data.cancelEditDoseLog}
        onClose={closeEditDrawer}
      />
    </>
  );
}
