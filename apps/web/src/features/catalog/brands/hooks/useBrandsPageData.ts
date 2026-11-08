import { useEffect } from "react";

import { useApiProfiles } from "@/app/api_profiles";
import { useBrandMutations } from "./useBrandMutations";
import { useBrandSelection } from "./useBrandSelection";
import { useBrandsLoader } from "./useBrandsLoader";

export function useBrandsPageData() {
  const { effectiveApiBaseUrl } = useApiProfiles();
  const loader = useBrandsLoader();
  const selection = useBrandSelection(loader.brands);

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
    void loader.loadBrands();
  }, [effectiveApiBaseUrl]);

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
