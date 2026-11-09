import { CalendarDays, ChevronDown, RefreshCw } from "lucide-react";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import { Button } from "@/core/ui";
import { getStatisticsPresetRange } from "../utils/statisticsRange";
import { StatisticsDateEditor } from "./StatisticsDateEditor";
import type { StatisticsToolbarProps } from "./StatisticsToolbar";

type Props = Pick<
  StatisticsToolbarProps,
  "range" | "loading" | "showRange" | "showRefresh" | "onRangeChange" | "onRefresh"
>;

const PRESETS = [30, 90, 180, 365, 730] as const;

/** Keeps custom date drafts local until explicitly applied. */
export function StatisticsPeriodControls(props: Props) {
  const { t } = useTranslation("statistics");
  const [draftRange, setDraftRange] = useState(props.range);
  const [customOpen, setCustomOpen] = useState(false);
  const activePreset = PRESETS.find((days) => {
    const preset = getStatisticsPresetRange(days);
    return preset.startDate === props.range.startDate && preset.endDate === props.range.endDate;
  });

  useEffect(() => {
    setDraftRange(props.range);
  }, [props.range.startDate, props.range.endDate]);

  function applyPreset(days: number) {
    props.onRangeChange(getStatisticsPresetRange(days));
    setCustomOpen(false);
  }

  if (props.showRange === false) {
    return props.showRefresh !== false ? (
      <div className="mt-statistics-toolbar__standalone-refresh">
        <Button type="button" variant="quiet" disabled={props.loading} onClick={props.onRefresh}>
          <RefreshCw size={15} aria-hidden="true" />
          {t("actions.refresh")}
        </Button>
      </div>
    ) : null;
  }

  return (
    <section className="mt-statistics-toolbar__section" aria-label={t("layout.periodTitle")}>
      <div className="mt-statistics-toolbar__heading">
        <strong>{t("layout.periodTitle")}</strong>
        <span>{t("layout.periodHint")}</span>
      </div>
      <div className="mt-statistics-period-bar">
        <div className="mt-statistics-period-presets" role="group" aria-label={t("range.quick")}>
          {PRESETS.map((days) => (
            <Button
              key={days}
              type="button"
              className="mt-statistics-period-button"
              variant={activePreset === days ? "primary" : "quiet"}
              aria-pressed={activePreset === days}
              disabled={props.loading}
              onClick={() => applyPreset(days)}
            >
              {t(`range.presets.${days}`)}
            </Button>
          ))}
        </div>
        <div className="mt-statistics-period-actions">
          <Button
            type="button"
            className="mt-statistics-custom-trigger"
            variant={customOpen || activePreset === undefined ? "primary" : "quiet"}
            aria-expanded={customOpen}
            disabled={props.loading}
            onClick={() => setCustomOpen((open) => !open)}
          >
            <CalendarDays size={15} aria-hidden="true" />
            {t("layout.customRange")}
            <ChevronDown size={13} aria-hidden="true" />
          </Button>
          {props.showRefresh !== false ? (
            <Button
              type="button"
              variant="quiet"
              className="mt-statistics-refresh"
              disabled={props.loading}
              onClick={props.onRefresh}
            >
              <RefreshCw size={15} aria-hidden="true" />
              {t("actions.refresh")}
            </Button>
          ) : null}
        </div>
      </div>
      {!activePreset && !customOpen ? (
        <span className="mt-statistics-period-current">
          {props.range.startDate} → {props.range.endDate}
        </span>
      ) : null}
      {customOpen ? (
        <StatisticsDateEditor
          range={props.range}
          draftRange={draftRange}
          loading={props.loading}
          onDraftChange={setDraftRange}
          onApply={() => {
            props.onRangeChange(draftRange);
            setCustomOpen(false);
          }}
          onRangeChange={props.onRangeChange}
        />
      ) : null}
    </section>
  );
}
