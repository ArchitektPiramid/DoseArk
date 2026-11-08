import { useEffect, useMemo } from "react";

import { useApiProfiles } from "@/app/api_profiles";
import { useProductMutations } from "./useProductMutations";
import { useProductSelection } from "./useProductSelection";
import { useProductsLoader } from "./useProductsLoader";

export function useProductsPageData() {
  const { effectiveApiBaseUrl } = useApiProfiles();
  const loader = useProductsLoader();
  const selection = useProductSelection(loader.products);

  const brandNameById = useMemo(() => {
    return new Map(loader.brands.map((brand) => [brand.id, brand.name]));
  }, [loader.brands]);

  /** Loads catalog data without selecting a product while the create form is visible. */
  async function loadAll() {
    return loader.loadAll();
  }

  const mutations = useProductMutations({
    setStatus: loader.setStatus,
    setError: loader.setError,
    reload: loadAll,
    setSelectedProductId: selection.setSelectedProductId,
  });

  useEffect(() => {
    void loader.loadAll();
  }, [effectiveApiBaseUrl]);

  return {
    products: loader.products,
    brands: loader.brands,
    status: loader.status,
    error: loader.error,
    selectedProduct: selection.selectedProduct,
    selectedProductId: selection.selectedProductId,
    setSelectedProductId: selection.setSelectedProductId,
    brandNameById,
    loadAll,
    createBrand: mutations.createBrand,
    createProduct: mutations.createProduct,
    updateProduct: mutations.updateProduct,
  };
}
