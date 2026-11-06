import type { SystemHealth } from "@doseark/types";
import type { HttpTransport } from "../transport";

export async function getSystemHealth(
  transport: HttpTransport,
  timeoutMs = 5_000,
  healthPath = "/api/health",
): Promise<SystemHealth> {
  const controller = new AbortController();
  const timeout = globalThis.setTimeout(() => controller.abort(), timeoutMs);

  try {
    return await transport.get(healthPath, { signal: controller.signal });
  } finally {
    globalThis.clearTimeout(timeout);
  }
}
