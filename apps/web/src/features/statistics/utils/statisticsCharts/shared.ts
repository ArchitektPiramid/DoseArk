// Shared formatting, calendar alignment and CSS palette helpers for all chart builders.
import type { StatisticsDailyPoint } from "../statisticsSeries";

export type Palette = {
  text: string;
  muted: string;
  border: string;
  panel: string;
  accent: string;
  accentSoft: string;
};

/** Fills missing calendar days so chart series share a stable date axis. */
export function fillCalendarDays(
  rows: StatisticsDailyPoint[],
  startDate: string,
  endDate: string,
): StatisticsDailyPoint[] {
  const byDate = new Map<string, StatisticsDailyPoint>();

  for (const row of rows) {
    const current = byDate.get(row.date) ?? { date: row.date, amount: 0, doseCount: 0 };
    current.amount += row.amount;
    current.doseCount += row.doseCount;
    byDate.set(row.date, current);
  }

  const start = parseDateOnly(startDate);
  const end = parseDateOnly(endDate);
  if (!start || !end || start > end) {
    return [...byDate.values()].sort((a, b) => a.date.localeCompare(b.date));
  }

  const result: StatisticsDailyPoint[] = [];
  const cursor = new Date(start);

  while (cursor <= end) {
    const date = formatDateOnly(cursor);
    result.push(byDate.get(date) ?? { date, amount: 0, doseCount: 0 });
    cursor.setUTCDate(cursor.getUTCDate() + 1);
  }

  return result;
}

/** Computes a trailing mean in one pass through the points. */
export function rollingAverage(values: number[], windowDays: number): number[] {
  const result: number[] = [];
  let sum = 0;

  for (let index = 0; index < values.length; index += 1) {
    sum += values[index];
    if (index >= windowDays) {
      sum -= values[index - windowDays];
    }
    const divisor = Math.min(index + 1, windowDays);
    result.push(sum / divisor);
  }

  return result;
}

function parseDateOnly(value: string): Date | null {
  if (!/^\d{4}-\d{2}-\d{2}$/.test(value)) return null;
  const parsed = new Date(`${value}T00:00:00Z`);
  return Number.isNaN(parsed.getTime()) ? null : parsed;
}

function formatDateOnly(value: Date): string {
  return value.toISOString().slice(0, 10);
}

/** Reads theme colors, with safe defaults outside a browser. */
export function readPalette(): Palette {
  if (typeof document === "undefined") {
    return {
      text: "#d7dee8",
      muted: "#8b98a8",
      border: "#253241",
      panel: "#0d131b",
      accent: "#6f91b8",
      accentSoft: "#223448",
    };
  }

  const styles = getComputedStyle(document.documentElement);
  return {
    text: readCss(styles, "--mt-text", "#d7dee8"),
    muted: readCss(styles, "--mt-text-muted", "#8b98a8"),
    border: readCss(styles, "--mt-border", "#253241"),
    panel: readCss(styles, "--mt-bg-panel", "#0d131b"),
    accent: readCss(styles, "--mt-accent", "#6f91b8"),
    accentSoft: readCss(styles, "--mt-accent-muted", "#223448"),
  };
}

function readCss(styles: CSSStyleDeclaration, name: string, fallback: string): string {
  return styles.getPropertyValue(name).trim() || fallback;
}

export function formatDateLabel(value: string): string {
  const [, month, day] = value.split("-");
  return month && day ? `${day}.${month}` : value;
}

export function formatMonthLabel(value: string): string {
  const [year, month] = value.split("-");
  return year && month ? `${month}.${year.slice(2)}` : value;
}

export function formatNumber(value: number): string {
  return new Intl.NumberFormat(undefined, { maximumFractionDigits: 2 }).format(value);
}

export function round(value: number): number {
  return Math.round(value * 1000) / 1000;
}
