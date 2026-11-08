import type { NutrientTrendOption } from "@doseark/types";

export type StatisticsNutrientSelection = {
  nutrientId: string;
  unit: string;
};

const SEPARATOR = "::";

export function buildStatisticsNutrientKey(selection: StatisticsNutrientSelection): string {
  return `${selection.nutrientId}${SEPARATOR}${selection.unit}`;
}

export function parseStatisticsNutrientKey(key: string | null): StatisticsNutrientSelection | null {
  if (!key) return null;
  const separatorIndex = key.indexOf(SEPARATOR);
  if (separatorIndex <= 0) return null;

  const nutrientId = key.slice(0, separatorIndex);
  const unit = key.slice(separatorIndex + SEPARATOR.length);
  if (!nutrientId || !unit) return null;

  return { nutrientId, unit };
}

/** Retains a selected nutrient only if both ID and unit still exist in the report. */
export function resolveStatisticsNutrient(options: NutrientTrendOption[], key: string | null) {
  const preferred = parseStatisticsNutrientKey(key);
  const nutrient =
    options.find(
      (item) => item.nutrient_id === preferred?.nutrientId && item.unit === preferred?.unit,
    ) ?? options[0] ?? null;
  return {
    nutrient,
    key: nutrient ? buildStatisticsNutrientKey({ nutrientId: nutrient.nutrient_id, unit: nutrient.unit }) : null,
  };
}
