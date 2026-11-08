export type AppEventLevel = "trace" | "debug" | "info" | "success" | "warn" | "error";

export type AppEventSource =
  | "system"
  | "api"
  | "products"
  | "brands"
  | "nutrients"
  | "inventory"
  | "dosing"
  | "dashboard"
  | "settings-backup"
  | "statistics"
  | "settings"
  | "storage";

export type AppEvent = {
  id: string;
  sessionId: string;
  sequence: number;
  timestamp: string;
  level: AppEventLevel;
  source: AppEventSource;
  message: string;
  details?: unknown;
};

export type CreateAppEventInput = {
  level: AppEventLevel;
  source: AppEventSource;
  message: string;
  details?: unknown;
};
