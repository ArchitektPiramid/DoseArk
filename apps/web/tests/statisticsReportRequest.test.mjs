import assert from "node:assert/strict";
import { test } from "node:test";
import { createStatisticsReportRequest } from "../src/features/statistics/utils/statisticsReportRequest.ts";

const range = { startDate: "2024-02-01", endDate: "2024-02-29" };

test("product statistics keep the product ID and serving unit", () => {
  assert.deepEqual(
    createStatisticsReportRequest(
      range,
      { kind: "product", productId: "product-1", unit: "capsule" },
      -60,
    ),
    {
      startDate: "2024-02-01",
      endDate: "2024-02-29",
      timezoneOffsetMinutes: -60,
      productId: "product-1",
      nutrientId: null,
      unit: "capsule",
      dayStartHour: 0,
    },
  );
});

test("nutrient and discovery requests do not leak product filters", () => {
  const selected = createStatisticsReportRequest(
    range,
    { kind: "nutrient", nutrientId: "nutrient-1", unit: "mg" },
    120,
  );
  assert.equal(selected.productId, null);
  assert.equal(selected.nutrientId, "nutrient-1");
  assert.equal(selected.unit, "mg");
  const all = createStatisticsReportRequest(range, { kind: "all" }, 120);
  assert.equal(all.productId, null);
  assert.equal(all.nutrientId, null);
  assert.equal(all.unit, null);
});
