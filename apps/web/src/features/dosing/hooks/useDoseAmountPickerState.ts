import { useEffect, useMemo, useState } from "react";

import { formatDoseNumber } from "../utils/dosingFormat";
import {
  calculateSplitDose,
  clampInteger,
  createSplitState,
  getInitialDoseMode,
  isWholeNumber,
  parseDoseNumberInput,
  type DoseMode,
} from "../utils/doseAmount";

type Input = {
  value: number;
  unitWeightMg?: number | null;
  stockBefore?: number;
  onChange: (value: number) => void;
};

export function useDoseAmountPickerState({ value, unitWeightMg, stockBefore, onChange }: Input) {
  const initialSplit = createSplitState(value);
  const initialMode = getInitialDoseMode(value);
  const [mode, setMode] = useState<DoseMode>(initialMode);
  const [wholeUnits, setWholeUnits] = useState(
    initialMode === "whole" ? Math.max(1, Math.round(value || 1)) : initialSplit.wholeUnits,
  );
  const [splitParts, setSplitParts] = useState(initialSplit.splitParts);
  const [partsTaken, setPartsTaken] = useState(initialSplit.partsTaken);
  const [massInput, setMassInput] = useState("");

  const hasUnitWeight = unitWeightMg !== undefined && unitWeightMg !== null && unitWeightMg > 0;
  const stockAfter = stockBefore === undefined ? undefined : stockBefore - value;
  const maxMassMg = useMemo(() => {
    if (!hasUnitWeight || unitWeightMg === undefined || unitWeightMg === null) return undefined;
    return Math.max(0, (stockBefore ?? Math.max(value, 1)) * unitWeightMg);
  }, [hasUnitWeight, stockBefore, unitWeightMg, value]);

  useEffect(() => {
    if (!hasUnitWeight || unitWeightMg === undefined || unitWeightMg === null) {
      setMassInput("");
      return;
    }

    setMassInput(formatDoseNumber(value * unitWeightMg));
  }, [hasUnitWeight, unitWeightMg, value]);

  function activateWhole() {
    const nextWholeUnits = Math.max(1, Math.round(value || 1));
    setMode("whole");
    setWholeUnits(nextWholeUnits);
    onChange(nextWholeUnits);
  }

  function activateSplit() {
    const nextSplit = isWholeNumber(value)
      ? { wholeUnits: Math.max(0, Math.floor(value) - 1), splitParts: 2, partsTaken: 1 }
      : createSplitState(value);

    setMode("split");
    setWholeUnits(nextSplit.wholeUnits);
    setSplitParts(nextSplit.splitParts);
    setPartsTaken(nextSplit.partsTaken);
    onChange(calculateSplitDose(nextSplit.wholeUnits, nextSplit.partsTaken, nextSplit.splitParts));
  }

  function updateWholeDose(nextValue: number) {
    const nextWholeUnits = clampInteger(nextValue, 0, 10_000);
    setWholeUnits(nextWholeUnits);
    onChange(nextWholeUnits);
  }

  function updateSplitWholeUnits(nextValue: number) {
    const nextWholeUnits = clampInteger(nextValue, 0, 10_000);
    setWholeUnits(nextWholeUnits);
    onChange(calculateSplitDose(nextWholeUnits, partsTaken, splitParts));
  }

  function updateSplitParts(nextValue: number) {
    const nextSplitParts = clampInteger(nextValue, 2, 10);
    const nextPartsTaken = Math.min(partsTaken, nextSplitParts - 1);
    setSplitParts(nextSplitParts);
    setPartsTaken(nextPartsTaken);
    onChange(calculateSplitDose(wholeUnits, nextPartsTaken, nextSplitParts));
  }

  function updatePartsTaken(nextValue: number) {
    const nextPartsTaken = clampInteger(nextValue, 1, Math.max(1, splitParts - 1));
    setPartsTaken(nextPartsTaken);
    onChange(calculateSplitDose(wholeUnits, nextPartsTaken, splitParts));
  }

  function updateMassInput(rawValue: string) {
    setMassInput(rawValue);
    if (!hasUnitWeight || unitWeightMg === undefined || unitWeightMg === null) return;

    const parsed = parseDoseNumberInput(rawValue);
    if (parsed === null) {
      onChange(0);
      return;
    }

    const nextDose = Math.max(0, parsed) / unitWeightMg;
    syncControlsToDose(nextDose);
    onChange(nextDose);
  }

  function normalizeMassInput() {
    const parsed = parseDoseNumberInput(massInput);
    setMassInput(parsed === null ? "0" : formatDoseNumber(Math.max(0, parsed)));
  }

  function syncControlsToDose(nextDose: number) {
    if (nextDose <= 0) {
      setMode("whole");
      setWholeUnits(0);
      return;
    }

    if (isWholeNumber(nextDose) && nextDose >= 1) {
      setMode("whole");
      setWholeUnits(Math.round(nextDose));
      return;
    }

    const nextSplit = createSplitState(nextDose);
    setMode("split");
    setWholeUnits(nextSplit.wholeUnits);
    setSplitParts(nextSplit.splitParts);
    setPartsTaken(nextSplit.partsTaken);
  }

  return {
    mode,
    wholeUnits,
    splitParts,
    partsTaken,
    massInput,
    hasUnitWeight,
    stockAfter,
    maxMassMg,
    activateWhole,
    activateSplit,
    updateWholeDose,
    updateSplitWholeUnits,
    updateSplitParts,
    updatePartsTaken,
    updateMassInput,
    normalizeMassInput,
  };
}
