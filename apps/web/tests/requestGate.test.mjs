import assert from "node:assert/strict";
import { test } from "node:test";
import { createRequestGate } from "../src/core/async/requestGate.ts";

test("new requests invalidate previous success and error responses", () => {
  const gate = createRequestGate();
  const first = gate.begin();
  assert.equal(gate.isCurrent(first), true);
  const second = gate.begin();
  assert.equal(gate.isCurrent(first), false);
  assert.equal(gate.isCurrent(second), true);
});

test("profile switches and unmounts invalidate pending requests", () => {
  const gate = createRequestGate();
  const pending = gate.begin();
  gate.invalidate();
  assert.equal(gate.isCurrent(pending), false);
  assert.equal(gate.isCurrent(gate.begin()), true);
});
