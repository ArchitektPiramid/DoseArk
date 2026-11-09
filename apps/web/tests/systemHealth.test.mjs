import assert from "node:assert/strict";
import { test } from "node:test";
import {
  getHealthConnectionStatus,
  isCompatibleHealthResponse,
  formatConnectionError,
} from "../src/app/system_status/systemHealth.ts";

const healthy = { version: "0.2.0", status: "ok", database: { status: "ok", error: null } };

test("health response validation distinguishes compatible and legacy payloads", () => {
  assert.equal(isCompatibleHealthResponse(healthy), true);
  assert.equal(isCompatibleHealthResponse({ ...healthy, database: { status: "unknown" } }), false);
  assert.equal(isCompatibleHealthResponse({ ...healthy, version: 1 }), false);
});

test("database degradation is not reported as a healthy connection", () => {
  assert.equal(getHealthConnectionStatus(healthy), "ready");
  assert.equal(
    getHealthConnectionStatus({ ...healthy, database: { status: "error" } }),
    "degraded",
  );
});

test("timeout error uses the timeout of the active request", () => {
  assert.equal(
    formatConnectionError(new DOMException("aborted", "AbortError"), 15_000),
    "API health check timed out after 15 seconds.",
  );
});
