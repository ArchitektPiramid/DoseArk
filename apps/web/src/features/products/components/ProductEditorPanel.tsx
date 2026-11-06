import type { Brand, CreateProductInput, Product, UpdateProductInput } from "@doseark/types";

import { Panel } from "@/core/ui";

import { ProductCompositionPanel } from "./ProductCompositionPanel";
import { ProductDetailsPanel } from "./ProductDetailsPanel";
import { ProductForm } from "./ProductForm";
import { ProductWorkspaceTabs, type ProductWorkspaceTab } from "./ProductWorkspaceTabs";

type Props = {
  mode: "create" | "edit";
  formRevision: number;
  product?: Product;
  brands: Brand[];
  brandName?: string;
  activeTab: ProductWorkspaceTab;
  compositionCount?: number | null;
  saving?: boolean;
  onNew: () => void;
  onSaved: () => void;
  onTabChange: (tab: ProductWorkspaceTab) => void;
  onCompositionCountChange: (count: number) => void;
  onCreate: (input: CreateProductInput) => boolean | Promise<boolean>;
  onUpdate: (id: string, input: UpdateProductInput) => boolean | Promise<boolean>;
};

/** Renders product fields, composition, and details inline beside the product list. */
export function ProductEditorPanel({
  mode,
  formRevision,
  product,
  brands,
  brandName,
  activeTab,
  compositionCount,
  saving,
  onNew,
  onSaved,
  onTabChange,
  onCompositionCountChange,
  onCreate,
  onUpdate,
}: Props) {
  const editing = mode === "edit" && product !== undefined;

  return (
    <Panel
      className="mt-product-editor"
      title={editing ? "Edit product" : "Create product"}
    >
      <div className="mt-product-editor__tabs">
        <ProductWorkspaceTabs
          activeTab={activeTab}
          hasSelectedProduct={editing}
          compositionCount={compositionCount}
          onChange={onTabChange}
        />
      </div>

      {activeTab === "product" ? (
        <ProductForm
          key={editing ? `edit-${product.id}` : `create-${formRevision}`}
          mode={editing ? "edit" : "create"}
          product={editing ? product : undefined}
          brands={brands}
          disabled={saving}
          onCancel={onNew}
          onSaved={onSaved}
          onCreate={onCreate}
          onUpdate={onUpdate}
        />
      ) : null}

      {activeTab === "composition" && editing ? (
        <ProductCompositionPanel
          key={product.id}
          productId={product.id}
          onRowsCountChange={onCompositionCountChange}
        />
      ) : null}

      {activeTab === "details" && editing ? (
        <ProductDetailsPanel product={product} brandName={brandName} />
      ) : null}
    </Panel>
  );
}
