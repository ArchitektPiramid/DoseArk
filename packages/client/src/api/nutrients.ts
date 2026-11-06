import type {
  CreateNutrientInput,
  Nutrient,
  ProductNutrient,
  SaveProductNutrientInput,
  UpdateNutrientInput,
} from "@doseark/types";
import type { HttpTransport } from "../transport";

export function listNutrients(transport: HttpTransport): Promise<Nutrient[]> {
  return transport.get("/api/nutrients");
}

export function createNutrient(
  transport: HttpTransport,
  input: CreateNutrientInput,
): Promise<Nutrient> {
  return transport.post("/api/nutrients", input);
}

export function updateNutrient(
  transport: HttpTransport,
  id: string,
  input: UpdateNutrientInput,
): Promise<Nutrient> {
  return transport.patch(`/api/nutrients/${id}`, input);
}

export function listProductNutrients(
  transport: HttpTransport,
  productId: string,
): Promise<ProductNutrient[]> {
  return transport.get(`/api/products/${productId}/nutrients`);
}

export function replaceProductNutrients(
  transport: HttpTransport,
  productId: string,
  items: SaveProductNutrientInput[],
): Promise<ProductNutrient[]> {
  return transport.put(`/api/products/${productId}/nutrients`, items);
}
