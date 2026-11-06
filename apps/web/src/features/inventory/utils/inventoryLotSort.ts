import type { InventoryLot, StorageLocation } from "@doseark/types";

import type { TableSortState } from "@/core/ui";

import { INVENTORY_LOT_STATUS_OPTIONS } from "./inventoryLotStatus";

export type InventoryLotSortKey =
  | "product"
  | "lot"
  | "remaining"
  | "status"
  | "expires"
  | "storage";

export type InventoryLotSortState = TableSortState<InventoryLotSortKey>;

type SortContext = {
  productNameById: Map<string, string>;
  storageLocationById: Map<string, StorageLocation>;
};

const TEXT_COLLATOR = new Intl.Collator(undefined, {
  numeric: true,
  sensitivity: "base",
});

function compareOptionalText(
  left: string | null | undefined,
  right: string | null | undefined,
  direction: InventoryLotSortState["direction"],
): number {
  const leftValue = left?.trim() ?? "";
  const rightValue = right?.trim() ?? "";

  if (!leftValue || !rightValue) {
    if (!leftValue && !rightValue) return 0;
    return !leftValue ? 1 : -1;
  }

  return TEXT_COLLATOR.compare(leftValue, rightValue) * (direction === "asc" ? 1 : -1);
}

function getStorageLabel(
  lot: InventoryLot,
  storageLocationById: Map<string, StorageLocation>,
) {
  return lot.storage_location_id
    ? storageLocationById.get(lot.storage_location_id)?.name ?? lot.storage_location
    : lot.storage_location;
}

export function sortInventoryLots(
  lots: InventoryLot[],
  sort: InventoryLotSortState,
  { productNameById, storageLocationById }: SortContext,
): InventoryLot[] {
  const multiplier = sort.direction === "asc" ? 1 : -1;

  return [...lots].sort((left, right) => {
    let result = 0;

    switch (sort.key) {
      case "product":
        result = compareOptionalText(
          productNameById.get(left.product_id),
          productNameById.get(right.product_id),
          sort.direction,
        );
        break;
      case "lot":
        result = compareOptionalText(left.lot_number, right.lot_number, sort.direction);
        break;
      case "remaining":
        result = (left.units_remaining - right.units_remaining) * multiplier;
        break;
      case "status": {
        const leftRank = INVENTORY_LOT_STATUS_OPTIONS.indexOf(left.status);
        const rightRank = INVENTORY_LOT_STATUS_OPTIONS.indexOf(right.status);
        result = (leftRank - rightRank) * multiplier;
        break;
      }
      case "expires":
        result = compareOptionalText(left.expires_at, right.expires_at, sort.direction);
        break;
      case "storage":
        result = compareOptionalText(
          getStorageLabel(left, storageLocationById),
          getStorageLabel(right, storageLocationById),
          sort.direction,
        );
        break;
    }

    if (result !== 0) return result;

    const byProduct = TEXT_COLLATOR.compare(
      productNameById.get(left.product_id) ?? "",
      productNameById.get(right.product_id) ?? "",
    );
    if (byProduct !== 0) return byProduct;

    const byLot = TEXT_COLLATOR.compare(left.lot_number ?? "", right.lot_number ?? "");
    return byLot || TEXT_COLLATOR.compare(left.id, right.id);
  });
}
