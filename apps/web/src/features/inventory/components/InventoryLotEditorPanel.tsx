import type {
  CreateInventoryLotInput,
  InventoryLot,
  Product,
  StorageLocation,
  UpdateInventoryLotInput,
} from "@doseark/types";

import { Button, Panel } from "@/core/ui";

import { InventoryLotDetailsPanel } from "./InventoryLotDetailsPanel";
import { InventoryLotForm } from "./InventoryLotForm";
import { InventoryLotStockMovementsPanel } from "./InventoryLotStockMovementsPanel";
import { InventoryWorkspaceTabs, type InventoryWorkspaceTab } from "./InventoryWorkspaceTabs";

type Props = {
  mode: "create" | "edit";
  formRevision: number;
  lot?: InventoryLot;
  products: Product[];
  productNameById: Map<string, string>;
  storageLocations: StorageLocation[];
  storageLocation?: StorageLocation;
  productName?: string;
  activeTab: InventoryWorkspaceTab;
  saving?: boolean;
  onNew: () => void;
  onSaved: () => void;
  onTabChange: (tab: InventoryWorkspaceTab) => void;
  onCreate: (input: CreateInventoryLotInput) => boolean | Promise<boolean>;
  onUpdate: (id: string, input: UpdateInventoryLotInput) => boolean | Promise<boolean>;
};

/** Shows lot fields, details, and stock movements beside the lot table. */
export function InventoryLotEditorPanel({
  mode,
  formRevision,
  lot,
  products,
  productNameById,
  storageLocations,
  storageLocation,
  productName,
  activeTab,
  saving,
  onNew,
  onSaved,
  onTabChange,
  onCreate,
  onUpdate,
}: Props) {
  const editing = mode === "edit" && lot !== undefined;

  return (
    <Panel
      className="mt-inventory-editor"
      title={editing ? "Edit inventory lot" : "Create inventory lot"}
      action={<Button disabled={saving} onClick={onNew}>New lot</Button>}
    >
      <div className="mt-inventory-editor__tabs">
        <InventoryWorkspaceTabs
          activeTab={activeTab}
          hasSelectedLot={editing}
          onChange={onTabChange}
        />
      </div>

      {activeTab === "lot" ? (
        <InventoryLotForm
          key={editing ? `edit-${lot.id}` : `create-${formRevision}`}
          mode={editing ? "edit" : "create"}
          lot={editing ? lot : undefined}
          products={products}
          productNameById={productNameById}
          storageLocations={storageLocations}
          disabled={saving}
          onCancel={onNew}
          onSaved={onSaved}
          onCreate={onCreate}
          onUpdate={onUpdate}
        />
      ) : null}

      {activeTab === "info" && editing ? (
        <InventoryLotDetailsPanel lot={lot} productName={productName} storageLocation={storageLocation} />
      ) : null}

      {activeTab === "movements" && editing ? (
        <InventoryLotStockMovementsPanel lotId={lot.id} />
      ) : null}
    </Panel>
  );
}
