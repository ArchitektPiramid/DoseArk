import type {
  BackupFile,
  BackupList,
  DatabaseOverview,
  DatabaseTableStats,
  ProfileValidation,
  RuntimeConfiguration,
} from "@doseark/types";
import type { HttpTransport } from "../transport";

export function listBackups(transport: HttpTransport): Promise<BackupList> {
  return transport.get("/api/admin/backups");
}

export function createBackup(transport: HttpTransport): Promise<BackupFile> {
  return transport.post("/api/admin/backups", {});
}

export function getDatabaseOverview(transport: HttpTransport): Promise<DatabaseOverview> {
  return transport.get("/api/admin/database/overview");
}

export function listDatabaseTables(transport: HttpTransport): Promise<DatabaseTableStats[]> {
  return transport.get("/api/admin/database/tables");
}


export function getRuntimeConfiguration(transport: HttpTransport): Promise<RuntimeConfiguration> {
  return transport.get("/api/admin/runtime/config");
}

export function validateProfile(transport: HttpTransport, profileId: string): Promise<ProfileValidation> {
  return transport.get(`/api/admin/profiles/${encodeURIComponent(profileId)}/validate`);
}
