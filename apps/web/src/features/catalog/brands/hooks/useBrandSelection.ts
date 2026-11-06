import type { Brand } from "@doseark/types";
import { useMemo, useState } from "react";

export function useBrandSelection(brands: Brand[]) {
  const [selectedBrandId, setSelectedBrandId] = useState<string | null>(null);

  const selectedBrand = useMemo(
    () => brands.find((brand) => brand.id === selectedBrandId) ?? undefined,
    [brands, selectedBrandId],
  );

  return {
    selectedBrand,
    selectedBrandId,
    setSelectedBrandId,
  };
}
