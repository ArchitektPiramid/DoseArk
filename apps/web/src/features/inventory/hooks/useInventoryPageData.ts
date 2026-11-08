import { useEffect, useMemo } from "react";

import { useApiProfiles } from "@/app/api_profiles";
import { useInventoryLotMutations } from "./useInventoryLotMutations";
import { useInventoryLotSelection } from "./useInventoryLotSelection";
import { useInventoryLotsLoader } from "./useInventoryLotsLoader";

export function useInventoryPageData() {
  const { effectiveApiBaseUrl } = useApiProfiles();
  const loader = useInventoryLotsLoader();
  const selection = useInventoryLotSelection(loader.lots);

  const productNameById = useMemo(() => {
    const brandNameById = new Map(loader.brands.map((brand) => [brand.id, brand.name]));
    const map = new Map<string, string>();
    for (const product of loader.products) {
      const brandName = product.brand_id ? brandNameById.get(product.brand_id) : undefined;
      map.set(product.id, brandName ? `${brandName} / ${product.name}` : product.name);
    }
    return map;
  }, [loader.products, loader.brands]);

  const storageLocationById = useMemo(() => {
    return new Map(loader.storageLocations.map((location) => [location.id, location]));
  }, [loader.storageLocations]);

  async function loadAll() {
    return loader.loadAll();
  }

  const mutations = useInventoryLotMutations({
    setStatus: loader.setStatus,
    setError: loader.setError,
    reload: loadAll,
    setSelectedLotId: selection.setSelectedLotId,
  });

  useEffect(() => {
    void loader.loadAll();
  }, [effectiveApiBaseUrl]);

  return {
    lots: loader.lots,
    products: loader.products,
    brands: loader.brands,
    storageLocations: loader.storageLocations,
    status: loader.status,
    error: loader.error,
    selectedLot: selection.selectedLot,
    selectedLotId: selection.selectedLotId,
    setSelectedLotId: selection.setSelectedLotId,
    productNameById,
    storageLocationById,
    loadAll,
    createInventoryLot: mutations.createInventoryLot,
    updateInventoryLot: mutations.updateInventoryLot,
  };
}
