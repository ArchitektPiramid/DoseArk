export type StatisticsComparisonRollingAverage = {
  enabled: boolean;
  days: number;
};

export type StatisticsComparisonRollingAverages = Record<
  string,
  StatisticsComparisonRollingAverage
>;

export const DEFAULT_COMPARISON_ROLLING_AVERAGE_DAYS = 7;

export function getComparisonRollingAverage(
  settings: StatisticsComparisonRollingAverages,
  key: string,
): StatisticsComparisonRollingAverage {
  return settings[key] ?? {
    enabled: false,
    days: DEFAULT_COMPARISON_ROLLING_AVERAGE_DAYS,
  };
}

export function sanitizeComparisonRollingAverageDays(value: number): number {
  if (!Number.isFinite(value)) {
    return DEFAULT_COMPARISON_ROLLING_AVERAGE_DAYS;
  }

  return Math.max(1, Math.min(730, Math.round(value)));
}

export function estimateLoadedComparisonData(
  series: readonly { rows: readonly unknown[] }[],
): { pointCount: number; approxBytes: number } {
  const pointCount = series.reduce((total, item) => total + item.rows.length, 0);

  try {
    const serialized = JSON.stringify(series);
    const approxBytes = typeof TextEncoder === "undefined"
      ? serialized.length * 2
      : new TextEncoder().encode(serialized).byteLength;

    return { pointCount, approxBytes };
  } catch {
    return { pointCount, approxBytes: 0 };
  }
}

export function formatApproxBytes(bytes: number): string {
  if (!Number.isFinite(bytes) || bytes <= 0) return "0 B";
  if (bytes < 1024) return `${Math.round(bytes)} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(bytes < 10 * 1024 ? 1 : 0)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}
