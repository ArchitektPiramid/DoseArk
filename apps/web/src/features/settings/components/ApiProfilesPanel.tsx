import type { BackupList, RuntimeConfiguration } from "@doseark/types";
import { type ReactNode, useState } from "react";

import { useApiProfiles } from "@/app/api_profiles";
import { Button, Panel } from "@/core/ui";
import { ApiProfileList } from "./ApiProfileList";
import { ProfileBackupsSection } from "./ProfileBackupsSection";
import { ProfileRuntimeSection } from "./ProfileRuntimeSection";
import { ProfileVariablesSection } from "./ProfileVariablesSection";

type DetailTab = "profile" | "runtime" | "backups";

export function ApiProfilesPanel(props: {
  runtimeConfiguration: RuntimeConfiguration | null;
  backups: BackupList | null;
  backupCreating?: boolean;
  backupError?: string | null;
  onCreateBackup?: () => void;
}) {
  const profiles = useApiProfiles();
  const [activeTab, setActiveTab] = useState<DetailTab>("profile");
  const selectedProfile = profiles.selectedProfile;

  return (
    <Panel
      title="Profiles"
      className="mt-settings-panel--wide"
      action={<span className="mt-settings-count">{profiles.profiles.length} profiles</span>}
    >
      <div className="mt-profile-manager">
        <ApiProfileList
          profiles={profiles.profiles}
          defaultProfileId={profiles.defaultProfileId}
          selectedProfileId={profiles.selectedProfileId}
          activeProfileId={profiles.runtimeSource === "profile" ? profiles.selectedProfileId : null}
          onSelect={profiles.setSelectedProfileId}
        />

        <section className="mt-profile-editor">
          <header className="mt-profile-editor__header">
            <div className="mt-profile-editor__heading">
              <div className="mt-profile-editor__title">{selectedProfile?.name ?? "Profile details"}</div>
              <small>
                {profiles.runtimeSource === "profile"
                  ? "Selected profile is the temporary frontend runtime source. Rust database/storage still comes from environment variables."
                  : "Profile preview only. The current frontend runtime source is .env / build environment."}
              </small>
            </div>

            {selectedProfile ? (
              <div className="mt-profile-editor__overview" aria-label="Selected profile summary">
                <ProfileOverviewFact label="Mode" value={selectedProfile.mode} />
                <ProfileOverviewFact
                  label="Database"
                  value={selectedProfile.database.name
                    ? `${selectedProfile.database.type} · ${selectedProfile.database.name}`
                    : selectedProfile.database.type}
                />
                <ProfileOverviewFact label="API" value={selectedProfile.baseUrl || "local / IPC"} />
              </div>
            ) : null}
          </header>

          <div className="mt-tabs mt-profile-detail-tabs" role="tablist" aria-label="Profile details">
            <DetailTabButton active={activeTab === "profile"} onClick={() => setActiveTab("profile")}>Profile variables</DetailTabButton>
            <DetailTabButton active={activeTab === "runtime"} onClick={() => setActiveTab("runtime")}>Detected Rust runtime</DetailTabButton>
            <DetailTabButton active={activeTab === "backups"} onClick={() => setActiveTab("backups")}>Backups</DetailTabButton>
          </div>

          <div className="mt-profile-detail-pane" role="tabpanel">
            {activeTab === "profile" ? <ProfileVariablesSection profile={selectedProfile} /> : null}
            {activeTab === "runtime" ? <ProfileRuntimeSection runtimeConfiguration={props.runtimeConfiguration} /> : null}
            {activeTab === "backups" ? (
              <ProfileBackupsSection
                backups={props.backups}
                creating={props.backupCreating}
                error={props.backupError}
                onCreateBackup={props.onCreateBackup}
              />
            ) : null}
          </div>
        </section>
      </div>

      {profiles.error ? <div className="mt-settings-notice mt-settings-notice--warning">{profiles.error}</div> : null}
    </Panel>
  );
}

function ProfileOverviewFact({ label, value }: { label: string; value: string }) {
  return (
    <span className="mt-profile-editor__overview-fact">
      <small>{label}</small>
      <code title={value}>{value}</code>
    </span>
  );
}

function DetailTabButton({
  active,
  onClick,
  children,
}: {
  active: boolean;
  onClick: () => void;
  children: ReactNode;
}) {
  return (
    <Button
      type="button"
      role="tab"
      aria-selected={active}
      variant={active ? "primary" : "default"}
      className="mt-profile-detail-tab"
      onClick={onClick}
    >
      {children}
    </Button>
  );
}
