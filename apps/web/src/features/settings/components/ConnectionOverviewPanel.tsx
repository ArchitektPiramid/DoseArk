import type { DatabaseOverview } from "@doseark/types";
import { Database, MonitorSmartphone, Server } from "lucide-react";

import { getActiveApiBaseUrl } from "@/app/client";
import { type ConnectionStatus, useSystemStatus } from "@/app/system_status";
import { FRONTEND_VERSION_LABEL } from "@/core/version/appVersion";
import { Panel } from "@/core/ui";
import { formatBytes, formatStorageEngine } from "../utils/systemFormat";
import { SettingsStatusCard } from "./SettingsStatusCard";

export function ConnectionOverviewPanel({ database }: { database: DatabaseOverview | null }) {
  const system = useSystemStatus();
  const storageStatus = resolveStorageStatus(system.status, system.health?.database.status);

  return (
    <Panel
      title="Connection overview"
      action={
        <span className="mt-settings-overall-status" data-settings-status={system.status}>
          {system.status}
        </span>
      }
      className="mt-settings-panel--wide mt-settings-core-overview"
    >
      <div className="mt-settings-status-grid">
        <SettingsStatusCard
          icon={MonitorSmartphone}
          title="Web UI"
          status="ready"
          description="Browser client"
          detail={FRONTEND_VERSION_LABEL}
        />

        <SettingsStatusCard
          icon={Server}
          title="Remote API"
          status={system.status}
          description={system.health?.service ?? "Shared HTTP backend"}
          detail={getActiveApiBaseUrl()}
        />

        <SettingsStatusCard
          icon={Database}
          title="Storage"
          status={storageStatus}
          description="Active data store"
          detail={
            system.health?.database.database_name
              ? `${formatStorageEngine(system.health.database.engine)} · ${system.health.database.database_name}`
              : "Status unavailable without API connection"
          }
        />
      </div>

      <div className="mt-settings-core-meta" aria-label="Application runtime versions">
        <RuntimeMeta label="Frontend" value={FRONTEND_VERSION_LABEL} />
        <RuntimeMeta label="Backend" value={system.health?.version ? `v${system.health.version}` : "—"} />
        <RuntimeMeta label="Schema" value={database?.schema_version ? `v${database.schema_version}` : "—"} />
        <RuntimeMeta
          label="Migration"
          value={database ? `${database.applied_migration_revision}/${database.expected_migration_revision}` : "—"}
        />
        <RuntimeMeta label="DB size" value={database ? formatBytes(database.database_size_bytes) : "—"} />
      </div>

      {system.error ? (
        <div className="mt-settings-notice mt-settings-notice--danger">
          <strong>Connection check failed.</strong>
          <span>{system.error}</span>
        </div>
      ) : null}

      {system.health && !system.health.database.compatible ? (
        <div className="mt-settings-notice mt-settings-notice--warning">
          Storage migration revision does not match this API build.
        </div>
      ) : null}
    </Panel>
  );
}

function RuntimeMeta({ label, value }: { label: string; value: string }) {
  return (
    <span>
      {label} <strong>{value}</strong>
    </span>
  );
}

function resolveStorageStatus(
  apiStatus: ConnectionStatus,
  storageStatus: "ok" | "error" | undefined,
): ConnectionStatus {
  if (apiStatus === "checking") return "checking";
  if (apiStatus === "offline") return "offline";
  if (storageStatus === "ok") return "ready";
  return "degraded";
}
