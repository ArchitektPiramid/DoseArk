import { useState } from "react";

import { ApiErrorNotice } from "@/app/system_status";
import { Button, PageHeader } from "@/core/ui";

import { ProductEditorPanel } from "../components/ProductEditorPanel";
import { ProductListPanel } from "../components/ProductListPanel";
import type { ProductWorkspaceTab } from "../components/ProductWorkspaceTabs";
import { useProductCompositionCount } from "../hooks/useProductCompositionCount";
import { useProductsPageData } from "../hooks/useProductsPageData";

type FormMode = "create" | "edit";

/** Keeps the product editor and selectable list visible in one workspace. */
export function ProductsPage() {
  const data = useProductsPageData();
  const [mode, setMode] = useState<FormMode>("create");
  const [formRevision, setFormRevision] = useState(0);
  const [activeWorkspaceTab, setActiveWorkspaceTab] =
    useState<ProductWorkspaceTab>("product");

  const selectedProductId = mode === "edit" ? data.selectedProduct?.id : undefined;
  const { compositionCount, setCompositionCount } =
    useProductCompositionCount(selectedProductId);

  const selectedBrandName = data.selectedProduct?.brand_id
    ? data.brandNameById.get(data.selectedProduct.brand_id)
    : undefined;

  const isSaving = data.status === "saving";
  const isLoading = data.status === "loading";

  /** Opens the selected product in the left-hand editor without obscuring the table. */
  function selectProduct(id: string) {
    if (isSaving) return;
    data.setSelectedProductId(id);
    setMode("edit");
    setActiveWorkspaceTab("product");
  }

  /** Resets the editor to a blank product, including when Create is already selected. */
  function newProduct() {
    if (isSaving) return;
    data.setSelectedProductId(null);
    setCompositionCount(null);
    setMode("create");
    setActiveWorkspaceTab("product");
    setFormRevision((revision) => revision + 1);
  }

  /** After creation, the saved product can immediately receive a composition. */
  function productSaved() {
    setMode("edit");
    setActiveWorkspaceTab("product");
  }

  return (
    <div className="mt-page">
      <PageHeader
        title="Products"
        description="Manage supplement and medicine product definitions. Brands are managed on the Brands page."
        action={
          <>
            <Button variant="primary" disabled={isSaving} onClick={newProduct}>
              New product
            </Button>
            <Button disabled={isSaving} onClick={() => void data.loadAll()}>Reload</Button>
          </>
        }
      />

      {data.error ? (
        <ApiErrorNotice error={data.error} onRetry={() => void data.loadAll()} />
      ) : null}

      <div className="mt-grid mt-product-workspace">
        <ProductEditorPanel
          mode={mode}
          formRevision={formRevision}
          product={mode === "edit" ? data.selectedProduct : undefined}
          brands={data.brands}
          brandName={selectedBrandName}
          activeTab={activeWorkspaceTab}
          compositionCount={compositionCount}
          saving={isSaving}
          onNew={newProduct}
          onTabChange={setActiveWorkspaceTab}
          onCompositionCountChange={setCompositionCount}
          onSaved={productSaved}
          onCreate={data.createProduct}
          onUpdate={data.updateProduct}
        />

        <ProductListPanel
          products={data.products}
          selectedProductId={selectedProductId ?? null}
          loading={isLoading}
          brandNameById={data.brandNameById}
          onSelect={selectProduct}
        />
      </div>
    </div>
  );
}
