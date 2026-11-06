import type { Nutrient } from "@doseark/types";
import { useMemo, useState } from "react";

export function useNutrientSelection(nutrients: Nutrient[]) {
  const [selectedNutrientId, setSelectedNutrientId] = useState<string | null>(null);

  const selectedNutrient = useMemo(
    () => nutrients.find((nutrient) => nutrient.id === selectedNutrientId) ?? undefined,
    [nutrients, selectedNutrientId],
  );

  return {
    selectedNutrient,
    selectedNutrientId,
    setSelectedNutrientId,
  };
}
