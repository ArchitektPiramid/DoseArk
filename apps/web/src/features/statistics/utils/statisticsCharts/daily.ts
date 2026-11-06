// Basic daily, monthly and hourly ECharts options.
import type { EChartsOption } from "echarts";
import type { StatisticsDailyPoint, StatisticsMonthlyPoint, StatisticsHourlyPoint } from "../statisticsSeries";
import { readPalette, round, formatNumber, formatDateLabel, formatMonthLabel } from "./shared";

/** Builds the daily intake timeline with optional horizontal zoom. */
export function createDailyTotalsOption(
  rows: StatisticsDailyPoint[],
  unit: string,
  title: string,
): EChartsOption {
  const palette = readPalette();
  const showZoom = rows.length > 60;

  return {
    animationDuration: 180,
    backgroundColor: "transparent",
    aria: { enabled: true },
    grid: {
      left: 62,
      right: 20,
      top: 20,
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
      data: rows.map((row) => row.date),
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
    series: [
      {
        name: title,
        type: "line",
        data: rows.map((row) => round(row.amount)),
        smooth: false,
        showSymbol: rows.length <= 45,
        symbolSize: 5,
        lineStyle: { width: 2, color: palette.accent },
        itemStyle: { color: palette.accent },
        areaStyle: { color: palette.accentSoft, opacity: 0.28 },
      },
    ],
  };
}

/** Builds the monthly totals bar chart. */
export function createMonthlyTotalsOption(
  rows: StatisticsMonthlyPoint[],
  unit: string,
  title: string,
): EChartsOption {
  const palette = readPalette();

  return {
    animationDuration: 180,
    backgroundColor: "transparent",
    aria: { enabled: true },
    grid: { left: 58, right: 18, top: 20, bottom: 38 },
    tooltip: {
      trigger: "axis",
      backgroundColor: palette.panel,
      borderColor: palette.border,
      textStyle: { color: palette.text },
      valueFormatter: (value) => `${formatNumber(Number(value))}${unit ? ` ${unit}` : ""}`,
    },
    xAxis: {
      type: "category",
      data: rows.map((row) => formatMonthLabel(row.month)),
      axisLine: { lineStyle: { color: palette.border } },
      axisLabel: { color: palette.muted, hideOverlap: true },
    },
    yAxis: {
      type: "value",
      min: 0,
      axisLabel: { color: palette.muted },
      splitLine: { lineStyle: { color: palette.border, opacity: 0.65 } },
    },
    series: [
      {
        name: title,
        type: "bar",
        data: rows.map((row) => round(row.amount)),
        barMaxWidth: 42,
        itemStyle: { color: palette.accent },
      },
    ],
  };
}

/** Builds the hourly intake distribution chart. */
export function createHourlyDistributionOption(
  rows: StatisticsHourlyPoint[],
  title: string,
): EChartsOption {
  const palette = readPalette();

  return {
    animationDuration: 180,
    backgroundColor: "transparent",
    aria: { enabled: true },
    grid: { left: 48, right: 18, top: 20, bottom: 38 },
    tooltip: {
      trigger: "axis",
      backgroundColor: palette.panel,
      borderColor: palette.border,
      textStyle: { color: palette.text },
      valueFormatter: (value) => `${Number(value)} doses`,
    },
    xAxis: {
      type: "category",
      data: rows.map((row) => `${String(row.hour).padStart(2, "0")}:00`),
      axisLine: { lineStyle: { color: palette.border } },
      axisLabel: {
        color: palette.muted,
        interval: 2,
      },
    },
    yAxis: {
      type: "value",
      min: 0,
      minInterval: 1,
      axisLabel: { color: palette.muted },
      splitLine: { lineStyle: { color: palette.border, opacity: 0.65 } },
    },
    series: [
      {
        name: title,
        type: "bar",
        data: rows.map((row) => row.doseCount),
        barMaxWidth: 24,
        itemStyle: { color: palette.accent },
      },
    ],
  };
}
