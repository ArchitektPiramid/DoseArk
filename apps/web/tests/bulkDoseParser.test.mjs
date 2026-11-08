import assert from "node:assert/strict";
import { test } from "node:test";
import {
  bulkDoseDateTimeToIso,
  parseBulkDoseText,
} from "../src/features/dosing/utils/bulkDoseParser.ts";

test("bulk parser accepts date groups, inline rows, comments and decimal commas", () => {
  const parsed = parseBulkDoseText(
    `2024.02.29 # leap day\n08:05 250,5 mg\n23:59 - 1 tab\n2024-03-01 09:30 2 units`,
  );
  assert.deepEqual(parsed.errors, []);
  assert.deepEqual(
    parsed.entries.map(({ line, date, time, amount, amountModeOverride }) => ({
      line,
      date,
      time,
      amount,
      amountModeOverride,
    })),
    [
      { line: 2, date: "2024-02-29", time: "08:05", amount: 250.5, amountModeOverride: "mg" },
      { line: 3, date: "2024-02-29", time: "23:59", amount: 1, amountModeOverride: "units" },
      { line: 4, date: "2024-03-01", time: "09:30", amount: 2, amountModeOverride: "units" },
    ],
  );
  assert.equal(new Set(parsed.entries.map((item) => item.key)).size, 3);
});

test("bulk parser reports invalid date, invalid time, non-positive amount and missing date", () => {
  const parsed = parseBulkDoseText(
    `10:30 1\n2024-02-30\n11:00 1\n2024.03.01\n24:00 1\n12:30 0\n12:31 5`,
  );
  assert.deepEqual(
    parsed.errors.map((item) => item.line),
    [1, 2, 3, 5, 6],
  );
  assert.equal(parsed.entries.length, 1);
  assert.equal(parsed.entries[0].time, "12:31");
});

test("bulk date/time conversion keeps the local calendar fields", () => {
  const date = new Date(bulkDoseDateTimeToIso("2024-02-29", "07:05"));
  assert.equal(date.getFullYear(), 2024);
  assert.equal(date.getMonth(), 1);
  assert.equal(date.getDate(), 29);
  assert.equal(date.getHours(), 7);
  assert.equal(date.getMinutes(), 5);
});
