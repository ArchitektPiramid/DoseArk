import { useTranslation } from "react-i18next";

import { useDoseAmountPickerState } from "../hooks/useDoseAmountPickerState";
import { formatDoseMeta, formatDoseNumber } from "../utils/dosingFormat";
import {
  DoseAmountModeSelector,
  SplitDosePanel,
  WholeDosePanel,
} from "./DoseAmountControls";
import { DoseMassInput } from "./DoseMassInput";

type Props = {
  unit: string;
  unitWeightMg?: number | null;
  stockBefore?: number;
  value: number;
  onChange: (value: number) => void;
};

export function DoseAmountPicker({ unit, unitWeightMg, stockBefore, value, onChange }: Props) {
  const { t } = useTranslation("dosing");
  const state = useDoseAmountPickerState({ value, unitWeightMg, stockBefore, onChange });
  const summary = (
    <DoseSummary
      label={t("form.selectedDoseLabel")}
      value={`${formatDoseNumber(value)} ${unit}`}
      meta={formatDoseMeta(value, unitWeightMg)}
    />
  );

  return (
    <section className="mt-dose-picker">
      <div className="mt-dose-picker__header">
        <DoseAmountModeSelector
          mode={state.mode}
          wholeLabel={t("form.wholeUnitsMode")}
          splitLabel={t("form.splitUnitsMode")}
          onWhole={state.activateWhole}
          onSplit={state.activateSplit}
        />

        {stockBefore !== undefined && state.stockAfter !== undefined ? (
          <div className="mt-dose-inline-preview">
            {t("form.stockPreview", {
              before: formatDoseNumber(stockBefore),
              after: formatDoseNumber(Math.max(state.stockAfter, 0)),
            })}
          </div>
        ) : null}
      </div>

      {state.mode === "whole" ? (
        <WholeDosePanel
          label={t("form.wholeUnits")}
          value={state.wholeUnits}
          summary={summary}
          onChange={state.updateWholeDose}
        />
      ) : (
        <SplitDosePanel
          wholeLabel={t("form.wholeUnits")}
          splitIntoLabel={t("form.splitInto")}
          partsTakenLabel={t("form.partsTaken")}
          wholeUnits={state.wholeUnits}
          splitParts={state.splitParts}
          partsTaken={state.partsTaken}
          summary={summary}
          onWholeUnitsChange={state.updateSplitWholeUnits}
          onSplitPartsChange={state.updateSplitParts}
          onPartsTakenChange={state.updatePartsTaken}
        />
      )}

      {state.hasUnitWeight && unitWeightMg ? (
        <DoseMassInput
          value={state.massInput}
          maxMassMg={state.maxMassMg}
          fallbackUnitWeightMg={unitWeightMg}
          label={t("form.doseMass")}
          hint={(max) => t("form.doseMassHint", { max })}
          placeholder={t("form.doseMassPlaceholder")}
          unitLabel={t("form.doseMassUnit")}
          onChange={state.updateMassInput}
          onBlur={state.normalizeMassInput}
        />
      ) : null}
    </section>
  );
}

function DoseSummary({ label, value, meta }: { label: string; value: string; meta: string }) {
  return (
    <div className="mt-dose-selected">
      <span className="mt-dose-selected__label">{label}</span>
      <strong>{value}</strong>
      <span className="mt-dose-selected__meta">{meta}</span>
    </div>
  );
}
