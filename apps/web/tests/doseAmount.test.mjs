import assert from "node:assert/strict";
import { test } from "node:test";
import {
  calculateSplitDose,
  clampInteger,
  createSplitState,
  getInitialDoseMode,
  parseDoseNumberInput,
} from "../src/features/dosing/utils/doseAmount.ts";

test("dose amount parser accepts decimal commas and rejects invalid values", () => {
  assert.equal(parseDoseNumberInput(" 1,25 "), 1.25);
  assert.equal(parseDoseNumberInput("1.5"), 1.5);
  for (const input of ["", " ", "Infinity", "NaN", "abc"]) {
    assert.equal(parseDoseNumberInput(input), null);
  }
});

test("split picker approximates common fractions without losing whole units", () => {
  for (const amount of [0.5, 0.25, 0.75, 1.5, 2.2]) {
    const state = createSplitState(amount);
    assert.ok(
      Math.abs(calculateSplitDose(state.wholeUnits, state.partsTaken, state.splitParts) - amount) <
        1e-6,
    );
  }
  assert.equal(getInitialDoseMode(2), "whole");
  assert.equal(getInitialDoseMode(1.5), "split");
  assert.equal(clampInteger(Number.NaN, 1, 10), 1);
});
