import type { RuntimeConfiguration } from "@doseark/types";

import { getActiveApiBaseUrl } from "@/app/client";
import { Panel } from "@/core/ui";

export function ConfigurationSourcesPanel({ runtimeConfiguration }: { runtimeConfiguration: RuntimeConfiguration | null }) {
  const runtime = runtimeConfiguration;

  return (
    <Panel title="Environment configuration" className="mt-settings-panel--wide">
      <div className="mt-profile-source-list">
        <SourceRow label="Rust .env file" value={runtime?.env_file ?? "process environment / no .env file"} />
        <SourceRow label="VITE_API_BASE_URL" value={getActiveApiBaseUrl()} />
      </div>

      <div className="mt-settings-env-vars" aria-label="Rust environment variables">
        <div className="mt-settings-env-vars__header">
          <span>Rust variable</span>
          <span>Current value</span>
        </div>
        {runtime?.environment_variables?.length ? (
          runtime.environment_variables.map((variable) => (
            <div className="mt-settings-env-var" key={variable.name}>
              <code>{variable.name}</code>
              <span>{variable.value || "—"}</span>
            </div>
          ))
        ) : (
          <div className="mt-settings-empty">No recognized DoseArk variables were loaded.</div>
        )}
      </div>

      <div className="mt-profile-progress-note">
        <strong>ENV RUNTIME</strong>
        <span>
          Rust still uses <code>.env</code> / process environment as the effective configuration source. Profile values are preview-only.
        </span>
      </div>
    </Panel>
  );
}

function SourceRow({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <span>{label}</span>
      <code>{value}</code>
    </div>
  );
}
