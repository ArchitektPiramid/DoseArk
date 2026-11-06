import type { Product } from "@doseark/types";
import { useMemo, useState } from "react";

export function useProductSelection(products: Product[]) {
  const [selectedProductId, setSelectedProductId] = useState<string | null>(null);

  const selectedProduct = useMemo(() => {
    if (!selectedProductId) return undefined;

    return products.find((product) => product.id === selectedProductId);
  }, [products, selectedProductId]);

  return {
    selectedProduct,
    selectedProductId,
    setSelectedProductId,
  };
}
