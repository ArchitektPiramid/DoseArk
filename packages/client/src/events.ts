export type DoseArkRequestEvent = {
  requestId: string;
  phase: "started" | "succeeded" | "failed";
  method: string;
  path: string;
  durationMs?: number;
  status?: number;
  error?: string;
};

export type DoseArkClientOptions = {
  baseUrl: string;
  onRequestEvent?: (event: DoseArkRequestEvent) => void;
};
