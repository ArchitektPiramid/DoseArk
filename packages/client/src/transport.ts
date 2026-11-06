import { createClientError, DoseArkClientError, formatSafeRequestError } from "./errors";
import type { DoseArkClientOptions, DoseArkRequestEvent } from "./events";

let requestSequence = 0;

export class HttpTransport {
  private baseUrl: string;
  private readonly onRequestEvent?: (event: DoseArkRequestEvent) => void;

  constructor(options: DoseArkClientOptions) {
    this.baseUrl = normalizeBaseUrl(options.baseUrl);
    this.onRequestEvent = options.onRequestEvent;
  }

  setBaseUrl(baseUrl: string): void {
    this.baseUrl = normalizeBaseUrl(baseUrl);
  }

  getBaseUrl(): string {
    return this.baseUrl;
  }

  get<T>(path: string, init?: RequestInit): Promise<T> {
    return this.request<T>("GET", path, init);
  }

  post<T>(path: string, body: unknown): Promise<T> {
    return this.request<T>("POST", path, {
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
  }

  patch<T>(path: string, body: unknown): Promise<T> {
    return this.request<T>("PATCH", path, {
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
  }

  put<T>(path: string, body: unknown): Promise<T> {
    return this.request<T>("PUT", path, {
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
  }

  delete<T>(path: string): Promise<T> {
    return this.request<T>("DELETE", path);
  }

  private async request<T>(method: string, path: string, init: RequestInit = {}): Promise<T> {
    const requestId = createRequestId();
    const startedAt = performance.now();
    this.onRequestEvent?.({ requestId, phase: "started", method, path });

    try {
      const response = await fetch(`${this.baseUrl}${path}`, { ...init, method });
      if (!response.ok) {
        throw await createClientError(response);
      }

      this.onRequestEvent?.({
        requestId,
        phase: "succeeded",
        method,
        path,
        status: response.status,
        durationMs: Math.round(performance.now() - startedAt),
      });

      return response.json() as Promise<T>;
    } catch (error) {
      this.onRequestEvent?.({
        requestId,
        phase: "failed",
        method,
        path,
        status: error instanceof DoseArkClientError ? error.status : undefined,
        durationMs: Math.round(performance.now() - startedAt),
        error: formatSafeRequestError(error),
      });
      throw error;
    }
  }
}

function normalizeBaseUrl(value: string): string {
  return value.trim().replace(/\/$/, "");
}

function createRequestId(): string {
  requestSequence += 1;
  return globalThis.crypto?.randomUUID?.() ?? `req-${Date.now()}-${requestSequence}`;
}
