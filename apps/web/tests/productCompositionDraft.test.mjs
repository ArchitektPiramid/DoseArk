import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { test } from "node:test";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import vm from "node:vm";

const require = createRequire(import.meta.url);
const ts = require("typescript");
const here = dirname(fileURLToPath(import.meta.url));
const source = readFileSync(
  join(here, "../src/features/products/utils/productCompositionDraft.ts"),
  "utf8",
);
const js = ts.transpileModule(source, {
  compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS },
}).outputText;
const module = { exports: {} };
vm.runInNewContext(
  js,
  {
    module,
    exports: module.exports,
    crypto: { randomUUID: () => "test-row" },
    require: (name) => {
      if (name !== "@/core/format") throw new Error(`Unexpected import: ${name}`);
      return {
        formatEditableNumber: (value) => String(value),
        formatNumber: (value) => String(value),
        parseNumberInput: (value) => {
          if (!value.trim()) return null;
          const result = Number(value.trim().replace(",", "."));
          return Number.isFinite(result) ? result : null;
        },
      };
    },
  },
  { filename: "productCompositionDraft.ts" },
);
const { createEmptyCompositionDraftRow, toSaveProductNutrientInput } = module.exports;

test("composition form requires nutrient, positive amount and unit", () => {
  const row = { ...createEmptyCompositionDraftRow(), nutrient_id: "nutrient-a" };
  assert.equal(toSaveProductNutrientInput({ ...row, amount_per_serving: "0" }), null);
  assert.equal(toSaveProductNutrientInput({ ...row, amount_per_serving: "-1" }), null);
  assert.equal(toSaveProductNutrientInput({ ...row, amount_per_serving: "NaN" }), null);
  assert.equal(toSaveProductNutrientInput({ ...row, nutrient_id: "" }), null);
  assert.equal(toSaveProductNutrientInput({ ...row, unit: "  " }), null);
});

test("composition form keeps the nutrient payload and normalizes optional fields", () => {
  const row = {
    ...createEmptyCompositionDraftRow(),
    nutrient_id: "nutrient-a",
    amount_per_serving: "2,5",
    unit: " mg ",
    label_daily_value_percent: "25",
    label_reference_amount: "10,5",
    label_reference_unit: " mg ",
    notes: " With food ",
    source: " ",
  };
  const input = toSaveProductNutrientInput(row);
  assert.equal(input.nutrient_id, "nutrient-a");
  assert.equal(input.amount_per_serving, 2.5);
  assert.equal(input.unit, "mg");
  assert.equal(input.label_daily_value_percent, 25);
  assert.equal(input.label_reference_amount, 10.5);
  assert.equal(input.label_reference_unit, "mg");
  assert.equal(input.notes, "With food");
  assert.equal(input.source, "label");
});
