import {
  createContext,
  useContext,
  useEffect,
  useMemo,
  useState,
  type PropsWithChildren,
} from "react";

import { createLogger } from "@/core/logger/logger";

const STORAGE_KEY = "doseark.debug_mode";
const LEGACY_STORAGE_KEY = "medtaker.debug_mode";
const logger = createLogger("settings");

type DebugModeContextValue = {
  enabled: boolean;
  setEnabled: (enabled: boolean) => void;
  toggleDebugMode: () => void;
};

const DebugModeContext = createContext<DebugModeContextValue | null>(null);

export function DebugModeProvider({ children }: PropsWithChildren) {
  const [enabled, setEnabled] = useState(loadDebugMode);

  useEffect(() => {
    try {
      localStorage.setItem(STORAGE_KEY, enabled ? "1" : "0");
      logger.debug("Debug mode changed", { enabled });
    } catch (error) {
      logger.warn("Could not persist debug mode", error);
    }
  }, [enabled]);

  const value = useMemo(
    () => ({
      enabled,
      setEnabled,
      toggleDebugMode: () => setEnabled((current) => !current),
    }),
    [enabled],
  );

  return <DebugModeContext.Provider value={value}>{children}</DebugModeContext.Provider>;
}

export function useDebugMode() {
  const context = useContext(DebugModeContext);

  if (!context) {
    throw new Error("useDebugMode must be used inside DebugModeProvider");
  }

  return context;
}

function loadDebugMode(): boolean {
  try {
    const current = localStorage.getItem(STORAGE_KEY);
    if (current !== null) return current === "1";

    const legacy = localStorage.getItem(LEGACY_STORAGE_KEY);
    if (legacy === null) return false;

    localStorage.setItem(STORAGE_KEY, legacy);
    localStorage.removeItem(LEGACY_STORAGE_KEY);
    logger.info("Migrated debug mode to DoseArk storage key");
    return legacy === "1";
  } catch (error) {
    logger.warn("Could not load debug mode", error);
    return false;
  }
}
