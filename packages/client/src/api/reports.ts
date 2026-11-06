import type { DailyIntakeReport, NutrientTrendReport } from "@doseark/types";
import type { HttpTransport } from "../transport";

import type { NutrientTrendReportInput } from "../contracts";

export function getDailyIntakeReport(
  transport: HttpTransport,
  date: string,
  timezoneOffsetMinutes: number,
): Promise<DailyIntakeReport> {
  const params = new URLSearchParams({
    date,
    timezone_offset_minutes: String(timezoneOffsetMinutes),
  });

  return transport.get(`/api/reports/daily-intake?${params.toString()}`);
}

export function getNutrientTrendReport(
  transport: HttpTransport,
  input: NutrientTrendReportInput,
): Promise<NutrientTrendReport> {
  const params = new URLSearchParams({
    start_date: input.startDate,
    end_date: input.endDate,
    timezone_offset_minutes: String(input.timezoneOffsetMinutes),
    day_start_hour: String(input.dayStartHour ?? 0),
  });

  if (input.nutrientId) {
    params.set("nutrient_id", input.nutrientId);
  }
  if (input.productId) {
    params.set("product_id", input.productId);
  }
  if (input.unit) {
    params.set("unit", input.unit);
  }

  return transport.get(`/api/reports/nutrient-trend?${params.toString()}`);
}
