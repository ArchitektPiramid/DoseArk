import type {
  CreateProductInput,
  DosageForm,
  Product,
  ProductCategory,
  ProductUserImportance,
} from "@doseark/types";

import { parseNumberInput } from "@/core/format";

export type ProductFormValues = {
  brandId: string;
  name: string;
  category: ProductCategory;
  dosageForm: DosageForm;
  servingUnit: string;
  servingSize: string;
  unitWeightMg: string;
  barcode: string;
  description: string;
  archived: boolean;
  uiColor: string;
  uiIcon: string;
  userImportance: ProductUserImportance;
};

/** Fills the editor from an existing product or its create defaults. */
export function productFormValues(product?: Product): ProductFormValues {
  return {
    brandId: product?.brand_id ?? "",
    name: product?.name ?? "",
    category: product?.category ?? "supplement",
    dosageForm: product?.dosage_form ?? "tablet",
    servingUnit: product?.serving_unit ?? "tablet",
    servingSize: String(product?.serving_size_units ?? 1),
    unitWeightMg: product?.unit_weight_mg == null ? "" : String(product.unit_weight_mg),
    barcode: product?.barcode ?? "",
    description: product?.description ?? "",
    archived: product?.archived ?? false,
    uiColor: product?.ui_color ?? "",
    uiIcon: product?.ui_icon ?? "",
    userImportance: product?.user_importance ?? "normal",
  };
}

/** Validates the few fields required by the product API. */
export function validateProductForm(values: ProductFormValues) {
  const servingSize = parseNumberInput(values.servingSize);
  const weight = parseNumberInput(values.unitWeightMg);
  const nameMissing = values.name.trim().length === 0;
  const unitMissing = values.servingUnit.trim().length === 0;
  const servingSizeInvalid = servingSize === null || servingSize <= 0;
  const weightInvalid = values.unitWeightMg.trim() !== "" && (weight === null || weight < 0);

  return {
    nameMissing,
    unitMissing,
    servingSizeInvalid,
    weightInvalid,
    valid: !nameMissing && !unitMissing && !servingSizeInvalid && !weightInvalid,
  };
}

/** Builds the API payload; optional fields stay null when empty. */
export function buildProductInput(values: ProductFormValues): CreateProductInput | null {
  if (!validateProductForm(values).valid) return null;
  const servingSize = parseNumberInput(values.servingSize);
  if (servingSize === null) return null;

  return {
    brand_id: values.brandId || null,
    name: values.name.trim(),
    category: values.category,
    dosage_form: values.dosageForm,
    serving_unit: values.servingUnit.trim(),
    serving_size_units: servingSize,
    unit_weight_mg: parseNumberInput(values.unitWeightMg),
    barcode: values.barcode.trim() || null,
    description: values.description.trim() || null,
    archived: values.archived,
    ui_color: values.uiColor || null,
    ui_icon: values.uiIcon || null,
    user_importance: values.userImportance,
  };
}
