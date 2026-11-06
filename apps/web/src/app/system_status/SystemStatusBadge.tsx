import type { ConnectionStatus } from "./SystemStatusProvider";

const LABELS: Record<ConnectionStatus, string> = {
  checking: "CHECKING",
  ready: "READY",
  degraded: "DEGRADED",
  offline: "OFFLINE",
};

type Props = {
  label: string;
  status: ConnectionStatus;
  title?: string;
};

export function SystemStatusBadge({ label, status, title }: Props) {
  return (
    <div className={`mt-system-status-badge mt-system-status-badge--${status}`} title={title}>
      <span className="mt-system-status-badge__dot" aria-hidden="true" />
      <span>{label}: {LABELS[status]}</span>
    </div>
  );
}
