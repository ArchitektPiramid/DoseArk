import { useMemo, useState } from "react";

import { ApiErrorNotice } from "@/app/system_status";
import { Button, PageHeader } from "@/core/ui";

import { InventoryLotEditorPanel } from "../components/InventoryLotEditorPanel";
import { InventoryLotsTable } from "../components/InventoryLotsTable";
import type { InventoryWorkspaceTab } from "../components/InventoryWorkspaceTabs";
import { useInventoryPageData } from "../hooks/useInventoryPageData";

type FormMode = "create" | "edit";

/** Keeps the inventory editor and lot list visible in the same workspace. */
export function InventoryPage() {
  const data = useInventoryPageData();
  const [mode, setMode] = useState<FormMode>("create");
  const [formRevision, setFormRevision] = useState(0);
  const [activeWorkspaceTab, setActiveWorkspaceTab] = useState<InventoryWorkspaceTab>("lot");

  const isLoading = data.status === "loading";
  const isSaving = data.status === "saving";
  const selectedLot = mode === "edit" ? data.selectedLot : undefined;

  const selectedProductName = selectedLot
    ? data.productNameById.get(selectedLot.product_id)
    : undefined;

  const selectedStorageLocation = useMemo(() => {
    if (!selectedLot?.storage_location_id) return undefined;
    return data.storageLocationById.get(selectedLot.storage_location_id);
  }, [selectedLot, data.storageLocationById]);

  /** Selects a lot for editing without hiding the inventory list. */
  function selectLot(id: string) {
    if (isSaving) return;
    data.setSelectedLotId(id);
    setMode("edit");
    setActiveWorkspaceTab("lot");
  }

  /** Clears the editor, including when it is already showing a new lot. */
  function newLot() {
    if (isSaving) return;
    data.setSelectedLotId(null);
    setMode("create");
    setActiveWorkspaceTab("lot");
    setFormRevision((revision) => revision + 1);
  }

  /** Keeps a newly saved lot selected so its details and movements can be viewed. */
  function lotSaved() {
    setMode("edit");
    setActiveWorkspaceTab("lot");
  }

  return (
    <div className="mt-page">
      <PageHeader
        title="Inventory"
        description="Manage purchased packages, locations and remaining stock."
        action={
          <>
            <Button variant="primary" disabled={isSaving} onClick={newLot}>New lot</Button>
            <Button disabled={isSaving} onClick={() => void data.loadAll()}>Reload</Button>
          </>
        }
      />

      {data.error ? (
        <ApiErrorNotice error={data.error} onRetry={() => void data.loadAll()} />
      ) : null}

      <div className="mt-grid mt-inventory-workspace">
        <InventoryLotEditorPanel
          mode={mode}
          formRevision={formRevision}
          lot={selectedLot}
          products={data.products}
          productNameById={data.productNameById}
          storageLocations={data.storageLocations}
          productName={selectedProductName}
          storageLocation={selectedStorageLocation}
          activeTab={activeWorkspaceTab}
          saving={isSaving}
          onNew={newLot}
          onSaved={lotSaved}
          onTabChange={setActiveWorkspaceTab}
          onCreate={data.createInventoryLot}
          onUpdate={data.updateInventoryLot}
        />

        <InventoryLotsTable
          lots={data.lots}
          selectedLotId={selectedLot?.id ?? null}
          loading={isLoading}
          productNameById={data.productNameById}
          storageLocationById={data.storageLocationById}
          onSelect={selectLot}
        />
      </div>
    </div>
  );
}
