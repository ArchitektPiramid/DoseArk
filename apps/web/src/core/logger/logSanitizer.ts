const REDACTED_VALUE = "[redacted]";
const MAX_DEPTH = 4;
const MAX_ARRAY_ITEMS = 20;
const MAX_STRING_LENGTH = 500;
const MAX_STACK_LENGTH = 2_000;

const SENSITIVE_KEY_PATTERNS = [
  /password/i,
  /passphrase/i,
  /token/i,
  /secret/i,
  /authorization/i,
  /cookie/i,
  /api.?key/i,
  /access.?key/i,
  /private.?key/i,
  /database.?url/i,
  /^notes?$/i,
  /^description$/i,
  /^name$/i,
  /product.?name/i,
  /display.?name/i,
];

const URL_KEY_PATTERNS = [/url/i, /uri/i, /endpoint/i];

export function sanitizeLogDetails(value: unknown): unknown {
  return sanitizeValue(value, 0, new WeakSet<object>());
}

function sanitizeValue(value: unknown, depth: number, seen: WeakSet<object>): unknown {
  if (value == null || typeof value === "number" || typeof value === "boolean") {
    return value;
  }

  if (typeof value === "string") {
    return truncate(value, MAX_STRING_LENGTH);
  }

  if (value instanceof Error) {
    return sanitizeError(value, depth, seen);
  }

  if (depth >= MAX_DEPTH) {
    return "[max-depth]";
  }

  if (Array.isArray(value)) {
    return value
      .slice(0, MAX_ARRAY_ITEMS)
      .map((entry) => sanitizeValue(entry, depth + 1, seen));
  }

  if (typeof value !== "object") {
    return String(value);
  }

  if (seen.has(value)) {
    return "[circular]";
  }

  seen.add(value);

  const sanitized: Record<string, unknown> = {};
  for (const [key, entry] of Object.entries(value)) {
    if (isSensitiveKey(key)) {
      sanitized[key] = REDACTED_VALUE;
      continue;
    }

    if (typeof entry === "string" && isUrlKey(key)) {
      sanitized[key] = sanitizeUrl(entry);
      continue;
    }

    sanitized[key] = sanitizeValue(entry, depth + 1, seen);
  }

  return sanitized;
}

function sanitizeError(error: Error, depth: number, seen: WeakSet<object>): Record<string, unknown> {
  const apiStatus = readNumericProperty(error, "status");
  const apiStatusText = readStringProperty(error, "statusText");
  const safeMessage =
    error.name === "DoseArkClientError" && apiStatus !== undefined
      ? `API request failed: ${apiStatus}${apiStatusText ? ` ${apiStatusText}` : ""}`
      : truncate(error.message, MAX_STRING_LENGTH);

  const result: Record<string, unknown> = {
    name: error.name,
    message: safeMessage,
  };

  if (apiStatus !== undefined) {
    result.status = apiStatus;
  }

  if (error.stack) {
    result.stack = truncate(error.stack, MAX_STACK_LENGTH);
  }

  if ("cause" in error && error.cause !== undefined) {
    result.cause = sanitizeValue(error.cause, depth + 1, seen);
  }

  return result;
}

function isSensitiveKey(key: string): boolean {
  return SENSITIVE_KEY_PATTERNS.some((pattern) => pattern.test(key));
}

function isUrlKey(key: string): boolean {
  return URL_KEY_PATTERNS.some((pattern) => pattern.test(key));
}

function sanitizeUrl(value: string): string {
  try {
    const url = new URL(value, globalThis.location?.origin ?? "http://localhost");
    url.username = "";
    url.password = "";
    url.search = "";
    url.hash = "";

    if (/^https?:\/\//i.test(value)) {
      return `${url.origin}${url.pathname}`;
    }

    return url.pathname;
  } catch {
    return truncate(value.split(/[?#]/, 1)[0] ?? value, MAX_STRING_LENGTH);
  }
}

function truncate(value: string, maxLength: number): string {
  return value.length > maxLength ? `${value.slice(0, maxLength)}…` : value;
}

function readNumericProperty(value: object, key: string): number | undefined {
  const entry = (value as Record<string, unknown>)[key];
  return typeof entry === "number" ? entry : undefined;
}

function readStringProperty(value: object, key: string): string | undefined {
  const entry = (value as Record<string, unknown>)[key];
  return typeof entry === "string" ? entry : undefined;
}
