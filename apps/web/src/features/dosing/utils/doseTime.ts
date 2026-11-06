export type DoseTimeMode = "now" | "custom";

export type DoseTimeFields = {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
};

export type DoseTimeSelection = {
  mode: DoseTimeMode;
  custom: DoseTimeFields;
};

export type CalendarMonth = {
  year: number;
  month: number;
};

export const HOURS = Array.from({ length: 24 }, (_, index) => index);
export const MINUTES = Array.from({ length: 12 }, (_, index) => index * 5);

export function patchDoseTimeFields(
  current: DoseTimeFields,
  patch: Partial<DoseTimeFields>,
): DoseTimeFields {
  const next = { ...current, ...patch };
  const maxDay = getDaysInMonth(next.year, next.month);
  if (next.day > maxDay) next.day = maxDay;
  return next;
}

export function createCalendarDays(year: number, month: number): Array<number | null> {
  const firstWeekdaySundayBased = new Date(year, month - 1, 1).getDay();
  const firstWeekdayMondayBased = (firstWeekdaySundayBased + 6) % 7;
  const daysInMonth = getDaysInMonth(year, month);

  return [
    ...Array.from({ length: firstWeekdayMondayBased }, () => null),
    ...Array.from({ length: daysInMonth }, (_, index) => index + 1),
  ];
}

export function formatDoseDisplayDate(value: DoseTimeFields): string {
  return `${padTwo(value.day)}.${padTwo(value.month)}.${value.year}`;
}

export function padTwo(value: number): string {
  return String(value).padStart(2, "0");
}

export function getDaysInMonth(year: number, month: number): number {
  return new Date(year, month, 0).getDate();
}

export function isToday(year: number, month: number, day: number): boolean {
  const today = new Date();
  return today.getFullYear() === year && today.getMonth() + 1 === month && today.getDate() === day;
}

export function createInitialDoseTimeSelection(): DoseTimeSelection {
  const now = new Date();

  return {
    mode: "now",
    custom: {
      year: now.getFullYear(),
      month: now.getMonth() + 1,
      day: now.getDate(),
      hour: now.getHours(),
      minute: roundMinuteToFive(now.getMinutes()),
    },
  };
}

export function doseTimeSelectionToIso(selection: DoseTimeSelection): string {
  if (selection.mode === "now") return new Date().toISOString();

  return new Date(
    selection.custom.year,
    selection.custom.month - 1,
    selection.custom.day,
    selection.custom.hour,
    selection.custom.minute,
    0,
    0,
  ).toISOString();
}

export function doseTimeSelectionFromIso(value: string): DoseTimeSelection {
  const date = new Date(value);

  return {
    mode: "custom",
    custom: {
      year: date.getFullYear(),
      month: date.getMonth() + 1,
      day: date.getDate(),
      hour: date.getHours(),
      minute: roundMinuteToFive(date.getMinutes()),
    },
  };
}

function roundMinuteToFive(minute: number): number {
  return Math.min(Math.round(minute / 5) * 5, 55);
}
