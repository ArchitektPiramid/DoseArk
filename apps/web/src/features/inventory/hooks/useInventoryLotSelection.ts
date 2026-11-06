import type { InventoryLot } from "@doseark/types";
import { useMemo, useState } from "react";

export function useInventoryLotSelection(lots: InventoryLot[]) {
  const [selectedLotId, setSelectedLotId] = useState<string | null>(null);

  const selectedLot = useMemo(
    () => lots.find((lot) => lot.id === selectedLotId) ?? undefined,
    [lots, selectedLotId],
  );

  return {
    selectedLot,
    selectedLotId,
    setSelectedLotId,
  };
}
