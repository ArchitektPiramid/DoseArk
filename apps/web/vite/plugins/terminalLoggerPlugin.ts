import type { IncomingMessage, ServerResponse } from "node:http";
import type { Plugin, ViteDevServer } from "vite";

const ENDPOINT = "/__doseark/dev-logs";
const MAX_BODY_BYTES = 64 * 1024;

const LEVEL_COLORS: Record<string, string> = {
  trace: "\u001b[90m",
  debug: "\u001b[36m",
  info: "\u001b[34m",
  success: "\u001b[32m",
  warn: "\u001b[33m",
  error: "\u001b[31m",
};
const RESET = "\u001b[0m";

export function terminalLoggerPlugin(): Plugin {
  return {
    name: "doseark-terminal-logger",
    apply: "serve",
    configureServer(server) {
      server.middlewares.use(ENDPOINT, (request, response, next) => {
        if (request.method !== "POST") {
          next();
          return;
        }

        receiveLogBatch(server, request, response);
      });
    },
  };
}

function receiveLogBatch(
  server: ViteDevServer,
  request: IncomingMessage,
  response: ServerResponse,
): void {
  let body = "";
  let bodyBytes = 0;

  request.setEncoding("utf8");
  request.on("data", (chunk: string) => {
    bodyBytes += Buffer.byteLength(chunk);
    if (bodyBytes <= MAX_BODY_BYTES) {
      body += chunk;
    }
  });

  request.on("end", () => {
    if (bodyBytes > MAX_BODY_BYTES) {
      response.statusCode = 413;
      response.end();
      return;
    }

    try {
      const parsed: unknown = JSON.parse(body);
      const events = Array.isArray(parsed) ? parsed : [parsed];

      for (const event of events) {
        printEvent(server, event);
      }

      response.statusCode = 204;
      response.end();
    } catch {
      response.statusCode = 400;
      response.end();
    }
  });
}

function printEvent(server: ViteDevServer, value: unknown): void {
  if (!isLogEvent(value)) return;

  const level = value.level.toLowerCase();
  const color = LEVEL_COLORS[level] ?? "";
  const timestamp = new Date(value.timestamp).toLocaleTimeString("en-GB", {
    hour12: false,
  });
  const details = formatDetails(value.details);
  const line = `${color}[web:${level}]${RESET} ${value.source}: ${value.message}${details}`;

  if (level === "error") {
    server.config.logger.error(line, { timestamp: true });
  } else if (level === "warn") {
    server.config.logger.warn(line, { timestamp: true });
  } else {
    server.config.logger.info(`${timestamp} ${line}`);
  }
}

function formatDetails(details: unknown): string {
  if (details == null) return "";

  try {
    const serialized = JSON.stringify(details);
    return serialized === "{}" ? "" : ` ${serialized}`;
  } catch {
    return " [unserializable-details]";
  }
}

function isLogEvent(value: unknown): value is {
  timestamp: string;
  level: string;
  source: string;
  message: string;
  details?: unknown;
} {
  if (value == null || typeof value !== "object") return false;

  const event = value as Record<string, unknown>;
  return (
    typeof event.timestamp === "string" &&
    typeof event.level === "string" &&
    typeof event.source === "string" &&
    typeof event.message === "string"
  );
}
