import type { NutrientTrendReportInput } from "@doseark/client";
import type { StatisticsDateRange } from "./statisticsRange";

export type StatisticsReportSubject =
  | { kind: "product"; productId: string; unit: string }
  | { kind: "nutrient"; nutrientId: string; unit: string }
  | { kind: "all" };

/** Builds the same backend report query for single-series and comparison views. */
export function createStatisticsReportRequest(
  range: StatisticsDateRange,
  subject: StatisticsReportSubject,
  timezoneOffsetMinutes = new Date().getTimezoneOffset(),
): NutrientTrendReportInput {
  return {
    startDate: range.startDate,
    endDate: range.endDate,
    timezoneOffsetMinutes,
    productId: subject.kind === "product" ? subject.productId : null,
    nutrientId: subject.kind === "nutrient" ? subject.nutrientId : null,
    unit: subject.kind === "all" ? null : subject.unit,
    dayStartHour: 0,
  };
}
