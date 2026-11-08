import type { Id, IsoDateTime } from "../common";

export type ProductCategory = "supplement" | "medication" | "food" | "herb" | "other";

export type DosageForm =
  | "tablet"
  | "capsule"
  | "softgel"
  | "powder"
  | "liquid"
  | "drops"
  | "spray"
  | "gummy"
  | "other";

export type ProductUserImportance = "low" | "normal" | "high" | "critical";

export type Product = {
  id: Id;
  brand_id: Id | null;
  name: string;
  category: ProductCategory;
  dosage_form: DosageForm;
  serving_unit: string;
  serving_size_units: number;
  unit_weight_mg: number | null;
  barcode: string | null;
  description: string | null;
  archived: boolean;
  created_at: IsoDateTime;
  updated_at: IsoDateTime;
  ui_color: string | null;
  ui_icon: string | null;
  user_importance: ProductUserImportance;
};

export type CreateProductInput = {
  brand_id?: Id | null;
  name: string;
  category: ProductCategory;
  dosage_form: DosageForm;
  serving_unit: string;
  serving_size_units: number;
  unit_weight_mg?: number | null;
  barcode?: string | null;
  description?: string | null;
  archived?: boolean;
  ui_color: string | null;
  ui_icon: string | null;
  user_importance: ProductUserImportance;
};

export type UpdateProductInput = {
  brand_id?: Id | null;
  name: string;
  category: ProductCategory;
  dosage_form: DosageForm;
  serving_unit: string;
  serving_size_units: number;
  unit_weight_mg?: number | null;
  barcode?: string | null;
  description?: string | null;
  archived?: boolean;
  ui_color: string | null;
  ui_icon: string | null;
  user_importance: ProductUserImportance;
};
