import { AlertTriangle, RefreshCw, Settings } from "lucide-react";
import { useNavigate } from "react-router";

import { getActiveApiBaseUrl } from "@/app/client";
import { Button, Panel } from "@/core/ui";

import { useSystemStatus } from "./SystemStatusProvider";

type Props = {
  error: string;
  onRetry?: () => void;
};

export function ApiErrorNotice({ error, onRetry }: Props) {
  const navigate = useNavigate();
  const system = useSystemStatus();
  const isOffline = system.status === "offline";

  return (
    <Panel
      title={isOffline ? "API unavailable" : "API request failed"}
      className="mb-3 mt-api-error-notice"
      action={
        <div className="flex gap-2">
          {onRetry ? (
            <Button onClick={onRetry} disabled={system.isRefreshing}>
              <RefreshCw size={14} />
              Retry
            </Button>
          ) : null}
          <Button onClick={() => navigate("/settings")}>
            <Settings size={14} />
            Settings
          </Button>
        </div>
      }
    >
      <div className="mt-api-error-notice__content">
        <AlertTriangle size={19} />
        <div>
          <strong>
            {isOffline
              ? "The web app cannot reach the remote API."
              : "The API responded, but this request could not be completed."}
          </strong>
          <p>
            Target: <code>{getActiveApiBaseUrl()}</code>
          </p>
          <details>
            <summary>Technical details</summary>
            <pre className="mt-mono">{error}</pre>
          </details>
        </div>
      </div>
    </Panel>
  );
}
