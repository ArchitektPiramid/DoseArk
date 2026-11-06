import type { NutrientTrendReport } from "@doseark/types";

export type StatisticsDailyPoint = {
  date: string;
  amount: number;
  doseCount: number;
};

export type StatisticsMonthlyPoint = {
  month: string;
  amount: number;
  doseCount: number;
};

export type StatisticsHourlyPoint = {
  hour: number;
  doseCount: number;
};

export type StatisticsSummary = {
  total: number;
  doseCount: number;
  recordedDays: number;
  averagePerRecordedDay: number;
};

export type StatisticsDataset = {
  unit: string;
  daily: StatisticsDailyPoint[];
  monthly: StatisticsMonthlyPoint[];
  hourly: StatisticsHourlyPoint[];
  summary: StatisticsSummary;
};

export function buildStatisticsDataset(report: NutrientTrendReport | null): StatisticsDataset {
  if (!report) {
    return emptyDataset();
  }

  const daily = report.days.map((day) => ({
    date: day.date,
    amount: day.amount_total,
    doseCount: day.dose_count,
  }));

  const monthlyMap = new Map<string, StatisticsMonthlyPoint>();
  for (const day of daily) {
    const month = day.date.slice(0, 7);
    const current = monthlyMap.get(month) ?? { month, amount: 0, doseCount: 0 };
    current.amount += day.amount;
    current.doseCount += day.doseCount;
    monthlyMap.set(month, current);
  }

  const hourly = Array.from({ length: 24 }, (_, hour) => ({ hour, doseCount: 0 }));
  for (const event of report.events) {
    const date = new Date(event.taken_at);
    if (Number.isNaN(date.getTime())) continue;
    hourly[date.getHours()].doseCount += 1;
  }

  const total = daily.reduce((sum, day) => sum + day.amount, 0);
  const doseCount = daily.reduce((sum, day) => sum + day.doseCount, 0);
  const recordedDays = daily.reduce((sum, day) => sum + (day.amount > 0 ? 1 : 0), 0);

  return {
    unit: report.selected_unit ?? report.days.find((day) => day.unit)?.unit ?? "",
    daily,
    monthly: Array.from(monthlyMap.values()),
    hourly,
    summary: {
      total,
      doseCount,
      recordedDays,
      averagePerRecordedDay: recordedDays > 0 ? total / recordedDays : 0,
    },
  };
}

function emptyDataset(): StatisticsDataset {
  return {
    unit: "",
    daily: [],
    monthly: [],
    hourly: Array.from({ length: 24 }, (_, hour) => ({ hour, doseCount: 0 })),
    summary: {
      total: 0,
      doseCount: 0,
      recordedDays: 0,
      averagePerRecordedDay: 0,
    },
  };
}
