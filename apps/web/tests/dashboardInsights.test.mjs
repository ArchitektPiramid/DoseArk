import assert from "node:assert/strict";
import { test } from "node:test";
import { getDashboardUsageInsights } from "../src/features/dashboard/utils/dashboardInsights.ts";

const dose = (product_id, hour, status = "active") => ({
  product_id,
  status,
  taken_at: new Date(2026, 10, 5, hour).toISOString(),
});

test("groups active dose events by product and by local hour", () => {
  const result = getDashboardUsageInsights([
    dose("vitamin-c", 9),
    dose("vitamin-c", 9),
    dose("magnesium", 21),
  ]);
  assert.equal(result.activeDoses, 3);
  assert.deepEqual(result.topProducts, [
    { productId: "vitamin-c", count: 2 },
    { productId: "magnesium", count: 1 },
  ]);
  assert.equal(result.hours[9], 2);
  assert.equal(result.hours[21], 1);
  assert.equal(result.peakHour, 9);
});

test("ignores cancelled and invalid records", () => {
  const result = getDashboardUsageInsights([
    dose("one", 14, "cancelled"),
    { ...dose("two", 14), taken_at: "not-a-date" },
  ]);
  assert.equal(result.activeDoses, 0);
  assert.deepEqual(result.topProducts, []);
  assert.equal(result.peakHour, null);
  assert.equal(
    result.hours.reduce((sum, count) => sum + count, 0),
    0,
  );
});

test("limits the ranking to five products without changing its count definition", () => {
  const result = getDashboardUsageInsights([
    dose("d", 12),
    dose("c", 12),
    dose("b", 12),
    dose("a", 12),
    dose("e", 12),
    dose("f", 12),
    dose("a", 12),
  ]);
  assert.deepEqual(
    result.topProducts.map((row) => row.productId),
    ["a", "b", "c", "d", "e"],
  );
  assert.equal(result.activeDoses, 7);
});
