import { Server } from "lucide-react";

import type { ApiProfile } from "@/app/api_profiles";

export function ApiProfileList({
  profiles,
  defaultProfileId,
  selectedProfileId,
  activeProfileId,
  onSelect,
}: {
  profiles: ApiProfile[];
  defaultProfileId: string | null;
  selectedProfileId: string | null;
  activeProfileId: string | null;
  onSelect: (profileId: string) => void;
}) {
  return (
    <aside className="mt-profile-list" aria-label="Display profiles">
      <div className="mt-profile-list__header">
        <span>Profiles</span>
        <small>FILE + TEMP SELECTION</small>
      </div>

      <div className="mt-profile-list__items">
        {profiles.map((profile) => (
          <button
            key={profile.id}
            type="button"
            className="mt-profile-card"
            data-selected={profile.id === selectedProfileId ? "true" : undefined}
            aria-pressed={profile.id === selectedProfileId}
            onClick={() => onSelect(profile.id)}
          >
            <span className="mt-profile-card__icon">
              <Server size={16} />
            </span>
            <span className="mt-profile-card__content">
              <span className="mt-profile-card__name">
                <span>{profile.name}</span>
                <span className="mt-profile-card__badges">
                  {profile.id === activeProfileId ? <small data-active="true">ACTIVE</small> : null}
                  {profile.id === defaultProfileId ? <small>DEFAULT</small> : null}
                </span>
              </span>

              <span className="mt-profile-card__summary-row">
                {profile.description ? (
                  <span className="mt-profile-card__description">{profile.description}</span>
                ) : (
                  <span className="mt-profile-card__description">Display-only profile preview.</span>
                )}
              </span>

              <span className="mt-profile-card__facts">
                <ProfileFact label="Mode" value={profile.mode} />
                <ProfileFact
                  label="DB"
                  value={profile.database.name
                    ? `${profile.database.type} · ${profile.database.name}`
                    : profile.database.type}
                />
                <ProfileFact label="API" value={profile.baseUrl || "local / IPC"} />
                <ProfileFact label="Data" value={formatDatabaseTarget(profile)} />
              </span>
            </span>
          </button>
        ))}
      </div>
    </aside>
  );
}

function ProfileFact({ label, value }: { label: string; value: string }) {
  return (
    <span className="mt-profile-card__fact">
      <small>{label}</small>
      <code>{value}</code>
    </span>
  );
}

function formatDatabaseTarget(profile: ApiProfile): string {
  if (profile.database.type === "sqlite") {
    return profile.database.path || "SQLite path not set";
  }

  if (profile.database.type === "postgres") {
    const host = profile.database.host || "host not set";
    return profile.database.port ? `${host}:${profile.database.port}` : host;
  }

  return profile.database.path || profile.database.host || "auto / detected";
}
