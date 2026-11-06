import type { NutrientTrendEvent } from "@doseark/types";

export type HalfLifePoint = { time: number; amount: number };
export type HalfLifeBar = { start: number; amount: number; count: number };

export type HalfLifeChartData = {
  curve: HalfLifePoint[];
  movingAverage: HalfLifePoint[];
  bars: HalfLifeBar[];
  start: number;
  end: number;
  lineMax: number;
  barMax: number;
};

const HOUR_MS = 3_600_000;
const DAY_MS = 86_400_000;

export function calculatePixelBucketHours(
  startDate: string,
  endDate: string,
  renderedSvgWidth: number,
  chartWidth: number,
  plotWidth: number,
  barWidthPixels: number,
): number {
  const { start, end } = parseRange(startDate, endDate);
  const renderedPlotWidth = Math.max(1, renderedSvgWidth * (plotWidth / chartWidth));
  const pixelCount = Math.max(1, Math.round(barWidthPixels));
  return Math.max((end - start) / renderedPlotWidth / HOUR_MS * pixelCount, 1 / 60);
}

export function buildHalfLifeChartData(
  events: NutrientTrendEvent[],
  halfLifeHours: number,
  bucketHours: number,
  movingAverageDays: number,
  startDate: string,
  endDate: string,
): HalfLifeChartData {
  const doses = events
    .map((event) => ({ time: new Date(event.taken_at).getTime(), amount: Number(event.amount) }))
    .filter((event) => Number.isFinite(event.time) && Number.isFinite(event.amount) && event.amount > 0)
    .sort((a, b) => a.time - b.time);

  const range = parseRange(startDate, endDate);
  const empty = {
    curve: [] as HalfLifePoint[],
    movingAverage: [] as HalfLifePoint[],
    bars: [] as HalfLifeBar[],
    start: range.start,
    end: range.end,
    lineMax: 0,
    barMax: 0,
  };

  if (!doses.length || halfLifeHours <= 0 || bucketHours <= 0 || movingAverageDays <= 0) {
    return empty;
  }

  const halfMs = halfLifeHours * HOUR_MS;
  const start = range.start;
  const end = range.end;
  const span = Math.max(HOUR_MS, end - start);
  const step = Math.max(30 * 60_000, Math.min(3 * HOUR_MS, span / 1100));

  const curve: HalfLifePoint[] = [];
  let remaining = 0;
  let doseIndex = 0;
  let previousTime = start;

  while (doseIndex < doses.length && doses[doseIndex].time < start) {
    doseIndex += 1;
  }

  for (let time = start; time <= end; time += step) {
    while (doseIndex < doses.length && doses[doseIndex].time <= time) {
      const dose = doses[doseIndex];
      remaining *= Math.pow(0.5, (dose.time - previousTime) / halfMs);
      remaining += dose.amount;
      previousTime = dose.time;
      doseIndex += 1;
    }

    remaining *= Math.pow(0.5, (time - previousTime) / halfMs);
    previousTime = time;
    curve.push({ time, amount: remaining });
  }

  if (curve[curve.length - 1]?.time !== end) {
    remaining *= Math.pow(0.5, (end - previousTime) / halfMs);
    curve.push({ time: end, amount: remaining });
  }

  const bucketMs = bucketHours * HOUR_MS;
  const buckets = new Map<number, HalfLifeBar>();

  for (const dose of doses) {
    if (dose.time < start || dose.time > end) continue;
    const bucketStart = start + Math.floor((dose.time - start) / bucketMs) * bucketMs;
    const bucket = buckets.get(bucketStart) ?? { start: bucketStart, amount: 0, count: 0 };
    bucket.amount += dose.amount;
    bucket.count += 1;
    buckets.set(bucketStart, bucket);
  }

  const bars = [...buckets.values()].sort((a, b) => a.start - b.start);
  const averageWindowMs = movingAverageDays * DAY_MS;
  const movingAverage: HalfLifePoint[] = [];
  let left = 0;
  let sum = 0;

  for (let right = 0; right < curve.length; right += 1) {
    sum += curve[right].amount;
    while (curve[right].time - curve[left].time > averageWindowMs && left < right) {
      sum -= curve[left].amount;
      left += 1;
    }
    movingAverage.push({ time: curve[right].time, amount: sum / (right - left + 1) });
  }

  return {
    curve,
    movingAverage,
    bars,
    start,
    end,
    lineMax: niceMax(Math.max(...curve.map((point) => point.amount), ...movingAverage.map((point) => point.amount), 1)),
    barMax: niceMax(Math.max(...bars.map((bar) => bar.amount), 1)),
  };
}

export function pathForHalfLife(
  points: HalfLifePoint[],
  x: (value: number) => number,
  y: (value: number) => number,
): string {
  return points
    .map((point, index) => `${index ? "L" : "M"}${x(point.time).toFixed(1)},${y(point.amount).toFixed(1)}`)
    .join(" ");
}

function parseRange(startDate: string, endDate: string) {
  const start = new Date(`${startDate}T00:00:00`).getTime();
  const end = new Date(`${endDate}T23:59:59.999`).getTime();

  if (!Number.isFinite(start) || !Number.isFinite(end) || end <= start) {
    const fallbackStart = Date.now() - DAY_MS;
    return { start: fallbackStart, end: fallbackStart + DAY_MS };
  }

  return { start, end };
}

function niceMax(value: number) {
  const magnitude = 10 ** Math.floor(Math.log10(value));
  return Math.ceil((value / magnitude) * 1.08 * 4) / 4 * magnitude;
}
