import type { Brand, CreateProductInput, Product, UpdateProductInput } from "@doseark/types";

import { Drawer } from "@/core/ui";

import { ProductCompositionPanel } from "./ProductCompositionPanel";
import { ProductDetailsPanel } from "./ProductDetailsPanel";
import { ProductForm } from "./ProductForm";
import { ProductWorkspaceTabs, type ProductWorkspaceTab } from "./ProductWorkspaceTabs";

type Props = {
  open: boolean;
  mode: "create" | "edit";
  product?: Product;
  brands: Brand[];
  brandName?: string;
  activeTab: ProductWorkspaceTab;
  compositionCount?: number | null;
  saving?: boolean;
  error?: string | null;
  onClose: () => void;
  onTabChange: (tab: ProductWorkspaceTab) => void;
  onCompositionCountChange: (count: number) => void;
  onCreate: (input: CreateProductInput) => boolean | Promise<boolean>;
  onUpdate: (id: string, input: UpdateProductInput) => boolean | Promise<boolean>;
};

export function ProductDrawer({
  open,
  mode,
  product,
  brands,
  brandName,
  activeTab,
  compositionCount,
  saving,
  error,
  onClose,
  onTabChange,
  onCompositionCountChange,
  onCreate,
  onUpdate,
}: Props) {
  const isEditMode = mode === "edit" && product !== undefined;

  return (
    <Drawer
      open={open}
      title={isEditMode ? "Edit product" : "New product"}
      description={
        isEditMode
          ? "Edit the product definition, composition and metadata."
          : "Create a product definition. Composition can be added after the product exists."
      }
      tabs={
        <ProductWorkspaceTabs
          activeTab={activeTab}
          hasSelectedProduct={Boolean(product)}
          compositionCount={compositionCount}
          onChange={onTabChange}
        />
      }
      className="mt-product-drawer"
      closeLabel="Close product editor"
      closeDisabled={saving}
      onClose={onClose}
    >
      {error ? <div className="mt-drawer__error" role="alert">{error}</div> : null}

      {activeTab === "product" ? (
        <ProductForm
          mode={mode}
          product={product}
          brands={brands}
          disabled={saving}
          onCancel={onClose}
          onSaved={onClose}
          onCreate={onCreate}
          onUpdate={onUpdate}
        />
      ) : null}

      {activeTab === "composition" ? (
        <ProductCompositionPanel
          productId={product?.id}
          onRowsCountChange={onCompositionCountChange}
        />
      ) : null}

      {activeTab === "details" ? (
        <ProductDetailsPanel product={product} brandName={brandName} />
      ) : null}
    </Drawer>
  );
}
