import type { Id, IsoDateTime } from "../common";

export type Brand = {
  id: Id;
  name: string;
  country: string | null;
  website: string | null;
  notes: string | null;
  created_at: IsoDateTime;
  updated_at: IsoDateTime;
};

export type CreateBrandInput = {
  name: string;
  country?: string | null;
  website?: string | null;
  notes?: string | null;
};

export type UpdateBrandInput = {
  name: string;
  country?: string | null;
  website?: string | null;
  notes?: string | null;
};
