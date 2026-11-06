import { useEffect, useRef } from "react";
import { useNutrientMutations } from "./useNutrientMutations";
import { useNutrientSelection } from "./useNutrientSelection";
import { useNutrientsLoader } from "./useNutrientsLoader";

export function useNutrientsPageData() {
  const loader = useNutrientsLoader();
  const selection = useNutrientSelection(loader.nutrients);
  const didInitialLoad = useRef(false);

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
    if (didInitialLoad.current) return;

    didInitialLoad.current = true;
    void loadNutrients();
  }, []);

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
