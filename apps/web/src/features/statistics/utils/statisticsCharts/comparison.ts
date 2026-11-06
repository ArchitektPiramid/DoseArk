// Multi-series comparison chart, including optional rolling averages.
import type { EChartsOption } from "echarts";
import type { LineSeriesOption } from "echarts/charts";
import type { StatisticsDailyPoint } from "../statisticsSeries";
import { getComparisonRollingAverage, sanitizeComparisonRollingAverageDays, type StatisticsComparisonRollingAverages } from "../statisticsComparison";
import { fillCalendarDays, rollingAverage, readPalette, round, formatNumber, formatDateLabel } from "./shared";

/** Default colors for the comparison series. */
export const STATISTICS_COMPARISON_COLORS = [
  "#6db4ff",
  "#ef7a6a",
  "#8ecf73",
  "#e6b85c",
  "#b58cff",
  "#5fd0c8",
  "#f28cc8",
  "#a4b0be",
] as const;

export type StatisticsComparisonChartSeries = {
  key: string;
  label: string;
  unit: string;
  rows: StatisticsDailyPoint[];
};

/** Aligns dates and draws selected series with optional moving averages. */
export function createDailyComparisonOption(
  inputSeries: StatisticsComparisonChartSeries[],
  startDate: string,
  endDate: string,
  seriesColors: Record<string, string> = {},
  rollingAverages: StatisticsComparisonRollingAverages = {},
): EChartsOption {
  const palette = readPalette();
  const aligned = inputSeries.map((item) => ({
    ...item,
    rows: fillCalendarDays(item.rows, startDate, endDate),
  }));
  const dates = aligned[0]?.rows.map((row) => row.date) ?? [];
  const unit = aligned[0]?.unit ?? "";
  const showZoom = dates.length > 60;
  const colors = aligned.map(
    (item, index) =>
      seriesColors[item.key] ??
      STATISTICS_COMPARISON_COLORS[index % STATISTICS_COMPARISON_COLORS.length],
  );

  const series: LineSeriesOption[] = [];

  aligned.forEach((item, index) => {
    const color = colors[index];
    series.push({
      name: item.label,
      type: "line",
      data: item.rows.map((row) => round(row.amount)),
      smooth: false,
      showSymbol: dates.length <= 45,
      symbolSize: 4,
      lineStyle: { width: 2, color },
      itemStyle: { color },
      emphasis: { focus: "series" },
    });

    const averageSettings = getComparisonRollingAverage(rollingAverages, item.key);
    if (!averageSettings.enabled) return;

    const days = sanitizeComparisonRollingAverageDays(averageSettings.days);
    const average = rollingAverage(item.rows.map((row) => row.amount), days);
    series.push({
      name: `${item.label} · MA ${days}d`,
      type: "line",
      data: average.map(round),
      smooth: false,
      showSymbol: false,
      lineStyle: { width: 2, type: "dashed", color, opacity: 0.9 },
      itemStyle: { color },
      emphasis: { focus: "series" },
      z: 3,
    });
  });

  return {
    animationDuration: 180,
    backgroundColor: "transparent",
    aria: { enabled: true },
    color: colors,
    legend: {
      type: "scroll",
      top: 0,
      left: 8,
      right: 8,
      textStyle: { color: palette.muted, fontSize: 11 },
      pageTextStyle: { color: palette.muted },
      pageIconColor: palette.accent,
      pageIconInactiveColor: palette.border,
    },
    grid: {
      left: 62,
      right: 20,
      top: 42,
      bottom: showZoom ? 68 : 42,
    },
    tooltip: {
      trigger: "axis",
      backgroundColor: palette.panel,
      borderColor: palette.border,
      textStyle: { color: palette.text },
      valueFormatter: (value) => `${formatNumber(Number(value))}${unit ? ` ${unit}` : ""}`,
    },
    xAxis: {
      type: "category",
      data: dates,
      boundaryGap: false,
      axisLine: { lineStyle: { color: palette.border } },
      axisLabel: {
        color: palette.muted,
        hideOverlap: true,
        formatter: (value: string) => formatDateLabel(value),
      },
    },
    yAxis: {
      type: "value",
      min: 0,
      axisLabel: { color: palette.muted },
      splitLine: { lineStyle: { color: palette.border, opacity: 0.65 } },
    },
    dataZoom: showZoom
      ? [
          { type: "inside", start: 0, end: 100 },
          {
            type: "slider",
            height: 20,
            bottom: 10,
            start: 0,
            end: 100,
            borderColor: palette.border,
            backgroundColor: palette.panel,
            fillerColor: palette.accentSoft,
            textStyle: { color: palette.muted },
          },
        ]
      : undefined,
    series,
  };
}
