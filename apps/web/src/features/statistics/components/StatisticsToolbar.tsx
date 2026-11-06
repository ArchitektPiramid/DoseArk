import type { NutrientTrendOption, Product } from "@doseark/types";
import { CalendarDays, ChevronDown, ChevronLeft, ChevronRight, RefreshCw } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";

import { Button, Field, SelectInput, TextInput } from "@/core/ui";
import type { StatisticsSource } from "../hooks/useStatistics";
import {
  getStatisticsPresetRange,
  getStatisticsRangeDayCount,
  isStatisticsRangeValid,
  shiftStatisticsRange,
  type StatisticsDateRange,
} from "../utils/statisticsRange";
import { buildStatisticsNutrientKey } from "../utils/statisticsSelection";

type Props = {
  range: StatisticsDateRange;
  source: StatisticsSource;
  products: Product[];
  nutrients: NutrientTrendOption[];
  selectedProductId: string | null;
  selectedNutrientKey: string | null;
  loading: boolean;
  showSubject?: boolean;
  showRefresh?: boolean;
  showRange?: boolean;
  onRangeChange: (range: StatisticsDateRange) => void;
  onSourceChange: (source: StatisticsSource) => void;
  onProductChange: (productId: string) => void;
  onNutrientChange: (key: string) => void;
  onRefresh: () => void;
};

const PRESETS = [30, 90, 180, 365, 730] as const;

/** Keeps the subject and time period separate so each control has one clear purpose. */
export function StatisticsToolbar(props: Props) {
  const { t } = useTranslation("statistics");
  const [draftRange, setDraftRange] = useState(props.range);
  const [customOpen, setCustomOpen] = useState(false);
  const dayCount = getStatisticsRangeDayCount(draftRange);
  const rangeValid = isStatisticsRangeValid(draftRange);
  const rangeDirty = draftRange.startDate !== props.range.startDate || draftRange.endDate !== props.range.endDate;
  const activePreset = PRESETS.find((days) => {
    const preset = getStatisticsPresetRange(days);
    return preset.startDate === props.range.startDate && preset.endDate === props.range.endDate;
  });

  useEffect(() => {
    setDraftRange(props.range);
  }, [props.range.startDate, props.range.endDate]);

  const rangeMessage = useMemo(() => {
    if (dayCount < 1) return t("range.invalid");
    if (dayCount > 732) return t("range.tooLong");
    return t("range.days", { count: dayCount });
  }, [dayCount, t]);

  /** Applies a preset immediately and closes the advanced date editor. */
  function applyPreset(days: number) {
    props.onRangeChange(getStatisticsPresetRange(days));
    setCustomOpen(false);
  }

  return (
    <div className="mt-statistics-toolbar">
      {props.showSubject !== false ? (
        <section className="mt-statistics-toolbar__section" aria-label={t("layout.subjectTitle")}>
          <div className="mt-statistics-toolbar__heading">
            <strong>{t("layout.subjectTitle")}</strong>
            <span>{t("layout.subjectHint")}</span>
          </div>
          <div className="mt-statistics-toolbar__subject">
            <div className="mt-statistics-segmented" role="group" aria-label={t("subject.source")}>
              {(["product", "nutrient"] as const).map((source) => (
                <Button key={source} type="button" variant={props.source === source ? "primary" : "quiet"}
                  aria-pressed={props.source === source} disabled={props.loading}
                  onClick={() => props.onSourceChange(source)}>
                  {t(`subject.${source}`)}
                </Button>
              ))}
            </div>
            <div className="mt-statistics-toolbar__subject-select">
              {props.source === "product" ? (
                <Field label={t("layout.selectedProduct")}>
                  <SelectInput value={props.selectedProductId ?? ""} disabled={props.loading || props.products.length === 0}
                    onChange={(event) => props.onProductChange(event.target.value)}>
                    {props.products.length === 0 ? <option value="">{t("subject.noProducts")}</option> : null}
                    {props.products.map((product) => (
                      <option key={product.id} value={product.id}>{product.name} · {product.serving_unit}</option>
                    ))}
                  </SelectInput>
                </Field>
              ) : (
                <Field label={t("layout.selectedNutrient")}>
                  <SelectInput value={props.selectedNutrientKey ?? ""} disabled={props.loading || props.nutrients.length === 0}
                    onChange={(event) => props.onNutrientChange(event.target.value)}>
                    {props.nutrients.length === 0 ? <option value="">{t("subject.noNutrients")}</option> : null}
                    {props.nutrients.map((nutrient) => {
                      const key = buildStatisticsNutrientKey({ nutrientId: nutrient.nutrient_id, unit: nutrient.unit });
                      return <option key={key} value={key}>{nutrient.nutrient_display_name} · {nutrient.unit}</option>;
                    })}
                  </SelectInput>
                </Field>
              )}
            </div>
          </div>
        </section>
      ) : null}

      {props.showRange === false && props.showRefresh !== false ? (
        <div className="mt-statistics-toolbar__standalone-refresh">
          <Button type="button" variant="quiet" disabled={props.loading} onClick={props.onRefresh}>
            <RefreshCw size={15} aria-hidden="true" />{t("actions.refresh")}
          </Button>
        </div>
      ) : null}

      {props.showRange !== false ? (
        <section className="mt-statistics-toolbar__section" aria-label={t("layout.periodTitle")}>
          <div className="mt-statistics-toolbar__heading">
            <strong>{t("layout.periodTitle")}</strong>
            <span>{t("layout.periodHint")}</span>
          </div>
          <div className="mt-statistics-period-bar">
            <div className="mt-statistics-period-presets" role="group" aria-label={t("range.quick")}>
              {PRESETS.map((days) => (
                <Button key={days} type="button" className="mt-statistics-period-button"
                  variant={activePreset === days ? "primary" : "quiet"} aria-pressed={activePreset === days}
                  disabled={props.loading} onClick={() => applyPreset(days)}>
                  {t(`range.presets.${days}`)}
                </Button>
              ))}
            </div>
            <div className="mt-statistics-period-actions">
              <Button type="button" className="mt-statistics-custom-trigger"
                variant={customOpen || activePreset === undefined ? "primary" : "quiet"}
                aria-expanded={customOpen} disabled={props.loading}
                onClick={() => setCustomOpen((open) => !open)}>
                <CalendarDays size={15} aria-hidden="true" />{t("layout.customRange")}
                <ChevronDown size={13} aria-hidden="true" />
              </Button>
              {props.showRefresh !== false ? (
                <Button type="button" variant="quiet" className="mt-statistics-refresh"
                  disabled={props.loading} onClick={props.onRefresh}>
                  <RefreshCw size={15} aria-hidden="true" />{t("actions.refresh")}
                </Button>
              ) : null}
            </div>
          </div>
          {!activePreset && !customOpen ? (
            <span className="mt-statistics-period-current">{props.range.startDate} → {props.range.endDate}</span>
          ) : null}
          {customOpen ? (
            <div className="mt-statistics-date-editor">
              <div className="mt-statistics-date-fields">
                <Field label={t("range.from")}>
                  <TextInput type="date" value={draftRange.startDate}
                    onChange={(event) => setDraftRange((current) => ({ ...current, startDate: event.target.value }))} />
                </Field>
                <Field label={t("range.to")}>
                  <TextInput type="date" value={draftRange.endDate}
                    onChange={(event) => setDraftRange((current) => ({ ...current, endDate: event.target.value }))} />
                </Field>
                <Button type="button" variant="primary" disabled={props.loading || !rangeValid || !rangeDirty}
                  onClick={() => { props.onRangeChange(draftRange); setCustomOpen(false); }}>
                  {t("range.apply")}
                </Button>
              </div>
              <div className="mt-statistics-date-navigation">
                <span className={rangeValid ? "mt-statistics-range-info" : "mt-statistics-range-error"}>{rangeMessage}</span>
                <div role="group" aria-label={t("range.shift")}>
                  <Button type="button" variant="quiet" disabled={props.loading}
                    title={t("layout.previousPeriod")} aria-label={t("layout.previousPeriod")}
                    onClick={() => props.onRangeChange(shiftStatisticsRange(props.range, -getStatisticsRangeDayCount(props.range)))}>
                    <ChevronLeft size={15} aria-hidden="true" /> {t("layout.previousPeriod")}
                  </Button>
                  <Button type="button" variant="quiet" disabled={props.loading}
                    title={t("layout.nextPeriod")} aria-label={t("layout.nextPeriod")}
                    onClick={() => props.onRangeChange(shiftStatisticsRange(props.range, getStatisticsRangeDayCount(props.range)))}>
                    {t("layout.nextPeriod")} <ChevronRight size={15} aria-hidden="true" />
                  </Button>
                </div>
              </div>
            </div>
          ) : null}
        </section>
      ) : null}
    </div>
  );
}
