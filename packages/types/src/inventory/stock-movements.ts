import type { Id, IsoDateTime } from "../common";

export type StockMovementType =
  | "dose"
  | "manual_add"
  | "manual_remove"
  | "correction"
  | "discard"
  | "open_package"
  | "reconcile";

export type StockMovement = {
  id: Id;
  inventory_lot_id: Id;
  product_id: Id;
  dose_log_id: Id | null;
  occurred_at: IsoDateTime;
  movement_type: StockMovementType;
  quantity_delta: number;
  quantity_after: number;
  reason: string | null;
  notes: string | null;
  created_at: IsoDateTime;
};
