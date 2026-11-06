import type {
  CreateDoseLogInput,
  DoseLog,
  InventoryLot,
  Product,
  UpdateDoseLogInput,
} from "@doseark/types";
import { useTranslation } from "react-i18next";

import { Button, FormActions, FormField, TextArea } from "@/core/ui";
import { useDoseLogForm } from "../hooks/useDoseLogForm";
import { DoseAmountPicker } from "./DoseAmountPicker";
import { DoseLotFields } from "./DoseLotFields";
import { DoseTimePicker } from "./DoseTimePicker";

export type DoseLogFormProps = {
  lots: InventoryLot[];
  products: Product[];
  productById: Map<string, Product>;
  productNameById: Map<string, string>;
  editingDoseLog?: DoseLog;
  disabled?: boolean;
  inline?: boolean;
  onCreate: (input: CreateDoseLogInput) => void | Promise<void>;
  onUpdate: (id: string, input: UpdateDoseLogInput) => void | Promise<void>;
  onCancelEdit: () => void;
  onClose: () => void;
};

export function DoseLogForm(props: DoseLogFormProps) {
  const { t } = useTranslation("dosing");
  const form = useDoseLogForm(props);
  const doseMessage = form.doseValidation ? t(`validation.${form.doseValidation}`) : undefined;

  function handleSecondaryAction() {
    if (form.isEditMode) {
      props.onCancelEdit();
      props.onClose();
      return;
    }

    props.onClose();
  }

  return (
    <div className="mt-form mt-dose-form-content">
      <DoseLotFields
        products={props.products}
        productId={form.productId}
        lotId={form.lotId}
        lots={form.filteredLots}
        selectedLot={form.selectedLot}
        productNameById={props.productNameById}
        lotFieldState={form.lotFieldState}
        onProductChange={form.changeProduct}
        onLotChange={form.changeLot}
      />

      <FormField label={t("form.unitsTaken")} state={form.doseFieldState} message={doseMessage}>
        <DoseAmountPicker
          key={form.dosePickerKey}
          unit={form.unit}
          unitWeightMg={form.unitWeightMg}
          value={form.unitsTaken}
          stockBefore={form.effectiveStockBefore}
          onChange={form.setUnitsTaken}
        />
      </FormField>

      <FormField label={t("form.timeTaken")} state="neutral">
        <DoseTimePicker value={form.timeSelection} onChange={form.setTimeSelection} />
      </FormField>

      <FormField label={t("form.notes")}>
        <TextArea
          className="mt-dose-notes"
          value={form.notes}
          onChange={(event) => form.setNotes(event.target.value)}
        />
      </FormField>

      <FormActions sticky={!props.inline}>
        <Button variant="quiet" onClick={handleSecondaryAction} disabled={props.disabled}>
          {form.isEditMode ? t("form.cancelEdit") : t(props.inline ? "form.reset" : "drawer.cancel")}
        </Button>
        <Button variant="primary" disabled={!form.canSubmit || props.disabled} onClick={form.submit}>
          {props.disabled ? t("form.saving") : form.isEditMode ? t("form.update") : t("form.create")}
        </Button>
      </FormActions>
    </div>
  );
}
