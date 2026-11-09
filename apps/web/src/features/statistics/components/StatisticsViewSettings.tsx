import { Settings2 } from "lucide-react";
import { useTranslation } from "react-i18next";

import { SelectInput } from "@/core/ui";
import type { StatisticsView } from "./StatisticsViewSelector";

type Props = {
  view: StatisticsView;
  loading: boolean;
  heatmapYear: number;
  onHeatmapYearChange: (year: number) => void;
  showHeatmapAmount: boolean;
  setShowHeatmapAmount: (value: boolean) => void;
  showHeatmapDoseCount: boolean;
  setShowHeatmapDoseCount: (value: boolean) => void;
  rollingAverageDays: number;
  setRollingAverageDays: (value: number) => void;
  showRollingDaily: boolean;
  setShowRollingDaily: (value: boolean) => void;
  showRollingAverage: boolean;
  setShowRollingAverage: (value: boolean) => void;
  halfLifeHours: number;
  setHalfLifeHours: (value: number) => void;
  halfLifeBarWidthPixels: number;
  setHalfLifeBarWidthPixels: (value: number) => void;
  halfLifeLineWidth: number;
  setHalfLifeLineWidth: (value: number) => void;
  halfLifeAverageDays: number;
  setHalfLifeAverageDays: (value: number) => void;
  showHalfLifeBars: boolean;
  setShowHalfLifeBars: (value: boolean) => void;
  showHalfLifeLine: boolean;
  setShowHalfLifeLine: (value: boolean) => void;
  showHalfLifeAverage: boolean;
  setShowHalfLifeAverage: (value: boolean) => void;
};

/** Shows only the settings relevant to the current analysis; advanced options stay optional. */
export function StatisticsViewSettings(props: Props) {
  const { t } = useTranslation("statistics");
  if (props.view === "charts" || props.view === "comparison") return null;

  return (
    <section className="mt-statistics-view-settings" aria-label={t("layout.analysisSettings")}>
      <div className="mt-statistics-view-settings__main">
        {props.view === "heatmap" ? (
          <label className="mt-statistics-year-select">
            <span>{t("heatmap.year")}</span>
            <SelectInput
              value={props.heatmapYear}
              disabled={props.loading}
              onChange={(event) => props.onHeatmapYearChange(Number(event.target.value))}
            >
              {availableYears().map((year) => (
                <option key={year} value={year}>
                  {year}
                </option>
              ))}
            </SelectInput>
          </label>
        ) : props.view === "rollingAverage" ? (
          <NumberControl
            label={t("rollingAverage.window")}
            value={props.rollingAverageDays}
            suffix={t("rollingAverage.days")}
            min={1}
            max={366}
            step={1}
            disabled={props.loading}
            onChange={props.setRollingAverageDays}
          />
        ) : (
          <NumberControl
            label={t("halfLife.input")}
            value={props.halfLifeHours}
            suffix={t("halfLife.hours")}
            min={0.1}
            step={0.1}
            disabled={props.loading}
            onChange={props.setHalfLifeHours}
          />
        )}
        <details className="mt-statistics-view-settings__advanced">
          <summary>
            <Settings2 size={14} aria-hidden="true" />
            {t("layout.advancedSettings")}
          </summary>
          <div className="mt-statistics-view-settings__advanced-body">
            {props.view === "heatmap" ? (
              <div className="mt-statistics-view-settings__toggles">
                <ToggleControl
                  label={t("heatmap.showAmount")}
                  checked={props.showHeatmapAmount}
                  onChange={props.setShowHeatmapAmount}
                />
                <ToggleControl
                  label={t("heatmap.showDoseCount")}
                  checked={props.showHeatmapDoseCount}
                  onChange={props.setShowHeatmapDoseCount}
                />
              </div>
            ) : props.view === "rollingAverage" ? (
              <div className="mt-statistics-view-settings__toggles">
                <ToggleControl
                  label={t("rollingAverage.toggleDaily")}
                  checked={props.showRollingDaily}
                  onChange={props.setShowRollingDaily}
                />
                <ToggleControl
                  label={t("rollingAverage.toggleAverage")}
                  checked={props.showRollingAverage}
                  onChange={props.setShowRollingAverage}
                />
              </div>
            ) : (
              <>
                <div className="mt-statistics-view-settings__numbers">
                  <NumberControl
                    label={t("halfLife.barWidth")}
                    value={props.halfLifeBarWidthPixels}
                    suffix={t("halfLife.pixels")}
                    min={1}
                    max={24}
                    step={1}
                    disabled={props.loading}
                    onChange={props.setHalfLifeBarWidthPixels}
                  />
                  <NumberControl
                    label={t("halfLife.lineWidth")}
                    value={props.halfLifeLineWidth}
                    suffix={t("halfLife.pixels")}
                    min={0.5}
                    max={8}
                    step={0.5}
                    disabled={props.loading}
                    onChange={props.setHalfLifeLineWidth}
                  />
                  <NumberControl
                    label={t("halfLife.movingAverage")}
                    value={props.halfLifeAverageDays}
                    suffix={t("halfLife.days")}
                    min={0.25}
                    step={0.25}
                    disabled={props.loading}
                    onChange={props.setHalfLifeAverageDays}
                  />
                </div>
                <div className="mt-statistics-view-settings__toggles">
                  <ToggleControl
                    label={t("halfLife.toggleDoses")}
                    checked={props.showHalfLifeBars}
                    onChange={props.setShowHalfLifeBars}
                  />
                  <ToggleControl
                    label={t("halfLife.toggleRemaining")}
                    checked={props.showHalfLifeLine}
                    onChange={props.setShowHalfLifeLine}
                  />
                  <ToggleControl
                    label={t("halfLife.toggleAverage")}
                    checked={props.showHalfLifeAverage}
                    onChange={props.setShowHalfLifeAverage}
                  />
                </div>
              </>
            )}
          </div>
        </details>
      </div>
    </section>
  );
}

type NumberProps = {
  label: string;
  value: number;
  suffix: string;
  min: number;
  max?: number;
  step: number;
  disabled: boolean;
  onChange: (value: number) => void;
};

/** Constrains the numeric input to the range accepted by the chart. */
function NumberControl({ label, value, suffix, min, max, step, disabled, onChange }: NumberProps) {
  return (
    <label className="mt-statistics-half-life-control">
      <span>{label}</span>
      <span className="mt-statistics-half-life-control__input">
        <input
          type="number"
          min={min}
          max={max}
          step={step}
          value={value}
          disabled={disabled}
          onChange={(event) => {
            const parsed = Number(event.target.value) || min;
            onChange(Math.min(max ?? Number.POSITIVE_INFINITY, Math.max(min, parsed)));
          }}
        />
        <em>{suffix}</em>
      </span>
    </label>
  );
}

function ToggleControl({
  label,
  checked,
  onChange,
}: {
  label: string;
  checked: boolean;
  onChange: (value: boolean) => void;
}) {
  return (
    <label className="mt-statistics-half-life-toggle">
      <input
        type="checkbox"
        checked={checked}
        onChange={(event) => onChange(event.target.checked)}
      />
      <span>{label}</span>
    </label>
  );
}

function availableYears(): number[] {
  const current = new Date().getFullYear();
  return Array.from({ length: 8 }, (_, index) => current - index);
}
