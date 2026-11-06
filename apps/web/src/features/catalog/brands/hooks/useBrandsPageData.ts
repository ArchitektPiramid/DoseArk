import { useEffect, useRef } from "react";
import { useBrandMutations } from "./useBrandMutations";
import { useBrandSelection } from "./useBrandSelection";
import { useBrandsLoader } from "./useBrandsLoader";

export function useBrandsPageData() {
  const loader = useBrandsLoader();
  const selection = useBrandSelection(loader.brands);
  const didInitialLoad = useRef(false);

  async function loadBrands() {
    return loader.loadBrands();
  }

  const mutations = useBrandMutations({
    setStatus: loader.setStatus,
    setError: loader.setError,
    reload: loadBrands,
    setSelectedBrandId: selection.setSelectedBrandId,
  });

  useEffect(() => {
    if (didInitialLoad.current) return;

    didInitialLoad.current = true;
    void loadBrands();
  }, []);

  return {
    brands: loader.brands,
    status: loader.status,
    error: loader.error,

    selectedBrand: selection.selectedBrand,
    selectedBrandId: selection.selectedBrandId,
    setSelectedBrandId: selection.setSelectedBrandId,

    loadBrands,
    createBrand: mutations.createBrand,
    updateBrand: mutations.updateBrand,
  };
}
