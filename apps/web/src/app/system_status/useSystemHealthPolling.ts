import type { SystemHealth } from "@doseark/types";
import { useCallback, useEffect, useRef, useState } from "react";

import { apiClient } from "@/app/client";
import { createRequestGate } from "@/core/async/requestGate";
import { createLogger } from "@/core/logger/logger";
import {
  formatConnectionError,
  getHealthConnectionStatus,
  isCompatibleHealthResponse,
  type ConnectionStatus,
} from "./systemHealth";

const logger = createLogger("system");

type PollOptions = {
  healthPath: string;
  timeoutMs: number;
  refreshIntervalSeconds: number;
  profileKey: string;
};

/** Polls the health endpoint and invalidates in-flight requests on option changes or unmount. */
export function useSystemHealthPolling({
  healthPath,
  timeoutMs,
  refreshIntervalSeconds,
  profileKey,
}: PollOptions) {
  const [status, setStatus] = useState<ConnectionStatus>("checking");
  const [health, setHealth] = useState<SystemHealth | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [checkedAt, setCheckedAt] = useState<Date | null>(null);
  const [roundTripMs, setRoundTripMs] = useState<number | null>(null);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const gate = useRef(createRequestGate());
  const lastLoggedStatus = useRef<ConnectionStatus | null>(null);

  const refresh = useCallback(async () => {
    const requestId = gate.current.begin();
    const requestBaseUrl = apiClient.getBaseUrl();
    const startedAt = performance.now();
    const isCurrent = () =>
      gate.current.isCurrent(requestId) && requestBaseUrl === apiClient.getBaseUrl();
    setIsRefreshing(true);

    try {
      const response = await apiClient.health(timeoutMs, healthPath);
      if (!isCurrent()) return;

      if (!isCompatibleHealthResponse(response)) {
        setHealth(null);
        setStatus("degraded");
        setError(
          "The API is reachable, but its health response is from an older or incompatible version.",
        );
        setCheckedAt(new Date());
        setRoundTripMs(Math.max(0, Math.round(performance.now() - startedAt)));
        logStatusTransition(lastLoggedStatus, "degraded", {
          reason: "incompatible-health-response",
        });
        return;
      }

      const nextStatus = getHealthConnectionStatus(response);
      setHealth(response);
      setStatus(nextStatus);
      setError(response.database.error);
      setCheckedAt(new Date());
      setRoundTripMs(Math.max(0, Math.round(performance.now() - startedAt)));
      logStatusTransition(lastLoggedStatus, nextStatus, {
        apiStatus: response.status,
        databaseStatus: response.database.status,
      });
    } catch (requestError) {
      if (!isCurrent()) return;
      setHealth(null);
      setStatus("offline");
      setError(formatConnectionError(requestError, timeoutMs));
      setCheckedAt(new Date());
      setRoundTripMs(null);
      logStatusTransition(lastLoggedStatus, "offline", { error: requestError });
    } finally {
      if (isCurrent()) setIsRefreshing(false);
    }
  }, [healthPath, timeoutMs, profileKey]);

  useEffect(() => {
    void refresh();
    return () => gate.current.invalidate();
  }, [refresh]);

  useEffect(() => {
    if (refreshIntervalSeconds <= 0) return;
    const interval = globalThis.setInterval(() => void refresh(), refreshIntervalSeconds * 1_000);
    return () => globalThis.clearInterval(interval);
  }, [refresh, refreshIntervalSeconds]);

  useEffect(() => {
    function handleOnline() {
      void refresh();
    }
    function handleVisibilityChange() {
      if (document.visibilityState === "visible") void refresh();
    }
    globalThis.addEventListener("online", handleOnline);
    document.addEventListener("visibilitychange", handleVisibilityChange);
    return () => {
      globalThis.removeEventListener("online", handleOnline);
      document.removeEventListener("visibilitychange", handleVisibilityChange);
    };
  }, [refresh]);

  return { status, health, error, checkedAt, roundTripMs, isRefreshing, refresh };
}

function logStatusTransition(
  lastStatusRef: { current: ConnectionStatus | null },
  nextStatus: ConnectionStatus,
  details: unknown,
): void {
  if (lastStatusRef.current === nextStatus) {
    logger.trace("System health poll completed", { status: nextStatus });
    return;
  }
  const previousStatus = lastStatusRef.current;
  lastStatusRef.current = nextStatus;
  const transitionDetails = { previousStatus, nextStatus, details };
  if (nextStatus === "offline" || nextStatus === "degraded") {
    logger.warn("System connection status changed", transitionDetails);
  } else {
    logger.info("System connection status changed", transitionDetails);
  }
}
