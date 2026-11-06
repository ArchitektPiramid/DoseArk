import { useState } from "react";
import { ApiErrorNotice } from "@/app/system_status";
import { Button, PageHeader } from "@/core/ui";
import { NutrientFormPanel } from "../components/NutrientFormPanel";
import { NutrientsTable } from "../components/NutrientsTable";
import { useNutrientsPageData } from "../hooks/useNutrientsPageData";

type FormMode = "create" | "edit";

export function NutrientsPage() {
  const data = useNutrientsPageData();
  const [mode, setMode] = useState<FormMode>("create");

  const isLoading = data.status === "loading";
  const isSaving = data.status === "saving";

  function selectNutrient(id: string) {
    data.setSelectedNutrientId(id);
    setMode("edit");
  }

  function newNutrient() {
    data.setSelectedNutrientId(null);
    setMode("create");
  }

  return (
    <>
      <PageHeader
        title="Nutrients"
        description="Manage vitamins, minerals, herbs, drug substances and other active ingredients."
        action={
          <>
            <Button onClick={newNutrient}>New nutrient</Button>
            <Button onClick={() => void data.loadNutrients()}>Reload</Button>
          </>
        }
      />

      {data.error ? (
        <ApiErrorNotice error={data.error} onRetry={() => void data.loadNutrients()} />
      ) : null}

      <div className="mt-grid mt-grid-master-detail">
        <NutrientFormPanel
          mode={mode}
          nutrient={data.selectedNutrient}
          disabled={isSaving}
          onNew={newNutrient}
          onCreate={(input) => void data.createNutrient(input)}
          onUpdate={(id, input) => void data.updateNutrient(id, input)}
        />

        <NutrientsTable
          nutrients={data.nutrients}
          selectedNutrientId={data.selectedNutrientId}
          loading={isLoading}
          onSelect={selectNutrient}
        />
      </div>
    </>
  );
}
