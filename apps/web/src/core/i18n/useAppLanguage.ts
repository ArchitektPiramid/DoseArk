import { useTranslation } from "react-i18next";

import { createLogger } from "@/core/logger/logger";
import { LANGUAGE_STORAGE_KEY, type AppLanguage, SUPPORTED_LANGUAGES } from "./language";

const logger = createLogger("settings");

export function useAppLanguage() {
  const { i18n } = useTranslation();

  async function setLanguage(language: AppLanguage) {
    try {
      localStorage.setItem(LANGUAGE_STORAGE_KEY, language);
    } catch (error) {
      logger.warn("Could not persist UI language", error);
    }

    const previousLanguage = i18n.language;
    await i18n.changeLanguage(language);
    logger.info("UI language changed", { previousLanguage, language });
  }

  return {
    language: i18n.language as AppLanguage,
    setLanguage,
    supportedLanguages: SUPPORTED_LANGUAGES,
  };
}
