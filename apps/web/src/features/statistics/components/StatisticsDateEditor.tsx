import { ChevronLeft, ChevronRight } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Button, Field, TextInput } from "@/core/ui";
import {
  getStatisticsRangeDayCount,
  isStatisticsRangeValid,
  shiftStatisticsRange,
  type StatisticsDateRange,
} from "../utils/statisticsRange";

type Props = {
  range: StatisticsDateRange;
  draftRange: StatisticsDateRange;
  loading: boolean;
  onDraftChange: (next: StatisticsDateRange) => void;
  onApply: () => void;
  onRangeChange: (next: StatisticsDateRange) => void;
};

/** Displays custom date edits and adjacent-period navigation. */
export function StatisticsDateEditor(props: Props) {
  const { t } = useTranslation("statistics");
  const dayCount = getStatisticsRangeDayCount(props.draftRange);
  const rangeValid = isStatisticsRangeValid(props.draftRange);
  const rangeDirty =
    props.draftRange.startDate !== props.range.startDate ||
    props.draftRange.endDate !== props.range.endDate;
  const rangeMessage =
    dayCount < 1
      ? t("range.invalid")
      : dayCount > 732
        ? t("range.tooLong")
        : t("range.days", { count: dayCount });

  return (
    <div className="mt-statistics-date-editor">
      <div className="mt-statistics-date-fields">
        <Field label={t("range.from")}>
          <TextInput
            type="date"
            value={props.draftRange.startDate}
            onChange={(event) =>
              props.onDraftChange({ ...props.draftRange, startDate: event.target.value })
            }
          />
        </Field>
        <Field label={t("range.to")}>
          <TextInput
            type="date"
            value={props.draftRange.endDate}
            onChange={(event) =>
              props.onDraftChange({ ...props.draftRange, endDate: event.target.value })
            }
          />
        </Field>
        <Button
          type="button"
          variant="primary"
          disabled={props.loading || !rangeValid || !rangeDirty}
          onClick={props.onApply}
        >
          {t("range.apply")}
        </Button>
      </div>
      <div className="mt-statistics-date-navigation">
        <span className={rangeValid ? "mt-statistics-range-info" : "mt-statistics-range-error"}>
          {rangeMessage}
        </span>
        <div role="group" aria-label={t("range.shift")}>
          <Button
            type="button"
            variant="quiet"
            disabled={props.loading}
            title={t("layout.previousPeriod")}
            aria-label={t("layout.previousPeriod")}
            onClick={() =>
              props.onRangeChange(
                shiftStatisticsRange(props.range, -getStatisticsRangeDayCount(props.range)),
              )
            }
          >
            <ChevronLeft size={15} aria-hidden="true" /> {t("layout.previousPeriod")}
          </Button>
          <Button
            type="button"
            variant="quiet"
            disabled={props.loading}
            title={t("layout.nextPeriod")}
            aria-label={t("layout.nextPeriod")}
            onClick={() =>
              props.onRangeChange(
                shiftStatisticsRange(props.range, getStatisticsRangeDayCount(props.range)),
              )
            }
          >
            {t("layout.nextPeriod")} <ChevronRight size={15} aria-hidden="true" />
          </Button>
        </div>
      </div>
    </div>
  );
}
