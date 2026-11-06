import type { RuntimeSourceMode } from "./apiProfile.types";

const STORAGE_KEY = "doseark.runtime-source.v1";

export type RuntimeSourcePreference = {
  source: RuntimeSourceMode;
  profileId: string | null;
};

const DEFAULT_PREFERENCE: RuntimeSourcePreference = {
  source: "env",
  profileId: null,
};

export function loadRuntimeSourcePreference(): RuntimeSourcePreference {
  try {
    const raw = globalThis.localStorage?.getItem(STORAGE_KEY);
    if (!raw) return DEFAULT_PREFERENCE;

    const parsed = JSON.parse(raw) as Partial<RuntimeSourcePreference>;
    return {
      source: parsed.source === "profile" ? "profile" : "env",
      profileId: typeof parsed.profileId === "string" && parsed.profileId.trim()
        ? parsed.profileId.trim()
        : null,
    };
  } catch {
    return DEFAULT_PREFERENCE;
  }
}

export function saveRuntimeSourcePreference(preference: RuntimeSourcePreference): void {
  try {
    globalThis.localStorage?.setItem(STORAGE_KEY, JSON.stringify(preference));
  } catch {
    // Temporary browser preference only; failure is non-fatal.
  }
}
