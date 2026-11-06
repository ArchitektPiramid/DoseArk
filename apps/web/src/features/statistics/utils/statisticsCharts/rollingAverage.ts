// Daily totals and moving-average overlay options.
import type { EChartsOption } from "echarts";
import type { LineSeriesOption } from "echarts/charts";
import type { StatisticsDailyPoint } from "../statisticsSeries";
import { fillCalendarDays, rollingAverage, readPalette, round, formatNumber, formatDateLabel } from "./shared";

type RollingAverageOptionInput = {
  rows: StatisticsDailyPoint[];
  unit: string;
  startDate: string;
  endDate: string;
  windowDays: number;
  showDaily: boolean;
  showAverage: boolean;
  dailyLabel: string;
  averageLabel: string;
};

/** Combines daily totals and a trailing moving average. */
export function createRollingAverageOption({
  rows,
  unit,
  startDate,
  endDate,
  windowDays,
  showDaily,
  showAverage,
  dailyLabel,
  averageLabel,
}: RollingAverageOptionInput): EChartsOption {
  const palette = readPalette();
  const daily = fillCalendarDays(rows, startDate, endDate);
  const safeWindowDays = Math.max(1, Math.min(366, Math.round(windowDays)));
  const average = rollingAverage(daily.map((row) => row.amount), safeWindowDays);
  const showZoom = daily.length > 60;
  const series: LineSeriesOption[] = [];

  if (showDaily) {
    series.push({
      name: dailyLabel,
      type: "line",
      data: daily.map((row) => round(row.amount)),
      smooth: false,
      showSymbol: false,
      lineStyle: { width: 1, color: palette.muted, opacity: 0.6 },
      itemStyle: { color: palette.muted },
      emphasis: { focus: "series" },
    });
  }

  if (showAverage) {
    series.push({
      name: averageLabel,
      type: "line",
      data: average.map(round),
      smooth: false,
      showSymbol: false,
      lineStyle: { width: 2.6, color: palette.accent },
      itemStyle: { color: palette.accent },
      emphasis: { focus: "series" },
    });
  }

  return {
    animationDuration: 180,
    backgroundColor: "transparent",
    aria: { enabled: true },
    grid: {
      left: 62,
      right: 20,
      top: 12,
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
      data: daily.map((row) => row.date),
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
