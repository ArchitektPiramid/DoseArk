import type {
  CreateProductInput,
  Product,
  UpdateProductInput,
} from "@doseark/types";
import type { HttpTransport } from "../transport";

export function listProducts(transport: HttpTransport): Promise<Product[]> {
  return transport.get("/api/products");
}

export function createProduct(
  transport: HttpTransport,
  input: CreateProductInput,
): Promise<Product> {
  return transport.post("/api/products", input);
}

export function updateProduct(
  transport: HttpTransport,
  id: string,
  input: UpdateProductInput,
): Promise<Product> {
  return transport.patch(`/api/products/${id}`, input);
}
