import {
  createContext,
  type ReactNode,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";

import { getFallbackApiBaseUrl, setActiveApiBaseUrl } from "@/app/client";
import { createLogger } from "@/core/logger/logger";
import type { ApiProfile, RuntimeConfigSource, RuntimeSourceMode } from "./apiProfile.types";
import {
  loadRuntimeSourcePreference,
  saveRuntimeSourcePreference,
} from "./runtimeSourcePreference";
import { loadRuntimeApiConfig, normalizeRuntimeProfile } from "./runtimeApiConfig";

const logger = createLogger("settings");

type ApiProfilesContextValue = {
  profiles: ApiProfile[];
  defaultProfileId: string | null;
  selectedProfileId: string | null;
  selectedProfile: ApiProfile | null;
  runtimeSource: RuntimeSourceMode;
  effectiveApiBaseUrl: string;
  configSource: RuntimeConfigSource | null;
  isLoading: boolean;
  error: string | null;
  setSelectedProfileId: (profileId: string) => void;
  setRuntimeSource: (source: RuntimeSourceMode) => void;
};

const ApiProfilesContext = createContext<ApiProfilesContextValue | null>(null);

export function ApiProfilesProvider({ children }: { children: ReactNode }) {
  const initialPreference = useMemo(loadRuntimeSourcePreference, []);
  const [profiles, setProfiles] = useState<ApiProfile[]>([]);
  const [defaultProfileId, setDefaultProfileId] = useState<string | null>(null);
  const [selectedProfileId, setSelectedProfileIdState] = useState<string | null>(
    initialPreference.profileId,
  );
  const [runtimeSource, setRuntimeSourceState] = useState<RuntimeSourceMode>(initialPreference.source);
  const [configSource, setConfigSource] = useState<RuntimeConfigSource | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;

    void loadRuntimeApiConfig()
      .then(({ config, source }) => {
        if (cancelled) return;

        const fileProfiles = config.apiProfiles.map(normalizeRuntimeProfile);
        const defaultId = fileProfiles.some((profile) => profile.id === config.defaultApiProfileId)
          ? config.defaultApiProfileId
          : fileProfiles[0]?.id ?? null;

        setProfiles(fileProfiles);
        setDefaultProfileId(defaultId);
        setSelectedProfileIdState((current) =>
          current && fileProfiles.some((profile) => profile.id === current)
            ? current
            : defaultId,
        );
        setConfigSource(source);
        setError(null);
        setIsLoading(false);

        logger.info("Profiles loaded", {
          profileCount: fileProfiles.length,
          defaultProfileId: defaultId,
          source: source.kind,
          configUrl: source.runtimeUrl,
        });
      })
      .catch((loadError) => {
        if (cancelled) return;
        setError(loadError instanceof Error ? loadError.message : "Could not load profiles");
        setIsLoading(false);
        logger.error("Failed to load profiles", loadError);
      });

    return () => {
      cancelled = true;
    };
  }, []);

  const selectedProfile = useMemo(
    () => profiles.find((profile) => profile.id === selectedProfileId) ?? null,
    [profiles, selectedProfileId],
  );

  const effectiveApiBaseUrl = useMemo(() => {
    if (runtimeSource === "profile" && selectedProfile?.baseUrl) {
      return selectedProfile.baseUrl;
    }
    return getFallbackApiBaseUrl();
  }, [runtimeSource, selectedProfile]);

  useEffect(() => {
    setActiveApiBaseUrl(effectiveApiBaseUrl);
  }, [effectiveApiBaseUrl]);

  useEffect(() => {
    saveRuntimeSourcePreference({
      source: runtimeSource,
      profileId: selectedProfileId,
    });
  }, [runtimeSource, selectedProfileId]);

  const setSelectedProfileId = useCallback((profileId: string) => {
    setSelectedProfileIdState(profileId);
  }, []);

  const setRuntimeSource = useCallback((source: RuntimeSourceMode) => {
    setRuntimeSourceState(source);
  }, []);

  const value = useMemo<ApiProfilesContextValue>(() => ({
    profiles,
    defaultProfileId,
    selectedProfileId,
    selectedProfile,
    runtimeSource,
    effectiveApiBaseUrl,
    configSource,
    isLoading,
    error,
    setSelectedProfileId,
    setRuntimeSource,
  }), [
    profiles,
    defaultProfileId,
    selectedProfileId,
    selectedProfile,
    runtimeSource,
    effectiveApiBaseUrl,
    configSource,
    isLoading,
    error,
    setSelectedProfileId,
    setRuntimeSource,
  ]);

  return (
    <ApiProfilesContext.Provider value={value}>
      {children}
    </ApiProfilesContext.Provider>
  );
}

export function useApiProfiles(): ApiProfilesContextValue {
  const value = useContext(ApiProfilesContext);
  if (!value) throw new Error("useApiProfiles must be used inside ApiProfilesProvider");
  return value;
}
