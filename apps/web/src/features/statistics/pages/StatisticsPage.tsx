import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";

import { ApiErrorNotice } from "@/app/system_status";
import { formatNumber } from "@/core/format";
import { PageHeader, Panel } from "@/core/ui";
import { StatisticsChart } from "../components/StatisticsChart";
import { StatisticsComparisonView } from "../components/StatisticsComparisonView";
import { StatisticsToolbar } from "../components/StatisticsToolbar";
import { StatisticsViewSelector, type StatisticsView } from "../components/StatisticsViewSelector";
import { StatisticsViewSettings } from "../components/StatisticsViewSettings";
import { StatisticsEmptyState } from "../components/StatisticsEmptyState";
import { getStatisticsPresetRange, getStatisticsRangeDayCount } from "../utils/statisticsRange";
import { StatisticsYearHeatmap } from "../components/StatisticsYearHeatmap";
import { StatisticsHalfLifeChart } from "../components/StatisticsHalfLifeChart";
import { StatisticsRollingAverageChart } from "../components/StatisticsRollingAverageChart";
import { useStatistics } from "../hooks/useStatistics";
import {
  createDailyTotalsOption,
  createHourlyDistributionOption,
  createMonthlyTotalsOption,
} from "../utils/statisticsCharts";
import { buildStatisticsDataset } from "../utils/statisticsSeries";

export function StatisticsPage() {
  const { t } = useTranslation("statistics");
  const statistics = useStatistics();
  const [view, setView] = useState<StatisticsView>("charts");
  const [halfLifeHours, setHalfLifeHours] = useState(8);
  const [halfLifeBarWidthPixels, setHalfLifeBarWidthPixels] = useState(1);
  const [halfLifeLineWidth, setHalfLifeLineWidth] = useState(2);
  const [halfLifeAverageDays, setHalfLifeAverageDays] = useState(3);
  const [showHalfLifeBars, setShowHalfLifeBars] = useState(true);
  const [showHalfLifeLine, setShowHalfLifeLine] = useState(true);
  const [showHalfLifeAverage, setShowHalfLifeAverage] = useState(true);
  const [rollingAverageDays, setRollingAverageDays] = useState(7);
  const [showRollingDaily, setShowRollingDaily] = useState(true);
  const [showRollingAverage, setShowRollingAverage] = useState(true);
  const [heatmapYear, setHeatmapYear] = useState(() => new Date().getFullYear());
  const [showHeatmapAmount, setShowHeatmapAmount] = useState(true);
  const [showHeatmapDoseCount, setShowHeatmapDoseCount] = useState(true);
  const dataset = useMemo(() => buildStatisticsDataset(statistics.report), [statistics.report]);
  const loading = statistics.status === "loading";

  const dailyOption = useMemo(
    () => createDailyTotalsOption(dataset.daily, dataset.unit, t("charts.daily.series")),
    [dataset.daily, dataset.unit, t],
  );
  const monthlyOption = useMemo(
    () => createMonthlyTotalsOption(dataset.monthly, dataset.unit, t("charts.monthly.series")),
    [dataset.monthly, dataset.unit, t],
  );
  const hourlyOption = useMemo(
    () => createHourlyDistributionOption(dataset.hourly, t("charts.hourly.series")),
    [dataset.hourly, t],
  );

  const hasData = dataset.summary.doseCount > 0;
  /** Switching to a year heatmap also switches the queried date range to that year. */
  function selectView(nextView: StatisticsView) {
    setView(nextView);
    if (nextView === "heatmap") {
      const nextRange = yearRange(heatmapYear);
      if (statistics.range.startDate !== nextRange.startDate || statistics.range.endDate !== nextRange.endDate) {
        statistics.setRange(nextRange);
      }
    }
  }

  return (
    <>
      <PageHeader title={t("page.title")} description={t("layout.pageDescription")} />
      <Panel className="mt-statistics-controls">
        <StatisticsToolbar
          range={statistics.range}
          source={statistics.source}
          products={statistics.products}
          nutrients={statistics.nutrients}
          selectedProductId={statistics.selectedProductId}
          selectedNutrientKey={statistics.selectedNutrientKey}
          loading={loading}
          showSubject={view !== "comparison"}
          showRange={view !== "heatmap"}
          showRefresh={view !== "comparison"}
          onRangeChange={statistics.setRange}
          onSourceChange={statistics.setSource}
          onProductChange={statistics.setSelectedProductId}
          onNutrientChange={statistics.setSelectedNutrientKey}
          onRefresh={statistics.refresh}
        />
      </Panel>

      <StatisticsViewSelector view={view} onChange={selectView} />

      {view !== "comparison" && hasData && !loading ? (
        <div className="mt-statistics-summary" aria-label={t("layout.summaryTitle")}>
          <div><span>{t("summary.entries")}</span><strong>{formatNumber(statistics.report?.events.length ?? 0)}</strong></div>
          <div><span>{t("summary.recordedDays")}</span><strong>{formatNumber(dataset.summary.recordedDays)}</strong></div>
          <div><span>{t("summary.total")}</span><strong>{formatNumber(dataset.summary.total)}{dataset.unit ? ` ${dataset.unit}` : ""}</strong></div>
        </div>
      ) : null}

      <StatisticsViewSettings
        view={view} loading={loading}
        heatmapYear={heatmapYear}
        onHeatmapYearChange={(year) => { setHeatmapYear(year); statistics.setRange(yearRange(year)); }}
        showHeatmapAmount={showHeatmapAmount} setShowHeatmapAmount={setShowHeatmapAmount}
        showHeatmapDoseCount={showHeatmapDoseCount} setShowHeatmapDoseCount={setShowHeatmapDoseCount}
        rollingAverageDays={rollingAverageDays} setRollingAverageDays={setRollingAverageDays}
        showRollingDaily={showRollingDaily} setShowRollingDaily={setShowRollingDaily}
        showRollingAverage={showRollingAverage} setShowRollingAverage={setShowRollingAverage}
        halfLifeHours={halfLifeHours} setHalfLifeHours={setHalfLifeHours}
        halfLifeBarWidthPixels={halfLifeBarWidthPixels} setHalfLifeBarWidthPixels={setHalfLifeBarWidthPixels}
        halfLifeLineWidth={halfLifeLineWidth} setHalfLifeLineWidth={setHalfLifeLineWidth}
        halfLifeAverageDays={halfLifeAverageDays} setHalfLifeAverageDays={setHalfLifeAverageDays}
        showHalfLifeBars={showHalfLifeBars} setShowHalfLifeBars={setShowHalfLifeBars}
        showHalfLifeLine={showHalfLifeLine} setShowHalfLifeLine={setShowHalfLifeLine}
        showHalfLifeAverage={showHalfLifeAverage} setShowHalfLifeAverage={setShowHalfLifeAverage}
      />

      {view !== "comparison" && statistics.error ? (
        <ApiErrorNotice error={statistics.error} onRetry={statistics.refresh} />
      ) : null}
      {view !== "comparison" && !statistics.error && (loading || !hasData) ? (
        <StatisticsEmptyState
          loading={loading}
          description={view === "heatmap" ? t("layout.emptyHeatmapDescription") : undefined}
          onExtendRange={view !== "heatmap" && getStatisticsRangeDayCount(statistics.range) < 365
            ? () => statistics.setRange(getStatisticsPresetRange(365)) : undefined}
        />
      ) : null}

      {view === "comparison" ? (
        <StatisticsComparisonView
          enabled
          range={statistics.range}
          products={statistics.products}
        />
      ) : !loading && hasData && !statistics.error ? (
        view === "heatmap" ? (
          <Panel title={t("heatmap.title")} className="mt-statistics-chart-card">
            <StatisticsYearHeatmap
              year={heatmapYear}
              rows={dataset.daily}
              unit={dataset.unit}
              showAmount={showHeatmapAmount}
              showDoseCount={showHeatmapDoseCount}
            />
          </Panel>
        ) : view === "rollingAverage" ? (
          <Panel title={t("rollingAverage.title")} className="mt-statistics-chart-card">
            <StatisticsRollingAverageChart
              rows={dataset.daily}
              unit={dataset.unit}
              startDate={statistics.range.startDate}
              endDate={statistics.range.endDate}
              windowDays={rollingAverageDays}
              showDaily={showRollingDaily}
              showAverage={showRollingAverage}
            />
          </Panel>
        ) : view === "halfLife" ? (
          <Panel title={t("halfLife.title")} className="mt-statistics-chart-card">
            <StatisticsHalfLifeChart
              events={statistics.report?.events ?? []}
              unit={dataset.unit}
              startDate={statistics.range.startDate}
              endDate={statistics.range.endDate}
              halfLifeHours={halfLifeHours}
              barWidthPixels={halfLifeBarWidthPixels}
              movingAverageDays={halfLifeAverageDays}
              lineWidth={halfLifeLineWidth}
              showBars={showHalfLifeBars}
              showLine={showHalfLifeLine}
              showAverage={showHalfLifeAverage}
            />
          </Panel>
        ) : (
          <div className="mt-statistics-charts" aria-busy={loading}>
            <Panel title={t("charts.daily.title")} className="mt-statistics-chart-card mt-statistics-chart-card--wide">
              <p className="mt-statistics-chart-description">{t("charts.daily.description")}</p>
              <StatisticsChart option={dailyOption} className="mt-statistics-chart mt-statistics-chart--daily" />
            </Panel>
            <Panel title={t("charts.monthly.title")} className="mt-statistics-chart-card">
              <p className="mt-statistics-chart-description">{t("charts.monthly.description")}</p>
              <StatisticsChart option={monthlyOption} className="mt-statistics-chart" />
            </Panel>
            <Panel title={t("charts.hourly.title")} className="mt-statistics-chart-card">
              <p className="mt-statistics-chart-description">{t("charts.hourly.description")}</p>
              <StatisticsChart option={hourlyOption} className="mt-statistics-chart" />
            </Panel>
          </div>
        )
      ) : null}
    </>
  );
}

/** Uses an entire calendar year for the year heatmap API query. */
function yearRange(year: number) {
  return { startDate: `${year}-01-01`, endDate: `${year}-12-31` };
}
