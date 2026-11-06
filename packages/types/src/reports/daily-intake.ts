import type { Id, IsoDateTime } from "../common";
import type { NutrientKind } from "../catalog/nutrients";
import type { DoseLog } from "../dosing/dose-logs";

export type DailyIntakeReportRow = {
  nutrient_id: Id;
  nutrient_canonical_name: string;
  nutrient_display_name: string;
  nutrient_kind: NutrientKind;
  amount_total: number;
  unit: string;
  reference_daily_amount: number | null;
  reference_daily_unit: string | null;
  reference_source: string | null;
  percent_of_reference_total: number | null;
  dose_count: number;
};

export type DailyIntakeReport = {
  date: string;
  start_at: IsoDateTime;
  end_at: IsoDateTime;
  rows: DailyIntakeReportRow[];
  dose_logs: DoseLog[];
};
