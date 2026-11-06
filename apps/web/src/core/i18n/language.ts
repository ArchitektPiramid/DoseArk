import { createLogger } from "@/core/logger/logger";

export type AppLanguage = "en" | "pl";

export const DEFAULT_LANGUAGE: AppLanguage = "en";

export const SUPPORTED_LANGUAGES: Array<{
  code: AppLanguage;
  label: string;
}> = [
  { code: "en", label: "EN" },
  { code: "pl", label: "PL" },
];

export const LANGUAGE_STORAGE_KEY = "doseark.language";
export const LEGACY_LANGUAGE_STORAGE_KEY = "medtaker.language";

const logger = createLogger("settings");

export function isAppLanguage(value: string | null): value is AppLanguage {
  return value === "en" || value === "pl";
}

export function getInitialLanguage(): AppLanguage {
  const stored = readStoredLanguage();

  if (isAppLanguage(stored)) {
    return stored;
  }

  const browserLanguage = navigator.language.toLowerCase();

  if (browserLanguage.startsWith("pl")) {
    return "pl";
  }

  return DEFAULT_LANGUAGE;
}

function readStoredLanguage(): string | null {
  try {
    const current = localStorage.getItem(LANGUAGE_STORAGE_KEY);
    if (current !== null) return current;

    const legacy = localStorage.getItem(LEGACY_LANGUAGE_STORAGE_KEY);
    if (legacy === null) return null;

    localStorage.setItem(LANGUAGE_STORAGE_KEY, legacy);
    localStorage.removeItem(LEGACY_LANGUAGE_STORAGE_KEY);
    logger.info("Migrated UI language to DoseArk storage key");
    return legacy;
  } catch (error) {
    logger.warn("Could not read stored UI language", error);
    return null;
  }
}
