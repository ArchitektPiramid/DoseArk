import assert from "node:assert/strict";
import { test } from "node:test";
import {
  buildStatisticsNutrientKey,
  resolveStatisticsNutrient,
} from "../src/features/statistics/utils/statisticsSelection.ts";

const nutrients = [
  { nutrient_id: "n-1", unit: "mg" },
  { nutrient_id: "n-1", unit: "µg" },
  { nutrient_id: "n-2", unit: "mg" },
];

test("statistics selection keeps a matching nutrient and unit", () => {
  const key = buildStatisticsNutrientKey({ nutrientId: "n-1", unit: "µg" });
  assert.deepEqual(resolveStatisticsNutrient(nutrients, key), { nutrient: nutrients[1], key });
});

test("statistics selection falls back to first available option", () => {
  assert.deepEqual(resolveStatisticsNutrient(nutrients, "missing::mg"), {
    nutrient: nutrients[0],
    key: "n-1::mg",
  });
  assert.deepEqual(resolveStatisticsNutrient([], "n-1::mg"), { nutrient: null, key: null });
});
