import { useMemo } from "react";
import { useTranslation } from "react-i18next";

import type { StatisticsDailyPoint } from "../utils/statisticsSeries";

type Props = {
  year: number;
  rows: StatisticsDailyPoint[];
  unit: string;
  showAmount?: boolean;
  showDoseCount?: boolean;
};

type HeatmapDay = {
  date: string;
  amount: number;
  doseCount: number;
  level: number;
};

type HeatmapMonth = {
  month: number;
  leadingDays: number;
  days: HeatmapDay[];
};

const DAY_MS = 86_400_000;
const WEEKDAY_LABELS = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

export function StatisticsYearHeatmap({
  year,
  rows,
  unit,
  showAmount = false,
  showDoseCount = false,
}: Props) {
  const { t, i18n } = useTranslation("statistics");
  const months = useMemo(() => buildHeatmapMonths(year, rows), [rows, year]);
  const numberFormatter = useMemo(
    () => new Intl.NumberFormat(i18n.language, { maximumFractionDigits: 3 }),
    [i18n.language],
  );
  const compactNumberFormatter = useMemo(
    () => new Intl.NumberFormat(i18n.language, {
      maximumFractionDigits: 2,
      notation: "compact",
    }),
    [i18n.language],
  );
  const dateFormatter = useMemo(
    () => new Intl.DateTimeFormat(i18n.language, { day: "numeric", month: "short", year: "numeric" }),
    [i18n.language],
  );
  const monthFormatter = useMemo(
    () => new Intl.DateTimeFormat(i18n.language, { month: "long" }),
    [i18n.language],
  );

  const valuesClass = showAmount || showDoseCount
    ? showAmount && showDoseCount
      ? "mt-statistics-heatmap--values-both"
      : showAmount
        ? "mt-statistics-heatmap--values-amount"
        : "mt-statistics-heatmap--values-count"
    : "";

  return (
    <div className="mt-statistics-heatmap-wrap">
      <div className={`mt-statistics-heatmap ${valuesClass}`.trim()} role="grid" aria-label={t("heatmap.gridLabel", { year })}>
        {months.map((month) => (
          <section className="mt-statistics-heatmap__month" key={month.month}>
            <div className="mt-statistics-heatmap__month-title">
              {monthFormatter.format(new Date(year, month.month, 1))}
            </div>

            <div className="mt-statistics-heatmap__month-weekdays" aria-hidden="true">
              {WEEKDAY_LABELS.map((label) => (
                <span key={label}>{t(`heatmap.weekdays.${label.toLowerCase()}`)}</span>
              ))}
            </div>

            <div className="mt-statistics-heatmap__month-days">
              {Array.from({ length: month.leadingDays }, (_, index) => (
                <span className="mt-statistics-heatmap__placeholder" key={`leading-${index}`} aria-hidden="true" />
              ))}

              {month.days.map((day) => {
                const date = parseLocalDate(day.date);
                const amount = numberFormatter.format(day.amount);
                const compactAmount = day.amount > 0 ? compactNumberFormatter.format(day.amount) : "";
                const title = day.doseCount > 0
                  ? t("heatmap.tooltip", {
                      date: dateFormatter.format(date),
                      amount,
                      unit,
                      count: day.doseCount,
                    })
                  : t("heatmap.tooltipEmpty", { date: dateFormatter.format(date) });

                return (
                  <span
                    key={day.date}
                    className={`mt-statistics-heatmap__cell mt-statistics-heatmap__cell--${day.level}`}
                    role="gridcell"
                    aria-label={title}
                    title={title}
                  >
                    {day.doseCount > 0 && showAmount ? (
                      <b className="mt-statistics-heatmap__cell-amount">{compactAmount}</b>
                    ) : null}
                    {day.doseCount > 0 && showDoseCount ? (
                      <small className="mt-statistics-heatmap__cell-count">×{day.doseCount}</small>
                    ) : null}
                  </span>
                );
              })}
            </div>
          </section>
        ))}
      </div>

      <div className="mt-statistics-heatmap__legend" aria-label={t("heatmap.legend.label")}>
        <span>{t("heatmap.legend.less")}</span>
        {[1, 2, 3, 4, 5].map((level) => (
          <i key={level} className={`mt-statistics-heatmap__cell mt-statistics-heatmap__cell--${level}`} />
        ))}
        <span>{t("heatmap.legend.more")}</span>
      </div>
    </div>
  );
}

function buildHeatmapMonths(year: number, rows: StatisticsDailyPoint[]): HeatmapMonth[] {
  const amountByDate = new Map<string, { amount: number; doseCount: number }>();

  for (const row of rows) {
    const current = amountByDate.get(row.date) ?? { amount: 0, doseCount: 0 };
    current.amount += row.amount;
    current.doseCount += row.doseCount;
    amountByDate.set(row.date, current);
  }

  const positiveAmounts = [...amountByDate.values()]
    .map((row) => row.amount)
    .filter((amount) => amount > 0)
    .sort((a, b) => a - b);
  const scaleMax = percentile(positiveAmounts, 0.9) || positiveAmounts.at(-1) || 0;

  return Array.from({ length: 12 }, (_, month) => {
    const first = new Date(year, month, 1);
    const last = new Date(year, month + 1, 0);
    const days: HeatmapDay[] = [];

    for (let cursor = new Date(first); cursor <= last; cursor = new Date(cursor.getTime() + DAY_MS)) {
      const date = toDateValue(cursor);
      const row = amountByDate.get(date);
      const amount = row?.amount ?? 0;

      days.push({
        date,
        amount,
        doseCount: row?.doseCount ?? 0,
        level: amount > 0 ? getLevel(amount, scaleMax) : 0,
      });
    }

    return {
      month,
      leadingDays: mondayIndex(first),
      days,
    };
  });
}

function getLevel(amount: number, scaleMax: number): number {
  if (amount <= 0 || scaleMax <= 0) return 0;
  const ratio = Math.min(amount / scaleMax, 1);
  return Math.min(5, Math.max(1, Math.ceil(ratio * 5)));
}

function percentile(values: number[], percentileValue: number): number {
  if (values.length === 0) return 0;
  const index = Math.min(values.length - 1, Math.floor((values.length - 1) * percentileValue));
  return values[index] ?? 0;
}

function mondayIndex(date: Date): number {
  return (date.getDay() + 6) % 7;
}

function toDateValue(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
}

function parseLocalDate(value: string): Date {
  const [year, month, day] = value.split("-").map(Number);
  return new Date(year, month - 1, day);
}
