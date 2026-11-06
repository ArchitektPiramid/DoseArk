import type { LucideIcon } from "lucide-react";

import type { ConnectionStatus } from "@/app/system_status";

const STATUS_LABELS: Record<ConnectionStatus, string> = {
  checking: "Checking",
  ready: "Ready",
  degraded: "Degraded",
  offline: "Offline",
};

type Props = {
  icon: LucideIcon;
  title: string;
  status: ConnectionStatus;
  description: string;
  detail?: string;
};

export function SettingsStatusCard({ icon: Icon, title, status, description, detail }: Props) {
  return (
    <article className="mt-settings-status-card" data-settings-status={status}>
      <div className="mt-settings-status-card__icon">
        <Icon size={19} />
      </div>

      <div className="mt-settings-status-card__content">
        <div className="mt-settings-status-card__header">
          <strong>{title}</strong>
          <span className="mt-settings-status-card__state">
            <span className="mt-status-dot" aria-hidden="true" />
            {STATUS_LABELS[status]}
          </span>
        </div>

        <p>{description}</p>
        {detail ? <div className="mt-settings-status-card__detail mt-mono">{detail}</div> : null}
      </div>
    </article>
  );
}
