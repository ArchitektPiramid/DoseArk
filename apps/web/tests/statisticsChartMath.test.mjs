import assert from "node:assert/strict";
import { test } from "node:test";
import {
  fillCalendarDays,
  rollingAverage,
} from "../src/features/statistics/utils/statisticsCharts/shared.ts";

test("calendar alignment inserts missing and leap days without shifting dates", () => {
  const result = fillCalendarDays(
    [
      { date: "2024-02-28", amount: 2, doseCount: 1 },
      { date: "2024-03-01", amount: 4, doseCount: 2 },
    ],
    "2024-02-28",
    "2024-03-01",
  );
  assert.deepEqual(
    result.map((row) => [row.date, row.amount, row.doseCount]),
    [
      ["2024-02-28", 2, 1],
      ["2024-02-29", 0, 0],
      ["2024-03-01", 4, 2],
    ],
  );
});

test("rolling average handles partial initial windows and zero days", () => {
  assert.deepEqual(rollingAverage([2, 0, 4, 6], 3), [2, 1, 2, 10 / 3]);
});
