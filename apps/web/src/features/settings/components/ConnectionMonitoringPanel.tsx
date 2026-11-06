import { RefreshCw } from "lucide-react";

import { useSystemStatus } from "@/app/system_status";
import { Button, Field, Panel, SelectInput } from "@/core/ui";

export function ConnectionMonitoringPanel() {
  const system = useSystemStatus();

  return (
    <Panel title="Connection monitoring">
      <p className="mt-settings-panel-description">
        Controls the real <code>/api/health</code> polling used by the sidebar status indicators.
      </p>

      <div className="mt-settings-form-grid">
        <Field label="Automatic refresh">
          <SelectInput
            value={String(system.settings.refreshIntervalSeconds)}
            onChange={(event) =>
              system.updateSettings({ refreshIntervalSeconds: Number(event.target.value) })
            }
          >
            <option value="0">Disabled</option>
            <option value="15">Every 15 seconds</option>
            <option value="30">Every 30 seconds</option>
            <option value="60">Every minute</option>
            <option value="120">Every 2 minutes</option>
          </SelectInput>
        </Field>

        <Field label="Request timeout">
          <SelectInput
            value={String(system.settings.requestTimeoutMs)}
            onChange={(event) =>
              system.updateSettings({ requestTimeoutMs: Number(event.target.value) })
            }
          >
            <option value="3000">3 seconds</option>
            <option value="5000">5 seconds</option>
            <option value="10000">10 seconds</option>
            <option value="15000">15 seconds</option>
          </SelectInput>
        </Field>
      </div>

      <Button onClick={() => void system.refresh()} disabled={system.isRefreshing}>
        <RefreshCw size={15} className={system.isRefreshing ? "mt-spin" : undefined} />
        {system.isRefreshing ? "Checking connection..." : "Check connection now"}
      </Button>
    </Panel>
  );
}
