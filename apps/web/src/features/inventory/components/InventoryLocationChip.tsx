import type { InventoryLot, StorageLocation } from "@doseark/types";

import { VisualIcon } from "@/core/visualTags";

type Props = {
  lot: InventoryLot;
  location?: StorageLocation;
};

export function InventoryLocationChip({ lot, location }: Props) {
  const label = location?.name ?? lot.storage_location ?? "-";

  if (label === "-") {
    return <span className="mt-muted">-</span>;
  }

  return (
    <span className="mt-inventory-location-chip">
      <span
        className="mt-inventory-location-chip__icon"
        style={{ color: location?.color ?? undefined }}
      >
        <VisualIcon name={location?.icon ?? "box"} size={13} />
      </span>
      <span className="mt-inventory-location-chip__label">{label}</span>
    </span>
  );
}
