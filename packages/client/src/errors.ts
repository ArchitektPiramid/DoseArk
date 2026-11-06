export class DoseArkClientError extends Error {
  readonly status: number;
  readonly statusText: string;

  constructor(status: number, statusText: string, message: string) {
    super(message);
    this.name = "DoseArkClientError";
    this.status = status;
    this.statusText = statusText;
  }
}

export async function createClientError(response: Response): Promise<Error> {
  try {
    const body = await response.json();
    return new DoseArkClientError(
      response.status,
      response.statusText,
      `API request failed: ${response.status} ${response.statusText}: ${JSON.stringify(body)}`,
    );
  } catch {
    return new DoseArkClientError(
      response.status,
      response.statusText,
      `API request failed: ${response.status} ${response.statusText}`,
    );
  }
}

export function formatSafeRequestError(error: unknown): string {
  if (error instanceof DoseArkClientError) {
    return `HTTP ${error.status} ${error.statusText}`;
  }
  if (error instanceof DOMException && error.name === "AbortError") {
    return "Request aborted or timed out";
  }
  return error instanceof Error ? error.name : "Unknown request error";
}
