import type {
  CreateDoseLogInput,
  DoseLog,
  InventoryLot,
  Product,
  UpdateDoseLogInput,
} from "@doseark/types";
import { useEffect, useMemo, useState } from "react";

import type { FormFieldState } from "@/core/ui";
import {
  createInitialDoseTimeSelection,
  doseTimeSelectionFromIso,
  doseTimeSelectionToIso,
  type DoseTimeSelection,
} from "../utils/doseTime";

type Input = {
  lots: InventoryLot[];
  productById: Map<string, Product>;
  editingDoseLog?: DoseLog;
  onCreate: (input: CreateDoseLogInput) => void | Promise<void>;
  onUpdate: (id: string, input: UpdateDoseLogInput) => void | Promise<void>;
  onCancelEdit: () => void;
};

export function useDoseLogForm({
  lots,
  productById,
  editingDoseLog,
  onCreate,
  onUpdate,
  onCancelEdit,
}: Input) {
  const [productId, setProductId] = useState("");
  const [lotId, setLotId] = useState("");
  const [unitsTaken, setUnitsTaken] = useState(1);
  const [timeSelection, setTimeSelection] = useState<DoseTimeSelection>(
    createInitialDoseTimeSelection(),
  );
  const [notes, setNotes] = useState("");
  const [dosePickerKey, setDosePickerKey] = useState(0);
  const [submitAttempted, setSubmitAttempted] = useState(false);

  const isEditMode = editingDoseLog !== undefined;
  const availableLots = useMemo(
    () => buildAvailableLots(lots, editingDoseLog),
    [editingDoseLog, lots],
  );
  const filteredLots = useMemo(
    () => (productId ? availableLots.filter((lot) => lot.product_id === productId) : availableLots),
    [availableLots, productId],
  );
  const selectedLot = useMemo(
    () => availableLots.find((lot) => lot.id === lotId),
    [availableLots, lotId],
  );
  const selectedProduct = selectedLot ? productById.get(selectedLot.product_id) : undefined;
  const unit = selectedProduct?.serving_unit ?? "unit";
  const unitWeightMg = selectedProduct?.unit_weight_mg ?? null;
  const effectiveStockBefore = useMemo(() => {
    if (!selectedLot) return undefined;

    if (editingDoseLog && editingDoseLog.inventory_lot_id === selectedLot.id) {
      return selectedLot.units_remaining + editingDoseLog.units_taken;
    }

    return selectedLot.units_remaining;
  }, [editingDoseLog, selectedLot]);

  const lotFieldState = getLotFieldState(Boolean(selectedLot), submitAttempted);
  const doseFieldState = getDoseFieldState(unitsTaken, effectiveStockBefore);
  const doseValidation = getDoseValidation(unitsTaken, effectiveStockBefore);
  const canSubmit =
    selectedLot !== undefined &&
    selectedProduct !== undefined &&
    unitsTaken > 0 &&
    effectiveStockBefore !== undefined &&
    unitsTaken <= effectiveStockBefore;

  useEffect(() => {
    if (!editingDoseLog) {
      resetFormState();
      return;
    }

    setProductId(editingDoseLog.product_id);
    setLotId(editingDoseLog.inventory_lot_id ?? "");
    setUnitsTaken(editingDoseLog.units_taken);
    setTimeSelection(doseTimeSelectionFromIso(editingDoseLog.taken_at));
    setNotes(editingDoseLog.notes ?? "");
    setSubmitAttempted(false);
    setDosePickerKey((value) => value + 1);
  }, [editingDoseLog?.id]);

  function changeProduct(nextProductId: string) {
    setProductId(nextProductId);
    if (!nextProductId) return;

    const currentLot = availableLots.find((lot) => lot.id === lotId);
    if (currentLot && currentLot.product_id !== nextProductId) setLotId("");
  }

  function changeLot(nextLotId: string) {
    setLotId(nextLotId);
    setUnitsTaken(1);
    setDosePickerKey((value) => value + 1);
  }

  function resetFormState() {
    setProductId("");
    setLotId("");
    setUnitsTaken(1);
    setTimeSelection(createInitialDoseTimeSelection());
    setNotes("");
    setDosePickerKey((value) => value + 1);
    setSubmitAttempted(false);
  }

  function resetForm() {
    if (isEditMode) {
      onCancelEdit();
      return;
    }

    resetFormState();
  }

  function submit() {
    setSubmitAttempted(true);
    if (!selectedLot || !selectedProduct || !canSubmit) return;

    const servingSize = selectedProduct.serving_size_units > 0 ? selectedProduct.serving_size_units : 1;
    const input = {
      product_id: selectedLot.product_id,
      inventory_lot_id: selectedLot.id,
      taken_at: doseTimeSelectionToIso(timeSelection),
      units_taken: unitsTaken,
      unit,
      serving_multiplier: unitsTaken / servingSize,
      reason: "manual_entry",
      notes: notes.trim() || null,
    };

    if (editingDoseLog) {
      onUpdate(editingDoseLog.id, input);
      return;
    }

    onCreate(input);
  }

  return {
    productId,
    lotId,
    unitsTaken,
    timeSelection,
    notes,
    dosePickerKey,
    isEditMode,
    availableLots,
    filteredLots,
    selectedLot,
    selectedProduct,
    unit,
    unitWeightMg,
    effectiveStockBefore,
    lotFieldState,
    doseFieldState,
    doseValidation,
    canSubmit,
    changeProduct,
    changeLot,
    setUnitsTaken,
    setTimeSelection,
    setNotes,
    resetForm,
    submit,
  };
}

function buildAvailableLots(lots: InventoryLot[], editingDoseLog?: DoseLog): InventoryLot[] {
  const activeLots = lots.filter((lot) => lot.status === "active" && lot.units_remaining > 0);
  if (!editingDoseLog?.inventory_lot_id) return activeLots;

  const editingLot = lots.find((lot) => lot.id === editingDoseLog.inventory_lot_id);
  if (!editingLot || activeLots.some((lot) => lot.id === editingLot.id)) return activeLots;
  return [editingLot, ...activeLots];
}

function getLotFieldState(hasLot: boolean, submitAttempted: boolean): FormFieldState {
  if (hasLot) return "ok";
  return submitAttempted ? "error" : "warning";
}

function getDoseFieldState(unitsTaken: number, stockRemaining?: number): FormFieldState {
  return getDoseValidation(unitsTaken, stockRemaining) ? "error" : "neutral";
}

function getDoseValidation(
  unitsTaken: number,
  stockRemaining?: number,
): "dosePositive" | "doseTooHigh" | null {
  if (!Number.isFinite(unitsTaken) || unitsTaken <= 0) return "dosePositive";
  if (stockRemaining !== undefined && unitsTaken > stockRemaining) return "doseTooHigh";
  return null;
}
