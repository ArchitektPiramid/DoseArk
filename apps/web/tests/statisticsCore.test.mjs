import assert from "node:assert/strict";
import { test } from "node:test";
import { buildStatisticsDataset } from "../src/features/statistics/utils/statisticsSeries.ts";
import {
  getStatisticsRangeDayCount,
  isStatisticsRangeValid,
  shiftStatisticsRange,
} from "../src/features/statistics/utils/statisticsRange.ts";

test("statistics report aggregates daily totals, counts and monthly buckets", () => {
  const report = {
    selected_unit: "mg",
    days: [
      { date: "2024-01-31", amount_total: 50, dose_count: 2, unit: "mg" },
      { date: "2024-02-01", amount_total: 0, dose_count: 0, unit: "mg" },
      { date: "2024-02-02", amount_total: 25, dose_count: 1, unit: "mg" },
    ],
    events: [{ taken_at: "bad-date" }],
  };
  const dataset = buildStatisticsDataset(report);
  assert.deepEqual(
    dataset.monthly.map(({ month, amount, doseCount }) => ({ month, amount, doseCount })),
    [
      { month: "2024-01", amount: 50, doseCount: 2 },
      { month: "2024-02", amount: 25, doseCount: 1 },
    ],
  );
  assert.deepEqual(dataset.summary, {
    total: 75,
    doseCount: 3,
    recordedDays: 2,
    averagePerRecordedDay: 37.5,
  });
  assert.equal(
    dataset.hourly.reduce((sum, hour) => sum + hour.doseCount, 0),
    0,
  );
});

test("empty statistics and date range boundaries remain consistent", () => {
  assert.deepEqual(buildStatisticsDataset(null).summary, {
    total: 0,
    doseCount: 0,
    recordedDays: 0,
    averagePerRecordedDay: 0,
  });
  assert.equal(getStatisticsRangeDayCount({ startDate: "2024-02-28", endDate: "2024-03-01" }), 3);
  assert.deepEqual(shiftStatisticsRange({ startDate: "2024-02-28", endDate: "2024-03-01" }, 1), {
    startDate: "2024-02-29",
    endDate: "2024-03-02",
  });
  assert.equal(isStatisticsRangeValid({ startDate: "2024-03-02", endDate: "2024-03-01" }), false);
});
