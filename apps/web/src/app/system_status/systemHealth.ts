import type { SystemHealth } from "@doseark/types";

export type ConnectionStatus = "checking" | "ready" | "degraded" | "offline";

export function isCompatibleHealthResponse(value: SystemHealth): boolean {
  return Boolean(
    value &&
      typeof value.version === "string" &&
      value.database &&
      (value.database.status === "ok" || value.database.status === "error"),
  );
}

export function getHealthConnectionStatus(health: SystemHealth): ConnectionStatus {
  return health.status === "ok" && health.database.status === "ok" ? "ready" : "degraded";
}

export function formatConnectionError(error: unknown, timeoutMs: number): string {
  if (error instanceof DOMException && error.name === "AbortError") {
    return `API health check timed out after ${Math.round(timeoutMs / 1_000)} seconds.`;
  }
  if (globalThis.navigator?.onLine === false) {
    return "The browser is offline. Check the network connection.";
  }
  if (error instanceof TypeError) {
    return "The API server could not be reached. Check whether it is running and whether the URL and CORS configuration are correct.";
  }
  return error instanceof Error ? error.message : "Unknown API connection error.";
}
