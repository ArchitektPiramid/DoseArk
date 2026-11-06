import { useState } from "react";
import { ApiErrorNotice } from "@/app/system_status";
import { Button, PageHeader } from "@/core/ui";
import { BrandFormPanel } from "../components/BrandFormPanel";
import { BrandsTable } from "../components/BrandsTable";
import { useBrandsPageData } from "../hooks/useBrandsPageData";

type FormMode = "create" | "edit";

export function BrandsPage() {
  const data = useBrandsPageData();
  const [mode, setMode] = useState<FormMode>("create");

  const isLoading = data.status === "loading";
  const isSaving = data.status === "saving";

  function selectBrand(id: string) {
    data.setSelectedBrandId(id);
    setMode("edit");
  }

  function newBrand() {
    data.setSelectedBrandId(null);
    setMode("create");
  }

  return (
    <>
      <PageHeader
        title="Brands"
        description="Manage product brands and manufacturers."
        action={
          <>
            <Button onClick={newBrand}>New brand</Button>
            <Button onClick={() => void data.loadBrands()}>Reload</Button>
          </>
        }
      />

      {data.error ? (
        <ApiErrorNotice error={data.error} onRetry={() => void data.loadBrands()} />
      ) : null}

      <div className="mt-grid mt-grid-master-detail">
        <BrandFormPanel
          mode={mode}
          brand={data.selectedBrand}
          disabled={isSaving}
          onNew={newBrand}
          onCreate={(input) => void data.createBrand(input)}
          onUpdate={(id, input) => void data.updateBrand(id, input)}
        />

        <BrandsTable
          brands={data.brands}
          selectedBrandId={data.selectedBrandId}
          loading={isLoading}
          onSelect={selectBrand}
        />
      </div>
    </>
  );
}
