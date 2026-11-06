import type { AppEventLevel } from "../events";

export type LogLevel = AppEventLevel | "silent";

const LEVEL_ORDER: Record<LogLevel, number> = {
  trace: 5,
  debug: 10,
  info: 20,
  success: 20,
  warn: 30,
  error: 40,
  silent: 999,
};

const VALID_LEVELS = new Set<LogLevel>([
  "trace",
  "debug",
  "info",
  "success",
  "warn",
  "error",
  "silent",
]);

export function parseLogLevel(value: string | undefined, fallback: LogLevel): LogLevel {
  return value && VALID_LEVELS.has(value as LogLevel) ? (value as LogLevel) : fallback;
}

export function isLevelEnabled(level: AppEventLevel, threshold: LogLevel): boolean {
  return LEVEL_ORDER[level] >= LEVEL_ORDER[threshold];
}
