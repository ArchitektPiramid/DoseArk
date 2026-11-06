import type { Nutrient, ProductNutrient, SaveProductNutrientInput } from "@doseark/types";

import { formatEditableNumber, formatNumber, parseNumberInput } from "@/core/format";

export type ProductCompositionDraftRow = {
  local_id: string;
  nutrient_id: string;
  amount_per_serving: string;
  unit: string;
  label_daily_value_percent: string;
  label_reference_amount: string;
  label_reference_unit: string;
  source: string;
  notes: string;
};

export type ReferenceInfo = {
  label: string;
  tone: "default" | "ok" | "warn";
};

export function createEmptyCompositionDraftRow(): ProductCompositionDraftRow {
  return {
    local_id: crypto.randomUUID(),
    nutrient_id: "",
    amount_per_serving: "1",
    unit: "mg",
    label_daily_value_percent: "",
    label_reference_amount: "",
    label_reference_unit: "",
    source: "label",
    notes: "",
  };
}

export function toCompositionDraftRow(row: ProductNutrient): ProductCompositionDraftRow {
  return {
    local_id: row.id,
    nutrient_id: row.nutrient_id,
    amount_per_serving: formatEditableNumber(row.amount_per_serving, 6),
    unit: row.unit,
    label_daily_value_percent:
      row.label_daily_value_percent === null || row.label_daily_value_percent === undefined
        ? ""
        : formatEditableNumber(row.label_daily_value_percent, 6),
    label_reference_amount:
      row.label_reference_amount === null || row.label_reference_amount === undefined
        ? ""
        : formatEditableNumber(row.label_reference_amount, 6),
    label_reference_unit: row.label_reference_unit ?? "",
    source: row.source,
    notes: row.notes ?? "",
  };
}

export function toSaveProductNutrientInput(
  row: ProductCompositionDraftRow,
): SaveProductNutrientInput | null {
  const amount = parseNumberInput(row.amount_per_serving);
  const labelDailyValuePercent = parseNumberInput(row.label_daily_value_percent);
  const labelReferenceAmount = parseNumberInput(row.label_reference_amount);

  if (!row.nutrient_id || amount === null || amount <= 0 || !row.unit.trim()) {
    return null;
  }

  return {
    nutrient_id: row.nutrient_id,
    amount_per_serving: amount,
    unit: row.unit.trim(),
    label_daily_value_percent: labelDailyValuePercent,
    label_reference_amount: labelReferenceAmount,
    label_reference_unit: row.label_reference_unit.trim() || null,
    source: row.source.trim() || "label",
    notes: row.notes.trim() || null,
  };
}

export function getReferenceInfo(
  row: ProductCompositionDraftRow,
  nutrient?: Nutrient,
): ReferenceInfo {
  const amount = parseNumberInput(row.amount_per_serving);
  const labelPercent = parseNumberInput(row.label_daily_value_percent);

  if (
    amount !== null &&
    nutrient?.reference_daily_amount &&
    nutrient.reference_daily_unit &&
    nutrient.reference_daily_unit === row.unit
  ) {
    const percent = (amount / nutrient.reference_daily_amount) * 100;

    return {
      label: `global ${formatNumber(percent, { maxDecimals: 1 })}%`,
      tone: "ok",
    };
  }

  if (nutrient?.reference_daily_amount && nutrient.reference_daily_unit !== row.unit) {
    return {
      label: "global unit mismatch",
      tone: "warn",
    };
  }

  if (labelPercent !== null) {
    return {
      label: `label ${formatNumber(labelPercent, { maxDecimals: 1 })}%`,
      tone: "default",
    };
  }

  return {
    label: "no reference",
    tone: "warn",
  };
}
