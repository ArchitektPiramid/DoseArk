import { DoseArkHttpClient, type DoseArkRequestEvent } from "@doseark/client";

import { createLogger } from "@/core/logger/logger";
import { getLoggerConfig } from "@/core/logger/loggerConfig";

const apiLogger = createLogger("api");
const FALLBACK_API_BASE_URL = (
  import.meta.env.VITE_API_BASE_URL ?? "http://localhost:8080"
).replace(/\/$/, "");

export const apiClient = new DoseArkHttpClient({
  baseUrl: FALLBACK_API_BASE_URL,
  onRequestEvent: logRequestEvent,
});

export function getActiveApiBaseUrl(): string {
  return apiClient.getBaseUrl();
}

export function setActiveApiBaseUrl(baseUrl: string): void {
  apiClient.setBaseUrl(baseUrl);
}


export function buildActiveApiUrl(path: string): string {
  const normalizedPath = path.startsWith("/") ? path : `/${path}`;
  return `${getActiveApiBaseUrl()}${normalizedPath}`;
}

export function getFallbackApiBaseUrl(): string {
  return FALLBACK_API_BASE_URL;
}

function logRequestEvent(event: DoseArkRequestEvent): void {
  const { slowRequestMs } = getLoggerConfig();
  const details = {
    requestId: event.requestId,
    method: event.method,
    path: event.path,
    status: event.status,
    durationMs: event.durationMs,
  };
  const isHealthPoll = event.path === "/api/health" || event.path.endsWith("/api/health");

  if (isHealthPoll) {
    apiLogger.trace(
      event.phase === "failed" ? "API health poll failed" : "API health poll completed",
      event.phase === "failed" ? { ...details, error: event.error } : details,
    );
    return;
  }

  if (event.phase === "failed") {
    const failureDetails = { ...details, error: event.error };
    if (event.status !== undefined && event.status < 500) {
      apiLogger.warn("API request rejected", failureDetails);
    } else {
      apiLogger.error("API request failed", failureDetails);
    }
    return;
  }

  if (event.phase === "succeeded") {
    if (event.durationMs !== undefined && event.durationMs >= slowRequestMs) {
      apiLogger.warn("Slow API request completed", { ...details, slowRequestMs });
    } else {
      apiLogger.debug("API request completed", details);
    }
    return;
  }

  apiLogger.trace("API request started", details);
}
