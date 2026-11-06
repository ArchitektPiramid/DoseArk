import { useEffect, useRef } from "react";
import { useLocation } from "react-router";

import { getActiveApiBaseUrl } from "@/app/client";
import { getAppEventSessionId } from "@/core/events";
import { createLogger } from "@/core/logger/logger";
import { getStartupLoggerConfig } from "@/core/logger/loggerConfig";
import { APP_NAME, FRONTEND_VERSION } from "@/core/version/appVersion";

const logger = createLogger("system");
let startupLogged = false;

export function FrontendEventBridge() {
  const location = useLocation();
  const previousPath = useRef<string | null>(null);

  useEffect(() => {
    if (!startupLogged) {
      startupLogged = true;
      logger.info("Web UI started", {
        appName: APP_NAME,
        frontendVersion: FRONTEND_VERSION,
        mode: import.meta.env.MODE,
        sessionId: getAppEventSessionId(),
        online: navigator.onLine,
        language: navigator.language,
        timeZone: Intl.DateTimeFormat().resolvedOptions().timeZone,
        apiBaseUrl: getActiveApiBaseUrl(),
        logging: getStartupLoggerConfig(),
      });
    }

    function handleOnline() {
      logger.info("Browser network connection restored");
    }

    function handleOffline() {
      logger.warn("Browser network connection lost");
    }

    function handleVisibilityChange() {
      logger.debug("Document visibility changed", { visibilityState: document.visibilityState });
    }

    function handleWindowError(event: ErrorEvent) {
      logger.error("Unhandled browser error", {
        error: event.error instanceof Error ? event.error : undefined,
        message: event.message,
        file: event.filename || undefined,
        line: event.lineno || undefined,
        column: event.colno || undefined,
      });
    }

    function handleUnhandledRejection(event: PromiseRejectionEvent) {
      logger.error("Unhandled promise rejection", {
        reason: event.reason instanceof Error ? event.reason : String(event.reason),
      });
    }

    globalThis.addEventListener("online", handleOnline);
    globalThis.addEventListener("offline", handleOffline);
    globalThis.addEventListener("error", handleWindowError);
    globalThis.addEventListener("unhandledrejection", handleUnhandledRejection);
    document.addEventListener("visibilitychange", handleVisibilityChange);

    return () => {
      globalThis.removeEventListener("online", handleOnline);
      globalThis.removeEventListener("offline", handleOffline);
      globalThis.removeEventListener("error", handleWindowError);
      globalThis.removeEventListener("unhandledrejection", handleUnhandledRejection);
      document.removeEventListener("visibilitychange", handleVisibilityChange);
    };
  }, []);

  useEffect(() => {
    logger.debug("Route changed", {
      previousPath: previousPath.current,
      path: location.pathname,
    });
    previousPath.current = location.pathname;
  }, [location.pathname]);

  return null;
}
