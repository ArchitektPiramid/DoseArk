import assert from "node:assert/strict";
import { test } from "node:test";
import {
  getActivityStart,
  getDoseActivity,
  loadActivityDoses,
} from "../src/features/dashboard/utils/dashboardActivity.ts";

const now = new Date(2026, 10, 5, 12);
const makeDose = (date, status = "active") => ({ taken_at: date.toISOString(), status });
const page = (items, total = items.length) => ({ items, total, limit: 100, offset: 0 });

test("local-day activity includes the entire 14-day window and excludes cancelled doses", () => {
  const recent = new Date(2026, 10, 5, 9);
  const older = new Date(2026, 9, 23, 18);
  const rows = getDoseActivity([
    makeDose(recent), makeDose(recent, "cancelled"), makeDose(older),
  ], 14, now);
  assert.equal(rows.length, 14);
  assert.equal(rows[0].date, "2026-10-23");
  assert.equal(rows[0].count, 1);
  assert.equal(rows.at(-1).date, "2026-11-05");
  assert.equal(rows.at(-1).count, 1);
  assert.equal(rows.reduce((total, row) => total + row.count, 0), 2);
});

test("paginates through all records in the active period", async () => {
  const doses = Array.from({ length: 130 }, () => makeDose(new Date(2026, 10, 4, 9)));
  let calls = 0;
  const result = await loadActivityDoses(page(doses.slice(0, 100), 130), getActivityStart(now), async (offset) => {
    calls += 1;
    assert.equal(offset, 100);
    return page(doses.slice(offset), 130);
  });
  assert.equal(calls, 1);
  assert.equal(result.doses.length, 130);
  assert.equal(result.truncated, false);
});

test("stops pagination when older than the activity window", async () => {
  const result = await loadActivityDoses(
    page([makeDose(new Date(2026, 7, 16))], 1000),
    getActivityStart(now),
    async () => { throw new Error("should not fetch older data"); },
  );
  assert.equal(result.doses.length, 0);
  assert.equal(result.truncated, false);
});

test("reports partial activity when pagination reaches the safety cap", async () => {
  const doses = Array.from({ length: 120 }, () => makeDose(new Date(2026, 10, 4, 9)));
  const result = await loadActivityDoses(
    page(doses.slice(0, 100), 200),
    getActivityStart(now),
    async () => page(doses.slice(100), 200),
    120,
  );
  assert.equal(result.doses.length, 120);
  assert.equal(result.truncated, true);
});
