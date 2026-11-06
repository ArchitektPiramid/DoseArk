export type InventoryWorkspaceTab = "lot" | "info" | "movements";

type Props = {
  activeTab: InventoryWorkspaceTab;
  hasSelectedLot: boolean;
  onChange: (tab: InventoryWorkspaceTab) => void;
};

const TABS: Array<{ id: InventoryWorkspaceTab; label: string; requiresLot?: boolean }> = [
  { id: "lot", label: "Lot" },
  { id: "info", label: "Info", requiresLot: true },
  { id: "movements", label: "Stock movements", requiresLot: true },
];

export function InventoryWorkspaceTabs({ activeTab, hasSelectedLot, onChange }: Props) {
  return (
    <div className="mt-workspace-tabs" role="tablist" aria-label="Inventory lot sections">
      {TABS.map((tab) => {
        const disabled = Boolean(tab.requiresLot && !hasSelectedLot);
        return (
          <button
            key={tab.id}
            type="button"
            role="tab"
            aria-selected={tab.id === activeTab}
            disabled={disabled}
            className={tab.id === activeTab ? "mt-workspace-tab is-active" : "mt-workspace-tab"}
            onClick={() => onChange(tab.id)}
          >
            <span className="mt-workspace-tab__label">{tab.label}</span>
          </button>
        );
      })}
    </div>
  );
}
