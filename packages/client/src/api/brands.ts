import type { Brand, CreateBrandInput, UpdateBrandInput } from "@doseark/types";
import type { HttpTransport } from "../transport";

export function listBrands(transport: HttpTransport): Promise<Brand[]> {
  return transport.get("/api/brands");
}

export function createBrand(transport: HttpTransport, input: CreateBrandInput): Promise<Brand> {
  return transport.post("/api/brands", input);
}

export function updateBrand(
  transport: HttpTransport,
  id: string,
  input: UpdateBrandInput,
): Promise<Brand> {
  return transport.patch(`/api/brands/${id}`, input);
}
