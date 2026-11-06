import { useTranslation } from "react-i18next";

import { SelectInput } from "@/core/ui";
import { HOURS, MINUTES, padTwo, type DoseTimeFields } from "../utils/doseTime";

export function DoseClockPicker(props: {
  value: DoseTimeFields;
  onChange: (patch: Partial<DoseTimeFields>) => void;
}) {
  const { t } = useTranslation("dosing");

  return (
    <div className="mt-dose-date-time-field">
      <span className="mt-label">{t("form.time")}</span>
      <div className="mt-dose-clock-picker">
        <SelectInput
          aria-label={t("form.hour")}
          value={props.value.hour}
          onChange={(event) => props.onChange({ hour: Number(event.target.value) })}
        >
          {HOURS.map((hour) => (
            <option key={hour} value={hour}>{padTwo(hour)}</option>
          ))}
        </SelectInput>

        <span className="mt-dose-clock-picker__separator" aria-hidden="true">:</span>

        <SelectInput
          aria-label={t("form.minute")}
          value={props.value.minute}
          onChange={(event) => props.onChange({ minute: Number(event.target.value) })}
        >
          {MINUTES.map((minute) => (
            <option key={minute} value={minute}>{padTwo(minute)}</option>
          ))}
        </SelectInput>
      </div>
    </div>
  );
}
