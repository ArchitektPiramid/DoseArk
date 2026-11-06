import type { AppEvent } from "../events";

const DEV_LOG_ENDPOINT = "/__doseark/dev-logs";
const FLUSH_DELAY_MS = 80;
const MAX_BATCH_SIZE = 50;

let queue: AppEvent[] = [];
let flushTimer: ReturnType<typeof setTimeout> | undefined;
let sinkFailureReported = false;

export function emitTerminalLog(event: AppEvent): void {
  if (!import.meta.env.DEV || typeof window === "undefined") {
    return;
  }

  queue.push(event);

  if (queue.length >= MAX_BATCH_SIZE) {
    void flushTerminalLogs();
    return;
  }

  if (flushTimer == null) {
    flushTimer = setTimeout(() => {
      flushTimer = undefined;
      void flushTerminalLogs();
    }, FLUSH_DELAY_MS);
  }
}

async function flushTerminalLogs(): Promise<void> {
  if (queue.length === 0) return;

  const batch = queue.splice(0, MAX_BATCH_SIZE);

  try {
    const response = await fetch(DEV_LOG_ENDPOINT, {
      method: "POST",
      headers: {
        "content-type": "application/json",
      },
      body: JSON.stringify(batch),
      keepalive: true,
    });

    if (!response.ok && !sinkFailureReported) {
      sinkFailureReported = true;
      console.debug(`[DoseArk:logger] Terminal sink returned HTTP ${response.status}.`);
    }
  } catch (error) {
    if (!sinkFailureReported) {
      sinkFailureReported = true;
      console.debug("[DoseArk:logger] Terminal sink is unavailable.", error);
    }
  }

  if (queue.length > 0) {
    void flushTerminalLogs();
  }
}
