import type { ApiProfile } from "@/app/api_profiles";

export function ProfileVariablesSection({ profile }: { profile: ApiProfile | null }) {
  if (!profile) {
    return <div className="mt-settings-empty">Select a profile to inspect its display variables.</div>;
  }

  return (
    <div className="mt-profile-runtime-section">
      <VariableGroup
        title="Profile"
        rows={[
          ["id", profile.id],
          ["name", profile.name],
          ["description", profile.description],
          ["mode", profile.mode],
        ]}
      />
      <VariableGroup
        title="Connection"
        rows={[
          ["baseUrl", profile.baseUrl],
          ["healthPath", profile.healthPath],
          ["requestTimeoutMs", `${profile.requestTimeoutMs}`],
        ]}
      />
      <VariableGroup
        title="Database"
        rows={[
          ["database.type", profile.database.type],
          ["database.name", profile.database.name],
          ["database.path", profile.database.path],
          ["database.host", profile.database.host],
          ["database.port", profile.database.port == null ? null : `${profile.database.port}`],
          ["database.sslMode", profile.database.sslMode],
        ]}
      />
      <VariableGroup
        title="Storage"
        rows={[
          ["backupPath", profile.backupPath],
          ["attachmentsPath", profile.attachmentsPath],
          ["credentialsRef", profile.credentialsRef],
        ]}
      />

      <div className="mt-profile-progress-note">
        <strong>PREVIEW ONLY</strong>
        <span>These values are loaded from <code>config/profiles.json</code> and do not configure the Rust runtime yet.</span>
      </div>
    </div>
  );
}

function VariableGroup({
  title,
  rows,
}: {
  title: string;
  rows: Array<[string, string | null]>;
}) {
  return (
    <section className="mt-profile-variable-group">
      <h3>{title}</h3>
      <div className="mt-profile-runtime-vars">
        {rows.map(([name, value]) => (
          <div className="mt-profile-runtime-var" key={name}>
            <code>{name}</code>
            <span>{value || "—"}</span>
          </div>
        ))}
      </div>
    </section>
  );
}
