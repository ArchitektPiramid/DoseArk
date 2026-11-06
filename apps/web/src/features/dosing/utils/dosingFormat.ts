import {
  formatDateLocal,
  formatNumber,
  formatStockQuantity,
  formatTimeLocal,
} from "@/core/format";

export function formatDoseNumber(value: number): string {
  return formatNumber(value, { maxDecimals: 2 });
}

export function formatDoseAmount(amount: number, unit: string): string {
  return `${formatDoseNumber(amount)} ${unit}`;
}

export function formatDoseAmountCompact(amount: number, unit: string): string {
  return `${formatDoseNumber(amount)} ${formatCompactDoseUnit(unit)}`;
}

export function formatMassMg(value: number): string {
  if (value >= 1000) {
    return `${formatNumber(value / 1000, { maxDecimals: 2 })} g`;
  }

  return `${formatNumber(value, { maxDecimals: 2 })} mg`;
}

export function formatDoseMeta(amount: number, unitWeightMg?: number | null): string {
  const percent = `${formatNumber(amount * 100, { maxDecimals: 1 })}%`;

  if (unitWeightMg !== undefined && unitWeightMg !== null && unitWeightMg > 0) {
    return `${formatMassMg(amount * unitWeightMg)} · ${percent}`;
  }

  return percent;
}

export function formatLotStock(remaining: number, initial: number): string {
  return `${formatStockQuantity(remaining)}/${formatStockQuantity(initial)}`;
}

export function formatChronologicalStock(
  before: number | null,
  after: number | null,
): string {
  if (before === null || after === null) {
    return "-";
  }

  return `${formatStockQuantity(before)} → ${formatStockQuantity(after)}`;
}

function formatCompactDoseUnit(unit: string): string {
  const normalized = unit.trim().toLowerCase();

  const abbreviations: Record<string, string> = {
    tablet: "tab.",
    tablets: "tab.",
    capsule: "cap.",
    capsules: "cap.",
    softgel: "softgel",
    softgels: "softgel",
    unit: "unit",
    units: "unit",
  };

  return abbreviations[normalized] ?? unit;
}

export { formatDateLocal, formatTimeLocal };
