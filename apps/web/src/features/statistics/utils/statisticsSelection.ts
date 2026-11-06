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
