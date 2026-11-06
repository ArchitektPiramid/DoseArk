import type { AppEvent, CreateAppEventInput } from "./appEvent.types";

const MAX_EVENTS = 300;
const SESSION_ID = createId();

let sequence = 0;
let events: AppEvent[] = [];
const listeners = new Set<() => void>();

export function createAppEvent(input: CreateAppEventInput): AppEvent {
  sequence += 1;

  return {
    id: createId(),
    sessionId: SESSION_ID,
    sequence,
    timestamp: new Date().toISOString(),
    ...input,
  };
}

export function storeAppEvent(event: AppEvent): void {
  events = [event, ...events].slice(0, MAX_EVENTS);
  notifyListeners();
}

export function clearAppEvents(): void {
  events = [];
  notifyListeners();
}

export function getAppEventsSnapshot(): AppEvent[] {
  return events;
}

export function subscribeAppEvents(listener: () => void): () => void {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

export function getAppEventSessionId(): string {
  return SESSION_ID;
}

function notifyListeners(): void {
  for (const listener of listeners) {
    listener();
  }
}

function createId(): string {
  return globalThis.crypto?.randomUUID?.() ?? `${Date.now()}-${Math.random().toString(36).slice(2)}`;
}
