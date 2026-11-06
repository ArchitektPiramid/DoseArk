import type { EChartsOption } from "echarts";
import type { DashboardDoseActivityPoint } from "./dashboardActivity";

/** Builds the compact 14-day activity chart using the application's theme tokens. */
export function createDashboardDoseActivityOption(rows: DashboardDoseActivityPoint[]): EChartsOption {
  const styles = typeof document === "undefined" ? null : getComputedStyle(document.documentElement);
  const color = (name: string, fallback: string) => styles?.getPropertyValue(name).trim() || fallback;
  const muted = color("--mt-text-muted", "#8b98a8");
  const border = color("--mt-border", "#253241");
  const accent = color("--mt-accent", "#6f91b8");

  return {
    animationDuration: 180,
    backgroundColor: "transparent",
    aria: { enabled: true },
    grid: { left: 26, right: 10, top: 18, bottom: 30 },
    tooltip: {
      trigger: "axis",
      backgroundColor: color("--mt-bg-panel", "#0d131b"),
      borderColor: border,
      textStyle: { color: color("--mt-text", "#d7dee8") },
      valueFormatter: (value) => `${Number(value)} doses`,
    },
    xAxis: {
      type: "category",
      data: rows.map(({ date }) => {
        const [, month, day] = date.split("-");
        return `${day}.${month}`;
      }),
      axisLine: { lineStyle: { color: border } },
      axisTick: { show: false },
      axisLabel: { color: muted, interval: 1 },
    },
    yAxis: {
      type: "value",
      min: 0,
      minInterval: 1,
      axisLabel: { color: muted },
      splitLine: { lineStyle: { color: border, opacity: .45 } },
    },
    series: [{
      type: "bar",
      data: rows.map(({ count }) => count),
      barMaxWidth: 24,
      itemStyle: { color: accent, borderRadius: [2, 2, 0, 0] },
    }],
  };
}
