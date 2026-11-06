import type { Id, IsoDateTime } from "../common";

export type NutrientKind =
  | "vitamin"
  | "mineral"
  | "amino_acid"
  | "fatty_acid"
  | "herb_extract"
  | "drug_substance"
  | "probiotic"
  | "other";

export type Nutrient = {
  id: Id;
  canonical_name: string;
  display_name: string;
  kind: NutrientKind;
  default_unit: string;
  notes: string | null;
  reference_daily_amount: number | null;
  reference_daily_unit: string | null;
  reference_source: string | null;
  reference_notes: string | null;
  created_at: IsoDateTime;
  updated_at: IsoDateTime;
};

export type CreateNutrientInput = {
  canonical_name: string;
  display_name: string;
  kind: NutrientKind;
  default_unit: string;
  notes?: string | null;
  reference_daily_amount?: number | null;
  reference_daily_unit?: string | null;
  reference_source?: string | null;
  reference_notes?: string | null;
};

export type UpdateNutrientInput = {
  canonical_name: string;
  display_name: string;
  kind: NutrientKind;
  default_unit: string;
  notes?: string | null;
  reference_daily_amount?: number | null;
  reference_daily_unit?: string | null;
  reference_source?: string | null;
  reference_notes?: string | null;
};

export type ProductNutrient = {
  id: Id;
  product_id: Id;
  nutrient_id: Id;
  amount_per_serving: number;
  unit: string;
  label_daily_value_percent: number | null;
  label_reference_amount: number | null;
  label_reference_unit: string | null;
  source: string;
  notes: string | null;
  created_at: IsoDateTime;
  updated_at: IsoDateTime;
};

export type SaveProductNutrientInput = {
  nutrient_id: Id;
  amount_per_serving: number;
  unit: string;
  label_daily_value_percent?: number | null;
  label_reference_amount?: number | null;
  label_reference_unit?: string | null;
  source: string;
  notes?: string | null;
};
