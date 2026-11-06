const DAY_MS = 86_400_000;

export type StatisticsDateRange = {
  startDate: string;
  endDate: string;
};

export function getStatisticsPresetRange(daysBack: number): StatisticsDateRange {
  const today = new Date();
  const start = new Date(today.getTime() - (daysBack - 1) * DAY_MS);

  return {
    startDate: toDateInputValue(start),
    endDate: toDateInputValue(today),
  };
}


export function shiftStatisticsRange(
  range: StatisticsDateRange,
  days: number,
): StatisticsDateRange {
  return {
    startDate: shiftDateInputValue(range.startDate, days),
    endDate: shiftDateInputValue(range.endDate, days),
  };
}

export function getStatisticsRangeDayCount(range: StatisticsDateRange): number {
  const start = parseDateInputValue(range.startDate);
  const end = parseDateInputValue(range.endDate);
  if (!start || !end) return 0;
  return Math.floor((end.getTime() - start.getTime()) / DAY_MS) + 1;
}

export function isStatisticsRangeValid(range: StatisticsDateRange): boolean {
  const dayCount = getStatisticsRangeDayCount(range);
  return dayCount >= 1 && dayCount <= 732;
}

function toDateInputValue(date: Date): string {
  const offsetMs = date.getTimezoneOffset() * 60_000;
  return new Date(date.getTime() - offsetMs).toISOString().slice(0, 10);
}

function parseDateInputValue(value: string): Date | null {
  if (!value) return null;
  const date = new Date(`${value}T00:00:00`);
  return Number.isNaN(date.getTime()) ? null : date;
}
function shiftDateInputValue(value: string, days: number): string {
  const match = /^(\d{4})-(\d{2})-(\d{2})$/.exec(value);
  if (!match) return value;

  const [, year, month, day] = match;
  const date = new Date(Date.UTC(Number(year), Number(month) - 1, Number(day) + days));
  return date.toISOString().slice(0, 10);
}

