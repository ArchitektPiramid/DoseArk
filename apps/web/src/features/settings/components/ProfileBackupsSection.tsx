import type { BackupList } from "@doseark/types";
import { Download, HardDriveDownload, Plus } from "lucide-react";

import { buildActiveApiUrl } from "@/app/client";
import { Button } from "@/core/ui";
import { formatBytes, formatDateTime } from "../utils/systemFormat";

export function ProfileBackupsSection(props: {
  backups: BackupList | null;
  creating?: boolean;
  error?: string | null;
  onCreateBackup?: () => void;
}) {
  const canCreate = Boolean(props.backups?.can_create && props.onCreateBackup);
  const latest = props.backups?.files[0] ?? null;
  const totalSizeBytes = props.backups?.files.reduce((total, backup) => total + backup.size_bytes, 0) ?? 0;

  return (
    <div className="mt-profile-editor__section">
      <div className="mt-profile-section-heading">
        <h3>Backups</h3>
        {canCreate ? (
          <Button type="button" onClick={props.onCreateBackup} disabled={props.creating}>
            <Plus size={14} /> {props.creating ? "Creating..." : "Create backup"}
          </Button>
        ) : null}
      </div>

      {props.error ? (
        <div className="mt-settings-notice mt-settings-notice--danger">{props.error}</div>
      ) : !props.backups ? (
        <div className="mt-settings-empty">Backup information is unavailable.</div>
      ) : props.backups.status === "unsupported" ? (
        <div className="mt-settings-empty">Backups are not supported by the active storage adapter.</div>
      ) : props.backups.status !== "ok" ? (
        <div className="mt-settings-notice mt-settings-notice--danger">{props.backups.error}</div>
      ) : (
        <>
          <div className="mt-settings-summary">
            <HardDriveDownload size={18} />
            <div className="mt-settings-summary__content">
              <strong>{props.backups.files.length} backup files · {formatBytes(totalSizeBytes)} total</strong>
              <small>
                {latest ? `${formatDateTime(latest.modified_at)} · ${formatBytes(latest.size_bytes)}` : "No backups"}
                {` · ${props.backups.directory}`}
              </small>
            </div>
          </div>
          <div className="mt-settings-backup-list mt-profile-backup-list">
            {props.backups.files.map((backup) => (
              <div key={backup.name} className="mt-settings-backup-row">
                <div>
                  <strong>{backup.name}</strong>
                  <small>{formatDateTime(backup.modified_at)} · {formatBytes(backup.size_bytes)} · {backup.format}</small>
                </div>
                <a className="mt-button" href={buildActiveApiUrl(backup.download_path)}>
                  <Download size={14} /> Download
                </a>
              </div>
            ))}
          </div>
        </>
      )}
    </div>
  );
}
