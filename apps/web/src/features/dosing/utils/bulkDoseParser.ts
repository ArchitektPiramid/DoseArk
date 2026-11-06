export type BulkDoseAmountMode = "mg" | "units";

export type BulkDoseParsedEntry = {
  key: string;
  line: number;
  date: string;
  time: string;
  amount: number;
  amountModeOverride: BulkDoseAmountMode | null;
};

export type BulkDoseParseError = {
  line: number;
  message: string;
};

export type BulkDoseParseResult = {
  entries: BulkDoseParsedEntry[];
  errors: BulkDoseParseError[];
};

const DATE_ONLY_PATTERN = /^\s*[-–—]?\s*(\d{4})[.-](\d{1,2})[.-](\d{1,2})(?:\s+.*)?$/;
const INLINE_PATTERN =
  /^\s*(\d{4})[.-](\d{1,2})[.-](\d{1,2})\s+(\d{1,2}):(\d{2})\s*(?:[-–—]\s*)?([0-9]+(?:[.,][0-9]+)?)\s*(mg|u|unit|units|tab|tabs|tablet|tablets)?\s*$/i;
const ENTRY_PATTERN =
  /^\s*(\d{1,2}):(\d{2})\s*(?:[-–—]\s*)?([0-9]+(?:[.,][0-9]+)?)\s*(mg|u|unit|units|tab|tabs|tablet|tablets)?\s*$/i;

export function parseBulkDoseText(source: string): BulkDoseParseResult {
  const entries: BulkDoseParsedEntry[] = [];
  const errors: BulkDoseParseError[] = [];
  const lines = source.split(/\r?\n/);
  let activeDate: string | null = null;

  for (let index = 0; index < lines.length; index += 1) {
    const lineNumber = index + 1;
    const rawLine = lines[index] ?? "";
    const line = stripComment(rawLine).trim();

    if (!line) continue;

    const inline = line.match(INLINE_PATTERN);
    if (inline) {
      const date = normalizeDate(inline[1], inline[2], inline[3]);
      const time = normalizeTime(inline[4], inline[5]);
      const amount = parseAmount(inline[6]);

      if (!date) {
        errors.push({ line: lineNumber, message: "Invalid date." });
        continue;
      }
      if (!time) {
        errors.push({ line: lineNumber, message: "Invalid time." });
        continue;
      }
      if (amount === null) {
        errors.push({ line: lineNumber, message: "Dose amount must be greater than zero." });
        continue;
      }

      entries.push({
        key: `${lineNumber}-${date}-${time}`,
        line: lineNumber,
        date,
        time,
        amount,
        amountModeOverride: parseAmountMode(inline[7]),
      });
      continue;
    }

    const dateOnly = line.match(DATE_ONLY_PATTERN);
    if (dateOnly) {
      const date = normalizeDate(dateOnly[1], dateOnly[2], dateOnly[3]);
      if (!date) {
        errors.push({ line: lineNumber, message: "Invalid date." });
        activeDate = null;
      } else {
        activeDate = date;
      }
      continue;
    }

    const entry = line.match(ENTRY_PATTERN);
    if (entry) {
      if (!activeDate) {
        errors.push({ line: lineNumber, message: "Add a date line before dose entries." });
        continue;
      }

      const time = normalizeTime(entry[1], entry[2]);
      const amount = parseAmount(entry[3]);

      if (!time) {
        errors.push({ line: lineNumber, message: "Invalid time." });
        continue;
      }
      if (amount === null) {
        errors.push({ line: lineNumber, message: "Dose amount must be greater than zero." });
        continue;
      }

      entries.push({
        key: `${lineNumber}-${activeDate}-${time}`,
        line: lineNumber,
        date: activeDate,
        time,
        amount,
        amountModeOverride: parseAmountMode(entry[4]),
      });
      continue;
    }

    errors.push({
      line: lineNumber,
      message: "Expected a date (YYYY.MM.DD) or dose row (HH:MM amount).",
    });
  }

  return { entries, errors };
}

export function bulkDoseDateTimeToIso(date: string, time: string): string {
  const [year, month, day] = date.split("-").map(Number);
  const [hour, minute] = time.split(":").map(Number);

  return new Date(year, month - 1, day, hour, minute, 0, 0).toISOString();
}

function stripComment(value: string): string {
  const hashIndex = value.indexOf("#");
  return hashIndex >= 0 ? value.slice(0, hashIndex) : value;
}

function normalizeDate(yearValue: string, monthValue: string, dayValue: string): string | null {
  const year = Number(yearValue);
  const month = Number(monthValue);
  const day = Number(dayValue);
  const candidate = new Date(year, month - 1, day);

  if (
    candidate.getFullYear() !== year ||
    candidate.getMonth() + 1 !== month ||
    candidate.getDate() !== day
  ) {
    return null;
  }

  return `${String(year).padStart(4, "0")}-${String(month).padStart(2, "0")}-${String(day).padStart(2, "0")}`;
}

function normalizeTime(hourValue: string, minuteValue: string): string | null {
  const hour = Number(hourValue);
  const minute = Number(minuteValue);
  if (!Number.isInteger(hour) || !Number.isInteger(minute) || hour < 0 || hour > 23 || minute < 0 || minute > 59) {
    return null;
  }

  return `${String(hour).padStart(2, "0")}:${String(minute).padStart(2, "0")}`;
}

function parseAmount(value: string): number | null {
  const parsed = Number(value.replace(",", "."));
  if (!Number.isFinite(parsed) || parsed <= 0) return null;
  return parsed;
}

function parseAmountMode(value: string | undefined): BulkDoseAmountMode | null {
  if (!value) return null;
  return value.toLowerCase() === "mg" ? "mg" : "units";
}
