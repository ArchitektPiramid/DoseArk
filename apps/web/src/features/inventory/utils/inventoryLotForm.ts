import type {
  CreateInventoryLotInput,
  InventoryLot,
  InventoryLotStatus,
  UpdateInventoryLotInput,
} from "@doseark/types";

import { formatEditableNumber, parseNumberInput } from "@/core/format";

export type InventoryLotFormState = {
  productId: string;
  lotNumber: string;
  storageLocationId: string;
  unitsInitial: string;
  unitsRemaining: string;
  purchaseDate: string;
  openedDate: string;
  expiresAt: string;
  priceAmount: string;
  priceCurrency: string;
  status: InventoryLotStatus;
  notes: string;
};

export function createEmptyInventoryLotFormState(): InventoryLotFormState {
  return {
    productId: "",
    lotNumber: "",
    storageLocationId: "",
    unitsInitial: "1",
    unitsRemaining: "1",
    purchaseDate: "",
    openedDate: "",
    expiresAt: "",
    priceAmount: "",
    priceCurrency: "PLN",
    status: "active",
    notes: "",
  };
}

export function inventoryLotToFormState(lot: InventoryLot): InventoryLotFormState {
  return {
    productId: lot.product_id,
    lotNumber: lot.lot_number ?? "",
    storageLocationId: lot.storage_location_id ?? "",
    unitsInitial: formatEditableNumber(lot.units_initial),
    unitsRemaining: formatEditableNumber(lot.units_remaining),
    purchaseDate: lot.purchase_date ?? "",
    openedDate: lot.opened_date ?? "",
    expiresAt: lot.expires_at ?? "",
    priceAmount: formatEditableNumber(lot.price_amount, 2),
    priceCurrency: lot.price_currency ?? "PLN",
    status: lot.status,
    notes: lot.notes ?? "",
  };
}

export function isInventoryLotFormValid(form: InventoryLotFormState): boolean {
  const unitsInitial = parseNumberInput(form.unitsInitial);
  const unitsRemaining = parseNumberInput(form.unitsRemaining);
  const priceAmount = parseOptionalNumber(form.priceAmount);

  return (
    form.productId.length > 0 &&
    unitsInitial !== null &&
    unitsInitial > 0 &&
    unitsRemaining !== null &&
    unitsRemaining >= 0 &&
    unitsRemaining <= unitsInitial &&
    priceAmount !== undefined &&
    (priceAmount === null || priceAmount >= 0)
  );
}

export function buildInventoryLotInput(
  form: InventoryLotFormState,
): CreateInventoryLotInput | UpdateInventoryLotInput | null {
  const unitsInitial = parseNumberInput(form.unitsInitial);
  const unitsRemaining = parseNumberInput(form.unitsRemaining);
  const priceAmount = parseOptionalNumber(form.priceAmount);

  if (
    !form.productId ||
    unitsInitial === null ||
    unitsInitial <= 0 ||
    unitsRemaining === null ||
    unitsRemaining < 0 ||
    unitsRemaining > unitsInitial ||
    priceAmount === undefined
  ) {
    return null;
  }

  return {
    product_id: form.productId,
    lot_number: form.lotNumber.trim() || null,
    storage_location_id: form.storageLocationId || null,
    storage_location: null,
    units_initial: unitsInitial,
    units_remaining: unitsRemaining,
    purchase_date: form.purchaseDate || null,
    opened_date: form.openedDate || null,
    expires_at: form.expiresAt || null,
    price_amount: priceAmount,
    price_currency: form.priceCurrency.trim() || null,
    status: form.status,
    notes: form.notes.trim() || null,
  };
}

function parseOptionalNumber(value: string): number | null | undefined {
  if (value.trim().length === 0) {
    return null;
  }

  const parsed = parseNumberInput(value);

  if (parsed === null) {
    return undefined;
  }

  return parsed;
}
