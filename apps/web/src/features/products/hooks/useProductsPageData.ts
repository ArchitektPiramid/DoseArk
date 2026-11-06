import { useEffect, useMemo, useRef } from "react";

import { useProductMutations } from "./useProductMutations";
import { useProductSelection } from "./useProductSelection";
import { useProductsLoader } from "./useProductsLoader";

export function useProductsPageData() {
  const loader = useProductsLoader();
  const selection = useProductSelection(loader.products);
  const didInitialLoad = useRef(false);

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
    if (didInitialLoad.current) return;

    didInitialLoad.current = true;
    void loadAll();
  }, []);

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
