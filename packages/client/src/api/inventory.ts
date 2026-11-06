import type {
  CreateInventoryLotInput,
  InventoryLot,
  StockMovement,
  UpdateInventoryLotInput,
} from "@doseark/types";
import type { HttpTransport } from "../transport";

export function listInventoryLots(transport: HttpTransport): Promise<InventoryLot[]> {
  return transport.get("/api/inventory-lots");
}

export function createInventoryLot(
  transport: HttpTransport,
  input: CreateInventoryLotInput,
): Promise<InventoryLot> {
  return transport.post("/api/inventory-lots", input);
}

export function updateInventoryLot(
  transport: HttpTransport,
  id: string,
  input: UpdateInventoryLotInput,
): Promise<InventoryLot> {
  return transport.patch(`/api/inventory-lots/${id}`, input);
}

export function listStockMovements(transport: HttpTransport): Promise<StockMovement[]> {
  return transport.get("/api/stock-movements");
}

export function listStockMovementsForLot(
  transport: HttpTransport,
  lotId: string,
): Promise<StockMovement[]> {
  return transport.get(`/api/inventory-lots/${lotId}/stock-movements`);
}
