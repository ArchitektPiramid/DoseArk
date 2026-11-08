import { useEffect } from "react";

import { useApiProfiles } from "@/app/api_profiles";
import { useNutrientMutations } from "./useNutrientMutations";
import { useNutrientSelection } from "./useNutrientSelection";
import { useNutrientsLoader } from "./useNutrientsLoader";

export function useNutrientsPageData() {
  const { effectiveApiBaseUrl } = useApiProfiles();
  const loader = useNutrientsLoader();
  const selection = useNutrientSelection(loader.nutrients);

  async function loadNutrients() {
    return loader.loadNutrients();
  }

  const mutations = useNutrientMutations({
    setStatus: loader.setStatus,
    setError: loader.setError,
    reload: loadNutrients,
    setSelectedNutrientId: selection.setSelectedNutrientId,
  });

  useEffect(() => {
    void loader.loadNutrients();
  }, [effectiveApiBaseUrl]);

  return {
    nutrients: loader.nutrients,
    status: loader.status,
    error: loader.error,
    selectedNutrient: selection.selectedNutrient,
    selectedNutrientId: selection.selectedNutrientId,
    setSelectedNutrientId: selection.setSelectedNutrientId,
    loadNutrients,
    createNutrient: mutations.createNutrient,
    updateNutrient: mutations.updateNutrient,
  };
}
