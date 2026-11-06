import type {
  CreateDoseLogInput,
  DoseLog,
  DoseLogListInput,
  DoseLogNutrient,
  DoseLogPage,
  UpdateDoseLogInput,
} from "@doseark/types";
import type { HttpTransport } from "../transport";

export function listDoseLogs(
  transport: HttpTransport,
  input?: Partial<DoseLogListInput>,
): Promise<DoseLogPage> {
  const params = new URLSearchParams();

  if (input?.limit !== undefined) {
    params.set("limit", String(input.limit));
  }
  if (input?.offset !== undefined) {
    params.set("offset", String(input.offset));
  }
  if (input?.product_id) {
    params.set("product_id", input.product_id);
  }

  const query = params.toString();
  return transport.get(`/api/dose-logs${query ? `?${query}` : ""}`);
}

export function createDoseLog(
  transport: HttpTransport,
  input: CreateDoseLogInput,
): Promise<DoseLog> {
  return transport.post("/api/dose-logs", input);
}

export function updateDoseLog(
  transport: HttpTransport,
  id: string,
  input: UpdateDoseLogInput,
): Promise<DoseLog> {
  return transport.patch(`/api/dose-logs/${id}`, input);
}

export function cancelDoseLog(transport: HttpTransport, id: string): Promise<DoseLog> {
  return transport.delete(`/api/dose-logs/${id}`);
}

export function listDoseLogNutrients(
  transport: HttpTransport,
  doseLogId: string,
): Promise<DoseLogNutrient[]> {
  return transport.get(`/api/dose-logs/${doseLogId}/nutrients`);
}
