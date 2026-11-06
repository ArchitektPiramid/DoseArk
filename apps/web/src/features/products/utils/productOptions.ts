import type {
  DosageForm,
  ProductCategory,
  ProductUserImportance,
} from "@doseark/types";

export const productCategoryOptions: ProductCategory[] = [
  "supplement",
  "medication",
  "food",
  "herb",
  "other",
];

export const dosageFormOptions: DosageForm[] = [
  "tablet",
  "capsule",
  "softgel",
  "powder",
  "liquid",
  "drops",
  "spray",
  "gummy",
  "other",
];

export const productImportanceOptions: ProductUserImportance[] = [
  "low",
  "normal",
  "high",
  "critical",
];
