import type { InventoryLot } from "@doseark/types";

export type InventoryLotStockLevel = "normal" | "warning" | "danger";

export function getInventoryLotRemainingPercent(
  remaining: number | null | undefined,
  initial: number | null | undefined,
): number | null {
  if (
    remaining === null ||
    remaining === undefined ||
    initial === null ||
    initial === undefined ||
    !Number.isFinite(remaining) ||
    !Number.isFinite(initial) ||
    initial <= 0
  ) {
    return null;
  }

  return (remaining / initial) * 100;
}

export function getInventoryLotStockLevel(
  remainingPercent: number | null,
): InventoryLotStockLevel {
  if (remainingPercent === null) {
    return "normal";
  }

  if (remainingPercent < 5) {
    return "danger";
  }

  if (remainingPercent < 15) {
    return "warning";
  }

  return "normal";
}

export function getInventoryLotAlertLevel(
  lot: InventoryLot,
): Exclude<InventoryLotStockLevel, "normal"> | null {
  if (lot.status !== "active") {
    return null;
  }

  const percent = getInventoryLotRemainingPercent(
    lot.units_remaining,
    lot.units_initial,
  );
  const level = getInventoryLotStockLevel(percent);

  return level === "normal" ? null : level;
}

export function clampPercent(value: number | null): number {
  if (value === null) {
    return 0;
  }

  return Math.min(Math.max(value, 0), 100);
}
