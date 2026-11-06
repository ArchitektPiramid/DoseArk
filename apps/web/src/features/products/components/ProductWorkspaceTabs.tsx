export type ProductWorkspaceTab = "product" | "composition" | "details";

type Props = {
  activeTab: ProductWorkspaceTab;
  hasSelectedProduct: boolean;
  compositionCount?: number | null;
  onChange: (tab: ProductWorkspaceTab) => void;
};

type TabDefinition = {
  id: ProductWorkspaceTab;
  label: string;
  requiresProduct?: boolean;
};

export function ProductWorkspaceTabs({
  activeTab,
  hasSelectedProduct,
  compositionCount,
  onChange,
}: Props) {
  const compositionLabel =
    compositionCount !== null && compositionCount !== undefined
      ? `Composition (${compositionCount})`
      : "Composition";

  const tabs: TabDefinition[] = [
    { id: "product", label: "Product" },
    { id: "composition", label: compositionLabel, requiresProduct: true },
    { id: "details", label: "Info", requiresProduct: true },
  ];

  return (
    <div
      className="mt-workspace-tabs"
      role="tablist"
      aria-label="Product editor sections"
    >
      {tabs.map((tab) => {
        const disabled = Boolean(tab.requiresProduct && !hasSelectedProduct);
        const isActive = tab.id === activeTab;

        return (
          <button
            key={tab.id}
            type="button"
            role="tab"
            aria-selected={isActive}
            disabled={disabled}
            className={
              isActive ? "mt-workspace-tab is-active" : "mt-workspace-tab"
            }
            onClick={() => onChange(tab.id)}
          >
            <span className="mt-workspace-tab__label">{tab.label}</span>
          </button>
        );
      })}
    </div>
  );
}
