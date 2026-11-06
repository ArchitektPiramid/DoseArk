import type { Id, IsoDateTime } from "../common";
import type { NutrientKind } from "../catalog/nutrients";

export type DoseLogStatus = "active" | "cancelled";

export type DoseLog = {
  id: Id;
  product_id: Id;
  inventory_lot_id: Id | null;
  taken_at: IsoDateTime;
  units_taken: number;
  unit: string;
  serving_multiplier: number;
  inventory_quantity_before: number | null;
  inventory_quantity_after: number | null;
  inventory_units_initial: number | null;
  reason: string | null;
  notes: string | null;
  status: DoseLogStatus;
  cancelled_at: IsoDateTime | null;
  cancellation_reason: string | null;
  created_at: IsoDateTime;
  updated_at: IsoDateTime;
};

export type DoseLogListInput = {
  limit: number;
  offset: number;
  product_id?: Id | null;
};

export type DoseLogPage = {
  items: DoseLog[];
  total: number;
  limit: number;
  offset: number;
};

export type CreateDoseLogInput = {
  product_id: Id;
  inventory_lot_id?: Id | null;
  taken_at: IsoDateTime;
  units_taken: number;
  unit: string;
  serving_multiplier: number;
  reason?: string | null;
  notes?: string | null;
};

export type UpdateDoseLogInput = {
  product_id: Id;
  inventory_lot_id?: Id | null;
  taken_at: IsoDateTime;
  units_taken: number;
  unit: string;
  serving_multiplier: number;
  reason?: string | null;
  notes?: string | null;
};

export type DoseLogNutrient = {
  id: Id;
  dose_log_id: Id;
  nutrient_id: Id;
  nutrient_canonical_name: string;
  nutrient_display_name: string;
  nutrient_kind: NutrientKind;
  amount: number;
  unit: string;
  reference_daily_amount: number | null;
  reference_daily_unit: string | null;
  reference_source: string | null;
  percent_of_reference: number | null;
  created_at: IsoDateTime;
};
