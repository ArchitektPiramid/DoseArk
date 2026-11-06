import type { ReactNode } from "react";

import { SelectInput, TextInput } from "@/core/ui";

import { SPLIT_PART_OPTIONS, type DoseMode } from "../utils/doseAmount";

export function DoseAmountModeSelector(props: {
  mode: DoseMode;
  wholeLabel: string;
  splitLabel: string;
  onWhole: () => void;
  onSplit: () => void;
}) {
  return (
    <div className="mt-dose-mode-grid">
      <button
        type="button"
        className={props.mode === "whole" ? "mt-dose-mode is-active" : "mt-dose-mode"}
        onClick={props.onWhole}
      >
        {props.wholeLabel}
      </button>
      <button
        type="button"
        className={props.mode === "split" ? "mt-dose-mode is-active" : "mt-dose-mode"}
        onClick={props.onSplit}
      >
        {props.splitLabel}
      </button>
    </div>
  );
}

export function WholeDosePanel(props: {
  label: string;
  value: number;
  summary: ReactNode;
  onChange: (value: number) => void;
}) {
  return (
    <div className="mt-dose-amount-panel mt-dose-amount-panel--whole">
      <CompactNumberField label={props.label} value={props.value} min={0} step={1} onChange={props.onChange} />
      {props.summary}
    </div>
  );
}

export function SplitDosePanel(props: {
  wholeLabel: string;
  splitIntoLabel: string;
  partsTakenLabel: string;
  wholeUnits: number;
  splitParts: number;
  partsTaken: number;
  summary: ReactNode;
  onWholeUnitsChange: (value: number) => void;
  onSplitPartsChange: (value: number) => void;
  onPartsTakenChange: (value: number) => void;
}) {
  return (
    <div className="mt-dose-amount-panel mt-dose-amount-panel--split">
      <CompactNumberField
        label={props.wholeLabel}
        value={props.wholeUnits}
        min={0}
        step={1}
        onChange={props.onWholeUnitsChange}
      />

      <label className="mt-dose-compact-field">
        <span className="mt-label">{props.splitIntoLabel}</span>
        <SelectInput
          value={String(props.splitParts)}
          onChange={(event) => props.onSplitPartsChange(Number(event.target.value))}
        >
          {SPLIT_PART_OPTIONS.map((option) => (
            <option key={option} value={option}>
              {option}
            </option>
          ))}
        </SelectInput>
      </label>

      <CompactNumberField
        label={props.partsTakenLabel}
        value={props.partsTaken}
        min={1}
        max={Math.max(1, props.splitParts - 1)}
        step={1}
        onChange={props.onPartsTakenChange}
      />

      <div className="mt-dose-split-slider">
        <input
          className="mt-dose-range"
          type="range"
          min={1}
          max={Math.max(1, props.splitParts - 1)}
          step={1}
          value={props.partsTaken}
          onChange={(event) => props.onPartsTakenChange(Number(event.target.value))}
        />
      </div>

      {props.summary}
    </div>
  );
}

function CompactNumberField(props: {
  label: string;
  value: number;
  min?: number;
  max?: number;
  step?: number;
  onChange: (value: number) => void;
}) {
  return (
    <label className="mt-dose-compact-field">
      <span className="mt-label">{props.label}</span>
      <TextInput
        type="number"
        min={props.min}
        max={props.max}
        step={props.step}
        value={props.value}
        onChange={(event) => props.onChange(Number(event.target.value))}
      />
    </label>
  );
}
