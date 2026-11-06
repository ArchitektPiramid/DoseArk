import type { Id, IsoDateTime } from "../common";
import type { NutrientKind } from "../catalog/nutrients";

export type NutrientTrendOption = {
  nutrient_id: Id;
  nutrient_canonical_name: string;
  nutrient_display_name: string;
  nutrient_kind: NutrientKind;
  unit: string;
  amount_total: number;
  dose_count: number;
  recorded_day_count: number;
};

export type NutrientTrendEvent = {
  dose_log_id: Id;
  product_id: Id;
  product_name: string;
  product_brand_name: string | null;
  taken_at: IsoDateTime;
  amount: number;
  unit: string;
  dose_units_taken: number;
  dose_unit: string;
  inventory_lot_number: string | null;
  notes: string | null;
};

export type NutrientTrendDay = {
  date: string;
  amount_total: number;
  unit: string;
  reference_daily_amount: number | null;
  reference_daily_unit: string | null;
  reference_source: string | null;
  percent_of_reference_total: number | null;
  dose_count: number;
};

export type NutrientTrendReport = {
  start_date: string;
  end_date: string;
  start_at: IsoDateTime;
  end_at: IsoDateTime;
  selected_nutrient_id: Id | null;
  selected_product_id: Id | null;
  selected_unit: string | null;
  day_start_hour: number;
  nutrients: NutrientTrendOption[];
  days: NutrientTrendDay[];
  events: NutrientTrendEvent[];
};
