import { RefreshCw } from "lucide-react";
import { useState } from "react";

import { useSystemStatus } from "@/app/system_status";
import { Button, PageHeader } from "@/core/ui";

import { ApiProfilesPanel } from "../components/ApiProfilesPanel";
import { ConnectionMonitoringPanel } from "../components/ConnectionMonitoringPanel";
import { ConfigurationSourcesPanel } from "../components/ConfigurationSourcesPanel";
import { ConnectionOverviewPanel } from "../components/ConnectionOverviewPanel";
import { UiPreferencesPanel } from "../components/UiPreferencesPanel";
import { RuntimeSourcePanel } from "../components/RuntimeSourcePanel";
import { useManagedBackup } from "../hooks/useManagedBackup";
import { useSettingsDiagnostics } from "../hooks/useSettingsDiagnostics";

type SettingsTabId = "general" | "profiles";

const SETTINGS_TABS: { id: SettingsTabId; label: string }[] = [
  { id: "general", label: "General" },
  { id: "profiles", label: "Profiles" },
];

export function SettingsPage() {
  const system = useSystemStatus();
  const diagnostics = useSettingsDiagnostics();
  const managedBackup = useManagedBackup(diagnostics.refresh);
  const [activeTab, setActiveTab] = useState<SettingsTabId>("general");

  async function refreshAll() {
    await Promise.all([system.refresh(), diagnostics.refresh()]);
  }

  return (
    <>
      <PageHeader
        title="Settings"
        description="App preferences and runtime configuration."
        action={
          <Button onClick={() => void refreshAll()} disabled={system.isRefreshing || diagnostics.isLoading}>
            <RefreshCw size={15} className={system.isRefreshing || diagnostics.isLoading ? "mt-spin" : undefined} />
            Refresh
          </Button>
        }
      />

      <ConnectionOverviewPanel database={diagnostics.database} />

      <div className="mt-tabs mt-settings-tabs" role="tablist" aria-label="Settings sections">
        {SETTINGS_TABS.map((tab) => (
          <Button
            key={tab.id}
            type="button"
            role="tab"
            aria-selected={activeTab === tab.id}
            variant={activeTab === tab.id ? "primary" : "default"}
            className="mt-settings-tab-button"
            onClick={() => setActiveTab(tab.id)}
          >
            {tab.label}
          </Button>
        ))}
      </div>

      {diagnostics.error ? (
        <div className="mt-settings-notice mt-settings-notice--warning">
          Some runtime storage information could not be loaded: {diagnostics.error}
        </div>
      ) : null}

      {activeTab === "general" ? (
        <div className="mt-settings-grid mt-settings-grid--general" role="tabpanel">
          <UiPreferencesPanel />
          <RuntimeSourcePanel />
          <ConnectionMonitoringPanel />
          <ConfigurationSourcesPanel runtimeConfiguration={diagnostics.runtimeConfiguration} />
        </div>
      ) : null}

      {activeTab === "profiles" ? (
        <div className="mt-settings-grid" role="tabpanel">
          <ApiProfilesPanel
            runtimeConfiguration={diagnostics.runtimeConfiguration}
            backups={diagnostics.backups}
            backupCreating={managedBackup.creating}
            backupError={managedBackup.error}
            onCreateBackup={diagnostics.backups?.can_create ? () => void managedBackup.createBackup() : undefined}
          />
        </div>
      ) : null}
    </>
  );
}
