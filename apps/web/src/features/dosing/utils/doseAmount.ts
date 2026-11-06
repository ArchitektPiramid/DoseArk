export const SPLIT_PART_OPTIONS = [2, 3, 4, 5, 6, 8, 10] as const;

export type DoseMode = "whole" | "split";

export type DoseSplitState = {
  wholeUnits: number;
  splitParts: number;
  partsTaken: number;
};

export function calculateSplitDose(
  wholeUnits: number,
  partsTaken: number,
  splitParts: number,
): number {
  return wholeUnits + partsTaken / splitParts;
}

export function createSplitState(value: number): DoseSplitState {
  const safeValue = Number.isFinite(value) && value > 0 ? value : 0.5;
  const wholeUnits = Math.max(0, Math.floor(safeValue));
  const fraction = safeValue - wholeUnits;

  if (fraction <= 0.000_001) {
    return {
      wholeUnits: Math.max(0, wholeUnits - 1),
      splitParts: 2,
      partsTaken: 1,
    };
  }

  let bestParts = 2;
  let bestTaken = 1;
  let bestDifference = Number.POSITIVE_INFINITY;

  for (const parts of SPLIT_PART_OPTIONS) {
    const taken = clampInteger(Math.round(fraction * parts), 1, parts - 1);
    const difference = Math.abs(fraction - taken / parts);

    if (difference < bestDifference) {
      bestDifference = difference;
      bestParts = parts;
      bestTaken = taken;
    }
  }

  return {
    wholeUnits,
    splitParts: bestParts,
    partsTaken: bestTaken,
  };
}

export function getInitialDoseMode(value: number): DoseMode {
  return isWholeNumber(value) && value >= 1 ? "whole" : "split";
}

export function isWholeNumber(value: number): boolean {
  return Number.isFinite(value) && Math.abs(value - Math.round(value)) < 0.000_001;
}

export function clampInteger(value: number, min: number, max: number): number {
  if (!Number.isFinite(value)) return min;
  return Math.min(Math.max(Math.round(value), min), max);
}

export function parseDoseNumberInput(value: string): number | null {
  const normalized = value.trim().replace(",", ".");
  if (normalized.length === 0) return null;

  const parsed = Number(normalized);
  return Number.isFinite(parsed) ? parsed : null;
}
