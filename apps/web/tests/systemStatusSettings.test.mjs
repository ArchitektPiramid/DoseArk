import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTsModule } from "./helpers/loadTsModule.mjs";

const logger = { info() {}, warn() {} };
const { normalizeSettings, getHealthRequestOptions } = loadTsModule(
  "src/app/system_status/systemStatusSettings.ts",
  {
    "@/core/logger/logger": { createLogger: () => logger },
  },
);

test("monitor settings clamp unsupported values to defaults", () => {
  assert.equal(
    normalizeSettings({ refreshIntervalSeconds: -1, requestTimeoutMs: 200 }).refreshIntervalSeconds,
    30,
  );
  assert.equal(
    normalizeSettings({ refreshIntervalSeconds: 0, requestTimeoutMs: 10_000 }).requestTimeoutMs,
    10_000,
  );
});

test("health options use profile overrides only in profile mode", () => {
  const settings = { refreshIntervalSeconds: 30, requestTimeoutMs: 3_000 };
  const profile = { healthPath: "/healthz", requestTimeoutMs: 15_000 };
  const selected = getHealthRequestOptions(settings, "profile", profile);
  assert.equal(selected.healthPath, "/healthz");
  assert.equal(selected.timeoutMs, 15_000);
  const fallback = getHealthRequestOptions(settings, "fallback", profile);
  assert.equal(fallback.healthPath, "/api/health");
  assert.equal(fallback.timeoutMs, 3_000);
});
