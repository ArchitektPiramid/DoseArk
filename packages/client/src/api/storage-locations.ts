import type {
  CreateStorageLocationInput,
  StorageLocation,
  UpdateStorageLocationInput,
} from "@doseark/types";
import type { HttpTransport } from "../transport";

export function listStorageLocations(
  transport: HttpTransport,
  includeArchived = false,
): Promise<StorageLocation[]> {
  const params = new URLSearchParams({
    include_archived: String(includeArchived),
  });

  return transport.get(`/api/storage-locations?${params.toString()}`);
}

export function createStorageLocation(
  transport: HttpTransport,
  input: CreateStorageLocationInput,
): Promise<StorageLocation> {
  return transport.post("/api/storage-locations", input);
}

export function updateStorageLocation(
  transport: HttpTransport,
  id: string,
  input: UpdateStorageLocationInput,
): Promise<StorageLocation> {
  return transport.patch(`/api/storage-locations/${id}`, input);
}

export function archiveStorageLocation(
  transport: HttpTransport,
  id: string,
): Promise<StorageLocation> {
  return transport.delete(`/api/storage-locations/${id}`);
}
