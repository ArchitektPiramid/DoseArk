import { parseLogLevel, type LogLevel } from "./logLevels";

const DEFAULT_SLOW_REQUEST_MS = 1_500;
const MIN_SLOW_REQUEST_MS = 100;
const MAX_SLOW_REQUEST_MS = 60_000;

export type LoggerConfig = Readonly<{
  storeLevel: LogLevel;
  terminalLevel: LogLevel;
  consoleLevel: LogLevel;
  slowRequestMs: number;
}>;

const LOGGER_CONFIG: LoggerConfig = Object.freeze({
  storeLevel: parseSinkLogLevel(import.meta.env.VITE_LOG_STORE_LEVEL, "trace"),
  terminalLevel: parseSinkLogLevel(import.meta.env.VITE_LOG_TERMINAL_LEVEL, "debug"),
  consoleLevel: parseSinkLogLevel(import.meta.env.VITE_LOG_CONSOLE_LEVEL, "warn"),
  slowRequestMs: parseSlowRequestMs(import.meta.env.VITE_LOG_SLOW_REQUEST_MS),
});

export function getLoggerConfig(): LoggerConfig {
  return LOGGER_CONFIG;
}

export function getStartupLoggerConfig(): LoggerConfig {
  return LOGGER_CONFIG;
}

function parseSinkLogLevel(value: string | undefined, fallback: LogLevel): LogLevel {
  const parsed = parseLogLevel(value, fallback);
  return parsed === "success" ? "info" : parsed;
}

function parseSlowRequestMs(value: string | undefined): number {
  if (!value) return DEFAULT_SLOW_REQUEST_MS;

  const parsed = Number(value);
  if (!Number.isFinite(parsed)) return DEFAULT_SLOW_REQUEST_MS;

  return Math.min(MAX_SLOW_REQUEST_MS, Math.max(MIN_SLOW_REQUEST_MS, Math.round(parsed)));
}
