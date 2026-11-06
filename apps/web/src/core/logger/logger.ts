import type { AppEvent, AppEventLevel, AppEventSource } from "../events";
import { createAppEvent, storeAppEvent } from "../events";
import { APP_NAME } from "../version/appVersion";
import { isLevelEnabled } from "./logLevels";
import { getLoggerConfig } from "./loggerConfig";
import { sanitizeLogDetails } from "./logSanitizer";
import { emitTerminalLog } from "./terminalLogSink";


export function createLogger(source: AppEventSource) {
  function emit(level: AppEventLevel, message: string, details?: unknown): void {
    const config = getLoggerConfig();
    const sanitizedDetails = sanitizeLogDetails(details);
    const storeEnabled = isLevelEnabled(level, config.storeLevel);
    const terminalEnabled = isLevelEnabled(level, config.terminalLevel);
    let event: AppEvent | undefined;

    if (storeEnabled || terminalEnabled) {
      event = createAppEvent({
        level,
        source,
        message,
        details: sanitizedDetails,
      });
    }

    if (event && storeEnabled) {
      storeAppEvent(event);
    }

    if (event && terminalEnabled) {
      emitTerminalLog(event);
    }

    if (isLevelEnabled(level, config.consoleLevel)) {
      writeBrowserConsole(level, source, message, sanitizedDetails);
    }
  }

  return {
    trace: (message: string, details?: unknown) => emit("trace", message, details),
    debug: (message: string, details?: unknown) => emit("debug", message, details),
    info: (message: string, details?: unknown) => emit("info", message, details),
    success: (message: string, details?: unknown) => emit("success", message, details),
    warn: (message: string, details?: unknown) => emit("warn", message, details),
    error: (message: string, details?: unknown) => emit("error", message, details),
  };
}

function writeBrowserConsole(
  level: AppEventLevel,
  source: AppEventSource,
  message: string,
  details: unknown,
): void {
  const label = `[${APP_NAME}:${source}]`;
  const args = details === undefined ? [label, message] : [label, message, details];

  if (level === "warn") {
    console.warn(...args);
  } else if (level === "error") {
    console.error(...args);
  } else if (level === "trace" || level === "debug") {
    console.debug(...args);
  } else {
    console.info(...args);
  }
}
