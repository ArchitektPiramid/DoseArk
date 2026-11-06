import type { CreateDoseLogInput, InventoryLot, Product } from "@doseark/types";
import { useMemo, useState } from "react";

import {
  bulkDoseDateTimeToIso,
  parseBulkDoseText,
  type BulkDoseAmountMode,
  type BulkDoseParsedEntry,
} from "../utils/bulkDoseParser";

export type BulkDosePreviewEntry = BulkDoseParsedEntry & {
  effectiveMode: BulkDoseAmountMode;
  unitsTaken: number | null;
  takenAt: string | null;
  error: string | null;
};

export type BulkDoseBatchItem = {
  key: string;
  input: CreateDoseLogInput;
};

type Input = {
  lots: InventoryLot[];
  products: Product[];
  productById: Map<string, Product>;
};

export function useBulkDoseEntry({ lots, products, productById }: Input) {
  const [productId, setProductId] = useState("");
  const [lotId, setLotId] = useState("");
  const [defaultAmountMode, setDefaultAmountMode] = useState<BulkDoseAmountMode>("mg");
  const [sourceText, setSourceText] = useState("");

  const activeLots = useMemo(
    () => lots.filter((lot) => lot.status === "active" && lot.units_remaining > 0),
    [lots],
  );
  const filteredLots = useMemo(
    () => (productId ? activeLots.filter((lot) => lot.product_id === productId) : []),
    [activeLots, productId],
  );
  const selectedLot = useMemo(() => activeLots.find((lot) => lot.id === lotId), [activeLots, lotId]);
  const selectedProduct = selectedLot ? productById.get(selectedLot.product_id) : productById.get(productId);
  const parsed = useMemo(() => parseBulkDoseText(sourceText), [sourceText]);

  const previewEntries = useMemo<BulkDosePreviewEntry[]>(() => {
    return parsed.entries.map((entry) => {
      const effectiveMode = entry.amountModeOverride ?? defaultAmountMode;
      const unitWeightMg = selectedProduct?.unit_weight_mg ?? null;
      let unitsTaken: number | null = null;
      let error: string | null = null;

      if (effectiveMode === "mg") {
        if (!unitWeightMg || unitWeightMg <= 0) {
          error = "Selected product has no unit weight in mg.";
        } else {
          unitsTaken = entry.amount / unitWeightMg;
        }
      } else {
        unitsTaken = entry.amount;
      }

      if (unitsTaken !== null && (!Number.isFinite(unitsTaken) || unitsTaken <= 0)) {
        unitsTaken = null;
        error = "Dose converts to an invalid unit amount.";
      }

      let takenAt: string | null = null;
      try {
        takenAt = bulkDoseDateTimeToIso(entry.date, entry.time);
      } catch {
        error = "Could not build dose date/time.";
      }

      return {
        ...entry,
        effectiveMode,
        unitsTaken,
        takenAt,
        error,
      };
    });
  }, [defaultAmountMode, parsed.entries, selectedProduct]);

  const totalUnits = useMemo(
    () => previewEntries.reduce((sum, entry) => sum + (entry.unitsTaken ?? 0), 0),
    [previewEntries],
  );
  const conversionErrors = previewEntries.filter((entry) => entry.error !== null);
  const stockExceeded = selectedLot !== undefined && totalUnits > selectedLot.units_remaining + 1e-9;
  const canSave =
    selectedProduct !== undefined &&
    selectedLot !== undefined &&
    previewEntries.length > 0 &&
    parsed.errors.length === 0 &&
    conversionErrors.length === 0 &&
    !stockExceeded;

  function changeProduct(nextProductId: string) {
    setProductId(nextProductId);
    setLotId("");

    const nextProduct = productById.get(nextProductId);
    if (nextProduct?.unit_weight_mg && nextProduct.unit_weight_mg > 0) {
      setDefaultAmountMode("mg");
    } else {
      setDefaultAmountMode("units");
    }
  }

  function changeLot(nextLotId: string) {
    setLotId(nextLotId);
    const lot = activeLots.find((row) => row.id === nextLotId);
    if (lot && lot.product_id !== productId) setProductId(lot.product_id);
  }

  function buildBatchItems(): BulkDoseBatchItem[] {
    if (!selectedProduct || !selectedLot || !canSave) return [];

    const servingSize = selectedProduct.serving_size_units > 0 ? selectedProduct.serving_size_units : 1;

    return previewEntries
      .map((entry) => ({ entry, timestamp: entry.takenAt ? new Date(entry.takenAt).getTime() : 0 }))
      .sort((left, right) => left.timestamp - right.timestamp || left.entry.line - right.entry.line)
      .map(({ entry }) => ({
        key: entry.key,
        input: {
          product_id: selectedLot.product_id,
          inventory_lot_id: selectedLot.id,
          taken_at: entry.takenAt!,
          units_taken: entry.unitsTaken!,
          unit: selectedProduct.serving_unit,
          serving_multiplier: entry.unitsTaken! / servingSize,
          reason: "bulk_debug_entry",
          notes: null,
        },
      }));
  }

  return {
    productId,
    lotId,
    defaultAmountMode,
    sourceText,
    products,
    filteredLots,
    selectedProduct,
    selectedLot,
    parsed,
    previewEntries,
    totalUnits,
    conversionErrors,
    stockExceeded,
    canSave,
    changeProduct,
    changeLot,
    setDefaultAmountMode,
    setSourceText,
    clearSourceText: () => setSourceText(""),
    buildBatchItems,
  };
}
