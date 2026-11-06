import { TriangleAlert } from "lucide-react";

import { useApiProfiles } from "@/app/api_profiles";
import { Panel, Switch } from "@/core/ui";

export function RuntimeSourcePanel() {
  const profiles = useApiProfiles();
  const profileMode = profiles.runtimeSource === "profile";
  const selectedProfile = profiles.selectedProfile;
  const profileHasHttpTarget = Boolean(selectedProfile?.baseUrl);

  return (
    <Panel title="Runtime source">
      <div className="mt-runtime-source-choice">
        <span className={!profileMode ? "is-active" : undefined}>.env</span>
        <Switch
          checked={profileMode}
          ariaLabel="Use selected profile instead of environment configuration"
          onChange={(checked) => profiles.setRuntimeSource(checked ? "profile" : "env")}
        />
        <span className={profileMode ? "is-active" : undefined}>Selected profile</span>
      </div>

      <div className="mt-runtime-source-summary">
        <div>
          <small>Selected profile</small>
          <code>{selectedProfile?.name ?? "—"}</code>
        </div>
        <div>
          <small>Effective API</small>
          <code>{profiles.effectiveApiBaseUrl}</code>
        </div>
      </div>

      <div className="mt-settings-notice mt-settings-notice--warning mt-runtime-source-warning">
        <TriangleAlert size={15} aria-hidden="true" />
        <span>
          Temporary solution: this choice and the selected profile are stored only in this browser. Profile mode currently changes the frontend API target and health settings; Rust database/storage configuration still comes from <code>.env</code> / process environment.
          {profileMode && !profileHasHttpTarget ? " The selected profile has no HTTP API URL, so the frontend keeps the .env API target." : ""}
        </span>
      </div>
    </Panel>
  );
}
