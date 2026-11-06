import type { InventoryLotStatus } from "@doseark/types";
import type { BadgeTone } from "@/core/ui";

export const INVENTORY_LOT_STATUS_OPTIONS: InventoryLotStatus[] = [
  "active",
  "reserve",
  "on_hold",
  "empty",
  "expired",
  "discarded",
  "archived",
];

export function getInventoryLotStatusTone(status: InventoryLotStatus): BadgeTone {
  switch (status) {
    case "active":
      return "ok";
    case "reserve":
    case "on_hold":
      return "warn";
    case "empty":
    case "expired":
    case "discarded":
    case "archived":
      return "danger";
    default:
      return "default";
  }
}

export function getInventoryLotStatusLabel(status: InventoryLotStatus): string {
  return status.replaceAll("_", " ");
}
