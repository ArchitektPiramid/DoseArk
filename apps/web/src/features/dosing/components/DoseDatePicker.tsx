import { CalendarDays, ChevronLeft, ChevronRight } from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";

import {
  createCalendarDays,
  formatDoseDisplayDate,
  isToday,
  type CalendarMonth,
  type DoseTimeFields,
} from "../utils/doseTime";

const WEEKDAY_REFERENCE_DATES = [5, 6, 7, 8, 9, 10, 11].map(
  (day) => new Date(Date.UTC(2026, 0, day)),
);

export function DoseDatePicker(props: {
  value: DoseTimeFields;
  onChange: (patch: Partial<DoseTimeFields>) => void;
}) {
  const { t, i18n } = useTranslation("dosing");
  const rootRef = useRef<HTMLDivElement>(null);
  const [open, setOpen] = useState(false);
  const [calendarMonth, setCalendarMonth] = useState<CalendarMonth>({
    year: props.value.year,
    month: props.value.month,
  });
  const locale = i18n.resolvedLanguage ?? i18n.language;
  const calendarDays = useMemo(
    () => createCalendarDays(calendarMonth.year, calendarMonth.month),
    [calendarMonth.month, calendarMonth.year],
  );
  const weekdayLabels = useMemo(
    () =>
      WEEKDAY_REFERENCE_DATES.map((date) =>
        new Intl.DateTimeFormat(locale, { weekday: "short" }).format(date),
      ),
    [locale],
  );

  useEffect(() => {
    if (open) {
      setCalendarMonth({ year: props.value.year, month: props.value.month });
    }
  }, [open, props.value.month, props.value.year]);

  useEffect(() => {
    if (!open) return;

    function closeOnOutsidePointer(event: PointerEvent) {
      if (!rootRef.current?.contains(event.target as Node)) setOpen(false);
    }

    function closeOnEscape(event: KeyboardEvent) {
      if (event.key === "Escape") setOpen(false);
    }

    document.addEventListener("pointerdown", closeOnOutsidePointer);
    document.addEventListener("keydown", closeOnEscape);
    return () => {
      document.removeEventListener("pointerdown", closeOnOutsidePointer);
      document.removeEventListener("keydown", closeOnEscape);
    };
  }, [open]);

  function moveMonth(offset: number) {
    const next = new Date(calendarMonth.year, calendarMonth.month - 1 + offset, 1);
    setCalendarMonth({ year: next.getFullYear(), month: next.getMonth() + 1 });
  }

  function selectDay(day: number) {
    props.onChange({ year: calendarMonth.year, month: calendarMonth.month, day });
    setOpen(false);
  }

  function selectToday() {
    const today = new Date();
    props.onChange({ year: today.getFullYear(), month: today.getMonth() + 1, day: today.getDate() });
    setOpen(false);
  }

  const monthTitle = new Intl.DateTimeFormat(locale, {
    month: "long",
    year: "numeric",
  }).format(new Date(calendarMonth.year, calendarMonth.month - 1, 1));

  return (
    <div className="mt-dose-date-time-field" ref={rootRef}>
      <span className="mt-label">{t("form.date")}</span>
      <button
        type="button"
        className={open ? "mt-dose-calendar-trigger is-open" : "mt-dose-calendar-trigger"}
        aria-expanded={open}
        aria-haspopup="dialog"
        onClick={() => setOpen((current) => !current)}
      >
        <CalendarDays size={17} aria-hidden="true" />
        <span>{formatDoseDisplayDate(props.value)}</span>
      </button>

      {open ? (
        <div className="mt-dose-calendar" role="dialog" aria-label={t("form.openCalendar")}>
          <div className="mt-dose-calendar__header">
            <button type="button" className="mt-dose-calendar__nav" aria-label={t("form.previousMonth")} onClick={() => moveMonth(-1)}>
              <ChevronLeft size={17} aria-hidden="true" />
            </button>
            <strong>{monthTitle}</strong>
            <button type="button" className="mt-dose-calendar__nav" aria-label={t("form.nextMonth")} onClick={() => moveMonth(1)}>
              <ChevronRight size={17} aria-hidden="true" />
            </button>
          </div>

          <div className="mt-dose-calendar__weekdays" aria-hidden="true">
            {weekdayLabels.map((label, index) => (
              <span key={`${label}-${index}`}>{label}</span>
            ))}
          </div>

          <div className="mt-dose-calendar__days">
            {calendarDays.map((day, index) => {
              if (day === null) return <span key={`empty-${index}`} className="mt-dose-calendar__empty" />;

              const selected =
                props.value.year === calendarMonth.year &&
                props.value.month === calendarMonth.month &&
                props.value.day === day;
              const today = isToday(calendarMonth.year, calendarMonth.month, day);

              return (
                <button
                  key={day}
                  type="button"
                  className={[
                    "mt-dose-calendar__day",
                    selected ? "is-selected" : "",
                    today ? "is-today" : "",
                  ].filter(Boolean).join(" ")}
                  aria-pressed={selected}
                  onClick={() => selectDay(day)}
                >
                  {day}
                </button>
              );
            })}
          </div>

          <button type="button" className="mt-dose-calendar__today" onClick={selectToday}>
            {t("form.today")}
          </button>
        </div>
      ) : null}
    </div>
  );
}
