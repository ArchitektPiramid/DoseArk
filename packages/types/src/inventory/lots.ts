import type { Id, IsoDateTime } from "../common";

export type InventoryLotStatus =
  | "active"
  | "reserve"
  | "on_hold"
  | "empty"
  | "expired"
  | "discarded"
  | "archived";

export type InventoryLot = {
  id: Id;
  product_id: Id;
  lot_number: string | null;
  storage_location: string | null;
  storage_location_id: Id | null;
  units_initial: number;
  units_remaining: number;
  purchase_date: string | null;
  opened_date: string | null;
  expires_at: string | null;
  price_amount: number | null;
  price_currency: string | null;
  status: InventoryLotStatus;
  notes: string | null;
  created_at: IsoDateTime;
  updated_at: IsoDateTime;
};

export type CreateInventoryLotInput = {
  product_id: Id;
  lot_number?: string | null;
  storage_location?: string | null;
  storage_location_id?: Id | null;
  units_initial: number;
  units_remaining?: number | null;
  purchase_date?: string | null;
  opened_date?: string | null;
  expires_at?: string | null;
  price_amount?: number | null;
  price_currency?: string | null;
  status?: InventoryLotStatus | null;
  notes?: string | null;
};

export type UpdateInventoryLotInput = {
  product_id: Id;
  lot_number?: string | null;
  storage_location?: string | null;
  storage_location_id?: Id | null;
  units_initial: number;
  units_remaining: number;
  purchase_date?: string | null;
  opened_date?: string | null;
  expires_at?: string | null;
  price_amount?: number | null;
  price_currency?: string | null;
  status: InventoryLotStatus;
  notes?: string | null;
};
