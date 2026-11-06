import { useAppLanguage } from "@/core/i18n/useAppLanguage";
import { Button, Panel } from "@/core/ui";
import { APP_NAME } from "@/core/version/appVersion";

import { SettingsDetailList } from "./SettingsDetailList";

const LANGUAGE_NAMES: Record<string, string> = {
  en: "English",
  pl: "Polski",
};

export function UiPreferencesPanel() {
  const language = useAppLanguage();

  return (
    <Panel title="UI preferences">
      <p className="mt-settings-panel-description">
        Language selection for this browser client. Version information is shown in the active profile runtime tab.
      </p>

      <div className="mt-settings-language-actions">
        {language.supportedLanguages.map((item) => (
          <Button
            key={item.code}
            variant={language.language === item.code ? "primary" : "default"}
            className="mt-settings-language-button"
            onClick={() => void language.setLanguage(item.code)}
          >
            {item.label}
          </Button>
        ))}
      </div>

      <SettingsDetailList
        rows={[
          {
            label: "Current language",
            value: <code>{LANGUAGE_NAMES[language.language] ?? language.language.toUpperCase()}</code>,
          },
          { label: "Application", value: <code>{APP_NAME}</code> },
        ]}
      />
    </Panel>
  );
}
