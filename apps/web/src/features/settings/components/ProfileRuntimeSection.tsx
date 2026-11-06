import type { RuntimeConfiguration } from "@doseark/types";

export function ProfileRuntimeSection({ runtimeConfiguration }: { runtimeConfiguration: RuntimeConfiguration | null }) {
  if (!runtimeConfiguration) {
    return <div className="mt-settings-empty">Rust runtime configuration is unavailable.</div>;
  }

  const runtime = runtimeConfiguration;
  const databaseVariable = runtime.database_target.variable
    ?? (runtime.storage_backend.value === "postgres" ? "DATABASE_URL" : "DOSEARK_SQLITE_PATH");

  return (
    <div className="mt-profile-runtime-section">
      <div className="mt-profile-runtime-source">
        <span>Loaded environment</span>
        <code>{runtime.env_file ?? "process environment / default dotenv lookup"}</code>
      </div>

      <div className="mt-profile-runtime-vars">
        <RuntimeVariable name="DOSEARK_STORAGE" value={runtime.storage_backend.value} />
        <RuntimeVariable name={databaseVariable} value={runtime.database_target.value} />
        <RuntimeVariable name="DOSEARK_BACKUP_DIR" value={runtime.backup_dir.value} />
        <RuntimeVariable name="DOSEARK_API_BIND" value={runtime.bind_addr.value} />
        <RuntimeVariable name="DOSEARK_PROFILES_FILE" value={runtime.profiles_file} />
        <RuntimeVariable
          name="DOSEARK_CREDENTIALS_FILE"
          value={`${runtime.credentials_file}${runtime.credentials_file_exists ? "" : " (not found)"}`}
        />
      </div>

      <div className="mt-profile-progress-note">
        <strong>IN PROGRESS</strong>
        <span>
          Rust runtime uses <code>.env</code> / process environment. Profiles shown on the left are display-only and do not change the active API or database configuration.
        </span>
      </div>
    </div>
  );
}

function RuntimeVariable({ name, value }: { name: string; value: string }) {
  return (
    <div className="mt-profile-runtime-var">
      <code>{name}</code>
      <span>{value || "—"}</span>
    </div>
  );
}
