import { useMemo } from "react";
import { useTranslation } from "react-i18next";

import { StatisticsChart } from "./StatisticsChart";
import { createRollingAverageOption } from "../utils/statisticsCharts";
import type { StatisticsDailyPoint } from "../utils/statisticsSeries";

type Props = {
  rows: StatisticsDailyPoint[];
  unit: string;
  startDate: string;
  endDate: string;
  windowDays: number;
  showDaily: boolean;
  showAverage: boolean;
};

export function StatisticsRollingAverageChart({
  rows,
  unit,
  startDate,
  endDate,
  windowDays,
  showDaily,
  showAverage,
}: Props) {
  const { t } = useTranslation("statistics");

  const option = useMemo(
    () =>
      createRollingAverageOption({
        rows,
        unit,
        startDate,
        endDate,
        windowDays,
        showDaily,
        showAverage,
        dailyLabel: t("rollingAverage.daily"),
        averageLabel: t("rollingAverage.average", { days: windowDays }),
      }),
    [rows, unit, startDate, endDate, windowDays, showDaily, showAverage, t],
  );

  return (
    <div className="mt-statistics-rolling-average">
      <div className="mt-statistics-rolling-average__legend" aria-hidden="true">
        {showDaily ? (
          <span>
            <i className="mt-statistics-rolling-average__legend-daily" />
            {t("rollingAverage.daily")}
          </span>
        ) : null}
        {showAverage ? (
          <span>
            <i className="mt-statistics-rolling-average__legend-average" />
            {t("rollingAverage.average", { days: windowDays })}
          </span>
        ) : null}
      </div>
      <StatisticsChart option={option} className="mt-statistics-chart mt-statistics-chart--rolling" />
    </div>
  );
}
