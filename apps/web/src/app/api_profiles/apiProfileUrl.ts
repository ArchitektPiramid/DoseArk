export function normalizeApiBaseUrl(value: string): string {
  const raw = value.trim();
  const base = typeof document === "undefined" ? "http://localhost/" : document.baseURI;
  const parsed = new URL(raw || ".", base);
  if (parsed.protocol !== "http:" && parsed.protocol !== "https:") {
    throw new Error("API URL must use http:// or https://.");
  }
  if (parsed.username || parsed.password) {
    throw new Error("Do not store credentials in the API URL.");
  }
  parsed.hash = "";
  parsed.search = "";
  return parsed.toString().replace(/\/$/, "");
}

export function safeApiUrlForLog(value: string): string {
  try {
    const base = typeof document === "undefined" ? "http://localhost/" : document.baseURI;
    const parsed = new URL(value || ".", base);
    return `${parsed.protocol}//${parsed.host}${parsed.pathname}`.replace(/\/$/, "");
  } catch {
    return "<invalid-api-url>";
  }
}
