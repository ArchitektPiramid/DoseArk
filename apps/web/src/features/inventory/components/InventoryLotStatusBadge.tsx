import type { InventoryLotStatus } from "@doseark/types";

import { Badge } from "@/core/ui";

import {
  getInventoryLotStatusLabel,
  getInventoryLotStatusTone,
} from "../utils/inventoryLotStatus";

type Props = {
  status: InventoryLotStatus;
};

export function InventoryLotStatusBadge({ status }: Props) {
  return (
    <Badge tone={getInventoryLotStatusTone(status)}>
      {getInventoryLotStatusLabel(status)}
    </Badge>
  );
}
