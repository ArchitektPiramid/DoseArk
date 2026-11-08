import type { Dispatch, SetStateAction } from "react";
import { useTranslation } from "react-i18next";

import type { StatisticsComparisonOption } from "../hooks/useStatisticsComparison";
import {
  getComparisonRollingAverage,
  sanitizeComparisonRollingAverageDays,
  type StatisticsComparisonRollingAverages,
} from "../utils/statisticsComparison";

type Props = {
  options: StatisticsComparisonOption[];
  colors: Record<string, string>;
  rollingAverages: StatisticsComparisonRollingAverages;
  setRollingAverages: Dispatch<SetStateAction<StatisticsComparisonRollingAverages>>;
};

/** Controls the selected chart series independently of fetching and rendering. */
export function StatisticsComparisonSeriesSettings({
  options,
  colors,
  rollingAverages,
  setRollingAverages,
}: Props) {
  const { t } = useTranslation("statistics");
  if (options.length === 0) return null;

  return (
    <div className="mt-statistics-comparison-series-settings">
      {options.map((option) => {
        const average = getComparisonRollingAverage(rollingAverages, option.key);
        return (
          <div key={option.key} className="mt-statistics-comparison-series-setting">
            <span
              className="mt-statistics-comparison-series-setting__color"
              style={{ backgroundColor: colors[option.key] }}
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
  );
}
