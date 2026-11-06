import type { Id, IsoDateTime } from "../common";

export type StorageLocation = {
  id: Id;
  name: string;
  description: string | null;
  color: string | null;
  icon: string | null;
  sort_order: number;
  is_archived: boolean;
  created_at: IsoDateTime;
  updated_at: IsoDateTime;
};

export type CreateStorageLocationInput = {
  name: string;
  description?: string | null;
  color?: string | null;
  icon?: string | null;
  sort_order?: number | null;
};

export type UpdateStorageLocationInput = {
  name: string;
  description?: string | null;
  color?: string | null;
  icon?: string | null;
  sort_order?: number | null;
  is_archived: boolean;
};
