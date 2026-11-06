import type {
  CreateInventoryLotInput,
  InventoryLot,
  InventoryLotStatus,
  Product,
  StorageLocation,
  UpdateInventoryLotInput,
} from "@doseark/types";
import { useEffect, useState } from "react";

import { parseNumberInput } from "@/core/format";
import { Button, Field, FormActions, FormSection, RequiredLabel, SelectInput, TextArea, TextInput } from "@/core/ui";

import { InventoryLotStockPreview } from "./InventoryLotStockPreview";
import { StorageLocationSelect } from "./StorageLocationSelect";
import {
  buildInventoryLotInput,
  createEmptyInventoryLotFormState,
  inventoryLotToFormState,
  isInventoryLotFormValid,
  type InventoryLotFormState,
} from "../utils/inventoryLotForm";
import { INVENTORY_LOT_STATUS_OPTIONS } from "../utils/inventoryLotStatus";

type Props = {
  mode: "create" | "edit";
  lot?: InventoryLot;
  products: Product[];
  productNameById: Map<string, string>;
  storageLocations: StorageLocation[];
  disabled?: boolean;
  onCancel: () => void;
  onSaved?: () => void;
  onCreate: (input: CreateInventoryLotInput) => boolean | Promise<boolean>;
  onUpdate: (id: string, input: UpdateInventoryLotInput) => boolean | Promise<boolean>;
};

export function InventoryLotForm({
  mode,
  lot,
  products,
  productNameById,
  storageLocations,
  disabled,
  onCancel,
  onSaved,
  onCreate,
  onUpdate,
}: Props) {
  const [form, setForm] = useState<InventoryLotFormState>(createEmptyInventoryLotFormState);

  useEffect(() => {
    setForm(mode === "edit" && lot ? inventoryLotToFormState(lot) : createEmptyInventoryLotFormState());
  }, [mode, lot]);

  const unitsInitial = parseNumberInput(form.unitsInitial);
  const unitsRemaining = parseNumberInput(form.unitsRemaining);
  const productMissing = form.productId.length === 0;
  const initialInvalid = unitsInitial === null || unitsInitial <= 0;
  const remainingInvalid =
    unitsRemaining === null ||
    unitsRemaining < 0 ||
    (unitsInitial !== null && unitsRemaining > unitsInitial);
  const canSubmit = isInventoryLotFormValid(form);

  function patchForm(patch: Partial<InventoryLotFormState>) {
    setForm((current) => ({ ...current, ...patch }));
  }

  /** In create mode, keeps the starting and remaining stock in sync. */
  function changeInitialUnits(value: string) {
    setForm((current) => ({
      ...current,
      unitsInitial: value,
      unitsRemaining: mode === "create" ? value : current.unitsRemaining,
    }));
  }

  /** Persists a valid lot and notifies the editor without closing the page. */
  async function submit() {
    if (!canSubmit || disabled) return;
    const input = buildInventoryLotInput(form);
    if (!input) return;

    const saved = mode === "edit" && lot
      ? await onUpdate(lot.id, input)
      : await onCreate(input);

    if (saved) {
      if (onSaved) onSaved();
      else onCancel();
    }
  }

  return (
    <form
      className="mt-form"
      onSubmit={(event) => {
        event.preventDefault();
        void submit();
      }}
    >
      <FormSection title="Lot identity">
        <div className="mt-form-grid mt-inventory-form-grid--identity">
          <Field label={<RequiredLabel label="Product" invalid={productMissing} />}>
            <SelectInput
              value={form.productId}
              required
              aria-invalid={productMissing}
              onChange={(event) => patchForm({ productId: event.target.value })}
            >
              <option value="">Select product</option>
              {products.map((product) => (
                <option key={product.id} value={product.id}>
                  {productNameById.get(product.id) ?? product.name}
                </option>
              ))}
            </SelectInput>
          </Field>

          <Field label="Lot number">
            <TextInput value={form.lotNumber} onChange={(event) => patchForm({ lotNumber: event.target.value })} />
          </Field>

          <Field label="Storage location">
            <StorageLocationSelect
              value={form.storageLocationId}
              locations={storageLocations}
              onChange={(storageLocationId) => patchForm({ storageLocationId })}
            />
          </Field>

          <Field label="Status">
            <SelectInput
              value={form.status}
              onChange={(event) => patchForm({ status: event.target.value as InventoryLotStatus })}
            >
              {INVENTORY_LOT_STATUS_OPTIONS.map((status) => (
                <option key={status} value={status}>{status}</option>
              ))}
            </SelectInput>
          </Field>
        </div>
      </FormSection>

      <FormSection title="Stock">
        <div className="mt-form-grid mt-inventory-form-grid--stock">
          <Field label={<RequiredLabel label="Initial units" invalid={initialInvalid} />}>
            <TextInput
              type="number"
              min={0.01}
              step={0.01}
              value={form.unitsInitial}
              required
              aria-invalid={initialInvalid}
              onChange={(event) => changeInitialUnits(event.target.value)}
            />
          </Field>
          <Field label={<RequiredLabel label="Remaining units" invalid={remainingInvalid} />}>
            <TextInput
              type="number"
              min={0}
              step={0.01}
              value={form.unitsRemaining}
              required
              aria-invalid={remainingInvalid}
              onChange={(event) => patchForm({ unitsRemaining: event.target.value })}
            />
          </Field>
        </div>
        <InventoryLotStockPreview initial={unitsInitial} remaining={unitsRemaining} />
      </FormSection>

      <FormSection title="Dates & purchase">
        <div className="mt-form-grid mt-inventory-form-grid--dates">
          <Field label="Purchase date">
            <TextInput type="date" value={form.purchaseDate} onChange={(event) => patchForm({ purchaseDate: event.target.value })} />
          </Field>
          <Field label="Opened date">
            <TextInput type="date" value={form.openedDate} onChange={(event) => patchForm({ openedDate: event.target.value })} />
          </Field>
          <Field label="Expires at">
            <TextInput type="date" value={form.expiresAt} onChange={(event) => patchForm({ expiresAt: event.target.value })} />
          </Field>
          <Field label="Price">
            <TextInput type="number" min={0} step={0.01} value={form.priceAmount} onChange={(event) => patchForm({ priceAmount: event.target.value })} />
          </Field>
          <Field label="Currency">
            <TextInput value={form.priceCurrency} onChange={(event) => patchForm({ priceCurrency: event.target.value })} />
          </Field>
        </div>
      </FormSection>

      <FormSection>
        <Field label="Notes">
          <TextArea
            className="mt-inventory-form-notes"
            value={form.notes}
            placeholder="Optional package notes..."
            onChange={(event) => patchForm({ notes: event.target.value })}
          />
        </Field>
      </FormSection>

      <FormActions>
        <Button type="button" onClick={onCancel} disabled={disabled}>Cancel</Button>
        <Button type="submit" variant="primary" disabled={!canSubmit || disabled}>
          {disabled ? "Saving..." : mode === "edit" ? "Save lot" : "Create lot"}
        </Button>
      </FormActions>
    </form>
  );
}
