import type { Product } from "@doseark/types";
import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";

import { Button, Panel } from "@/core/ui";
import type { StatisticsDateRange } from "../utils/statisticsRange";
import {
  STATISTICS_COMPARISON_COLORS,
  createDailyComparisonOption,
} from "../utils/statisticsCharts";
import { useStatisticsComparison } from "../hooks/useStatisticsComparison";
import {
  estimateLoadedComparisonData,
  formatApproxBytes,
  getComparisonRollingAverage,
  sanitizeComparisonRollingAverageDays,
  type StatisticsComparisonRollingAverages,
} from "../utils/statisticsComparison";
import { StatisticsChart } from "./StatisticsChart";

type Props = {
  enabled: boolean;
  range: StatisticsDateRange;
  products: Product[];
};

export function StatisticsComparisonView({ enabled, range, products }: Props) {
  const { t } = useTranslation("statistics");
  const comparison = useStatisticsComparison({ enabled, range, products });
  const [seriesColors, setSeriesColors] = useState<Record<string, string>>({});
  const [rollingAverages, setRollingAverages] = useState<StatisticsComparisonRollingAverages>({});

  const effectiveColors = useMemo(
    () => Object.fromEntries(
      comparison.selectedKeys.map((key, index) => [
        key,
        seriesColors[key] ?? STATISTICS_COMPARISON_COLORS[index % STATISTICS_COMPARISON_COLORS.length],
      ]),
    ),
    [comparison.selectedKeys, seriesColors],
  );

  const chartOption = useMemo(
    () => createDailyComparisonOption(
      comparison.series,
      range.startDate,
      range.endDate,
      effectiveColors,
      rollingAverages,
    ),
    [comparison.series, effectiveColors, range.endDate, range.startDate, rollingAverages],
  );

  const loadedData = useMemo(
    () => estimateLoadedComparisonData(comparison.series),
    [comparison.series],
  );

  const loading = comparison.status === "loading";
  const selectedLabel = comparison.selectedOptions.length === 0
    ? t("comparison.select")
    : comparison.selectedOptions.length === 1
      ? comparison.selectedOptions[0].label
      : t("comparison.selected", { count: comparison.selectedOptions.length });

  return (
    <Panel
      title={t("comparison.title")}
      className="mt-statistics-chart-card"
      action={comparison.series.length > 0 ? (
        <span className="mt-statistics-comparison-data-info">
          {t("comparison.loadedData", {
            series: comparison.series.length,
            points: loadedData.pointCount,
            size: formatApproxBytes(loadedData.approxBytes),
          })}
        </span>
      ) : undefined}
    >
      <div className="mt-statistics-comparison-toolbar">
        <div className="mt-statistics-comparison-controls">
          <div className="mt-tabs mt-tabs--compact" role="group" aria-label={t("subject.source")}>
            <Button
              type="button"
              variant={comparison.source === "product" ? "primary" : "quiet"}
              disabled={loading}
              onClick={() => comparison.changeSource("product")}
            >
              {t("subject.product")}
            </Button>
            <Button
              type="button"
              variant={comparison.source === "nutrient" ? "primary" : "quiet"}
              disabled={loading}
              onClick={() => comparison.changeSource("nutrient")}
            >
              {t("subject.nutrient")}
            </Button>
          </div>

          <details className="mt-statistics-multi-select">
            <summary>{selectedLabel}</summary>
            <div className="mt-statistics-multi-select__menu">
              <div className="mt-statistics-multi-select__header">
                <strong>{t("comparison.series")}</strong>
                {comparison.selectedKeys.length > 0 ? (
                  <button type="button" onClick={comparison.clearSelection}>
                    {t("comparison.clear")}
                  </button>
                ) : null}
              </div>
              <div className="mt-statistics-multi-select__options">
                {comparison.options.length === 0 ? (
                  <div className="mt-statistics-multi-select__empty">{t("comparison.emptyOptions")}</div>
                ) : (
                  comparison.options.map((option) => {
                    const checked = comparison.selectedKeys.includes(option.key);
                    const incompatible = Boolean(
                      comparison.selectedUnit &&
                      !checked &&
                      option.unit !== comparison.selectedUnit,
                    );

                    return (
                      <label
                        key={option.key}
                        className={incompatible ? "is-disabled" : undefined}
                        title={incompatible ? t("comparison.unitMismatch", { unit: comparison.selectedUnit }) : undefined}
                      >
                        <input
                          type="checkbox"
                          checked={checked}
                          disabled={loading || incompatible}
                          onChange={() => comparison.toggleSelection(option.key)}
                        />
                        <span>{option.label}</span>
                        <em>{option.unit}</em>
                        {checked ? (
                          <input
                            className="mt-statistics-series-color"
                            type="color"
                            value={effectiveColors[option.key]}
                            title={`${t("comparison.color")}: ${option.label}`}
                            aria-label={`${t("comparison.color")}: ${option.label}`}
                            onChange={(event) => {
                              const color = event.target.value;
                              setSeriesColors((current) => ({ ...current, [option.key]: color }));
                            }}
                          />
                        ) : null}
                      </label>
                    );
                  })
                )}
              </div>
            </div>
          </details>
        </div>

        {comparison.selectedOptions.length > 0 ? (
          <div className="mt-statistics-comparison-series-settings">
          {comparison.selectedOptions.map((option) => {
            const average = getComparisonRollingAverage(rollingAverages, option.key);

            return (
              <div key={option.key} className="mt-statistics-comparison-series-setting">
                <span
                  className="mt-statistics-comparison-series-setting__color"
                  style={{ backgroundColor: effectiveColors[option.key] }}
                  aria-hidden="true"
                />
                <span className="mt-statistics-comparison-series-setting__name" title={option.label}>
                  {option.label}
                </span>
                <label className="mt-statistics-comparison-average-toggle">
                  <input
                    type="checkbox"
                    checked={average.enabled}
                    onChange={(event) => {
                      const enabled = event.target.checked;
                      setRollingAverages((current) => ({
                        ...current,
                        [option.key]: { ...getComparisonRollingAverage(current, option.key), enabled },
                      }));
                    }}
                  />
                  <span>{t("comparison.rollingAverage")}</span>
                </label>
                <label className="mt-statistics-comparison-average-days">
                  <input
                    type="number"
                    min={1}
                    max={730}
                    step={1}
                    value={average.days}
                    disabled={!average.enabled}
                    aria-label={`${t("comparison.rollingAverageDays")}: ${option.label}`}
                    onChange={(event) => {
                      const days = sanitizeComparisonRollingAverageDays(Number(event.target.value));
                      setRollingAverages((current) => ({
                        ...current,
                        [option.key]: { ...getComparisonRollingAverage(current, option.key), days },
                      }));
                    }}
                  />
                  <span>{t("comparison.days")}</span>
                </label>
              </div>
            );
          })}
          </div>
        ) : null}
      </div>

      {comparison.error ? (
        <div className="mt-statistics-comparison-state mt-statistics-comparison-state--error">
          {comparison.error}
        </div>
      ) : comparison.series.length > 0 ? (
        <StatisticsChart option={chartOption} className="mt-statistics-chart mt-statistics-chart--comparison" />
      ) : (
        <div className="mt-statistics-chart-empty">
          {loading ? t("state.loading") : t("comparison.emptySelection")}
        </div>
      )}
    </Panel>
  );
}
