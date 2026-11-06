import { TextInput } from "@/core/ui";
import { formatDoseNumber } from "../utils/dosingFormat";

export function DoseMassInput(props: {
  value: string;
  maxMassMg?: number;
  fallbackUnitWeightMg: number;
  label: string;
  hint: (max: string) => string;
  placeholder: string;
  unitLabel: string;
  onChange: (value: string) => void;
  onBlur: () => void;
}) {
  return (
    <div className="mt-dose-mass-row">
      <div className="mt-dose-mass-row__heading">
        <span className="mt-label">{props.label}</span>
        <span className="mt-dose-mass-row__hint">
          {props.hint(formatDoseNumber(props.maxMassMg ?? props.fallbackUnitWeightMg))}
        </span>
      </div>

      <div className="mt-dose-mass-row__control">
        <TextInput
          type="number"
          min={0}
          max={props.maxMassMg}
          step={0.01}
          value={props.value}
          placeholder={props.placeholder}
          onChange={(event) => props.onChange(event.target.value)}
          onBlur={props.onBlur}
        />
        <span className="mt-dose-mass-row__unit">{props.unitLabel}</span>
      </div>
    </div>
  );
}
