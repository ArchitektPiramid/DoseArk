import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTsModule } from "./helpers/loadTsModule.mjs";

const number = loadTsModule("src/core/format/number.ts");
const {
  buildInventoryLotInput,
  createEmptyInventoryLotFormState,
  inventoryLotToFormState,
  isInventoryLotFormValid,
} = loadTsModule("src/features/inventory/utils/inventoryLotForm.ts", {
  "@/core/format": number,
});

function validForm() {
  return {
    ...createEmptyInventoryLotFormState(),
    productId: "product-a",
    unitsInitial: "10",
    unitsRemaining: "8",
  };
}

test("new lots may use the selected product", () => {
  const form = validForm();
  assert.equal(isInventoryLotFormValid(form), true);
  assert.equal(buildInventoryLotInput(form).product_id, "product-a");
});

test("editing keeps the existing lot's product identity", () => {
  const form = validForm();
  assert.equal(isInventoryLotFormValid(form, "product-a"), true);
  assert.equal(buildInventoryLotInput(form, "product-a").product_id, "product-a");

  const changedProduct = { ...form, productId: "product-b" };
  assert.equal(isInventoryLotFormValid(changedProduct, "product-a"), false);
  assert.equal(buildInventoryLotInput(changedProduct, "product-a"), null);
});

test("editing an existing lot preserves its product ID in the form", () => {
  const form = inventoryLotToFormState({
    product_id: "existing-product",
    units_initial: 10,
    units_remaining: 5,
    status: "active",
  });
  assert.equal(form.productId, "existing-product");
  assert.equal(isInventoryLotFormValid(form, "existing-product"), true);
  assert.equal(buildInventoryLotInput(form, "existing-product").product_id, "existing-product");
});
