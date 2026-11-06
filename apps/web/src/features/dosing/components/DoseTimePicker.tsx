import { useTranslation } from "react-i18next";

import {
  patchDoseTimeFields,
  type DoseTimeMode,
  type DoseTimeSelection,
} from "../utils/doseTime";
import { DoseClockPicker } from "./DoseClockPicker";
import { DoseDatePicker } from "./DoseDatePicker";

export type { DoseTimeFields, DoseTimeMode, DoseTimeSelection } from "../utils/doseTime";
export {
  createInitialDoseTimeSelection,
  doseTimeSelectionFromIso,
  doseTimeSelectionToIso,
} from "../utils/doseTime";

type Props = {
  value: DoseTimeSelection;
  onChange: (value: DoseTimeSelection) => void;
};

export function DoseTimePicker({ value, onChange }: Props) {
  const { t } = useTranslation("dosing");

  function setMode(mode: DoseTimeMode) {
    onChange({ ...value, mode });
  }

  function patchCustom(patch: Partial<DoseTimeSelection["custom"]>) {
    onChange({ ...value, custom: patchDoseTimeFields(value.custom, patch) });
  }

  return (
    <section className="mt-dose-section">
      <div className="mt-dose-mode-grid">
        <button
          type="button"
          className={value.mode === "now" ? "mt-dose-mode is-active" : "mt-dose-mode"}
          onClick={() => setMode("now")}
        >
          {t("form.now")}
        </button>
        <button
          type="button"
          className={value.mode === "custom" ? "mt-dose-mode is-active" : "mt-dose-mode"}
          onClick={() => setMode("custom")}
        >
          {t("form.custom")}
        </button>
      </div>

      {value.mode === "custom" ? (
        <div className="mt-dose-date-time-grid">
          <DoseDatePicker value={value.custom} onChange={patchCustom} />
          <DoseClockPicker value={value.custom} onChange={patchCustom} />
        </div>
      ) : null}
    </section>
  );
}
