import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import vm from "node:vm";

const require = createRequire(import.meta.url);
const ts = require("typescript");
const here = dirname(fileURLToPath(import.meta.url));

// These small helpers are tested without mounting React or a running API.
function loadModule(file, dependencies = {}) {
  const source = readFileSync(join(here, file), "utf8");
  const js = ts.transpileModule(source, {
    compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS },
  }).outputText;
  const module = { exports: {} };
  vm.runInNewContext(js, {
    module,
    exports: module.exports,
    require: (name) => {
      if (!(name in dependencies)) throw new Error(`Unexpected import: ${name}`);
      return dependencies[name];
    },
  }, { filename: file });
  return module.exports;
}

const number = loadModule("../src/core/format/number.ts");
const { productFormValues, validateProductForm, buildProductInput } = loadModule(
  "../src/features/products/utils/productFormModel.ts",
  { "@/core/format": number },
);

test("default product needs a name", () => {
  const values = productFormValues();
  assert.equal(values.servingSize, "1");
  assert.equal(values.servingUnit, "tablet");
  assert.equal(validateProductForm(values).valid, false);
  assert.equal(buildProductInput(values), null);
});

test("valid form produces the existing API payload", () => {
  const values = { ...productFormValues(), name: "Vitamin C", servingSize: "2", barcode: "  12345  " };
  const input = buildProductInput(values);
  assert.equal(input.name, "Vitamin C");
  assert.equal(input.serving_size_units, 2);
  assert.equal(input.barcode, "12345");
  assert.equal(input.unit_weight_mg, null);
  assert.equal(input.archived, false);
});

test("optional weight is validated instead of silently discarded", () => {
  const values = { ...productFormValues(), name: "Vitamin C" };
  for (const weight of ["-1", "Infinity", "abc"]) {
    assert.equal(validateProductForm({ ...values, unitWeightMg: weight }).valid, false);
    assert.equal(buildProductInput({ ...values, unitWeightMg: weight }), null);
  }
  assert.equal(buildProductInput({ ...values, unitWeightMg: "0" }).unit_weight_mg, 0);
  assert.equal(buildProductInput({ ...values, unitWeightMg: "250.5" }).unit_weight_mg, 250.5);
});

test("existing product values are preserved in the editor", () => {
  const product = {
    brand_id: "brand-1", name: "Magnesium", category: "supplement", dosage_form: "capsule",
    serving_unit: "capsule", serving_size_units: 2, unit_weight_mg: 100,
    barcode: "AAA", description: "With food", archived: true,
    ui_color: "cyan", ui_icon: "moon", user_importance: "high",
  };
  const values = productFormValues(product);
  assert.equal(values.servingSize, "2");
  assert.equal(values.unitWeightMg, "100");
  assert.equal(values.archived, true);
  assert.equal(values.uiIcon, "moon");
  const input = buildProductInput(values);
  assert.equal(input.description, "With food");
  assert.equal(input.user_importance, "high");
});
