import type { CreateInventoryLotInput, UpdateInventoryLotInput } from "@doseark/types";
import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";
import type { InventoryStatus } from "./useInventoryLotsLoader";

const logger = createLogger("inventory");

type Params = {
  setStatus: (status: InventoryStatus) => void;
  setError: (error: string | null) => void;
  reload: () => Promise<unknown>;
  setSelectedLotId: (id: string | null) => void;
};

export function useInventoryLotMutations(params: Params) {
  async function handleCreateInventoryLot(input: CreateInventoryLotInput) {
    try {
      params.setStatus("saving");
      params.setError(null);

      logger.info("Creating inventory lot", {
        product_id: input.product_id,
        units_initial: input.units_initial,
        storage_location_id: input.storage_location_id,
      });

      const lot = await apiClient.createInventoryLot(input);
      params.setSelectedLotId(lot.id);

      logger.success("Inventory lot created", {
        id: lot.id,
        product_id: lot.product_id,
      });

      await params.reload();
      return true;
    } catch (err) {
      logger.error("Failed to create inventory lot", err);
      params.setError(err instanceof Error ? err.message : "Unknown error");
      params.setStatus("error");
      return false;
    }
  }

  async function handleUpdateInventoryLot(id: string, input: UpdateInventoryLotInput) {
    try {
      params.setStatus("saving");
      params.setError(null);

      logger.info("Updating inventory lot", {
        id,
        product_id: input.product_id,
        storage_location_id: input.storage_location_id,
      });

      const lot = await apiClient.updateInventoryLot(id, input);
      params.setSelectedLotId(lot.id);

      logger.success("Inventory lot updated", {
        id: lot.id,
        product_id: lot.product_id,
      });

      await params.reload();
      return true;
    } catch (err) {
      logger.error("Failed to update inventory lot", err);
      params.setError(err instanceof Error ? err.message : "Unknown error");
      params.setStatus("error");
      return false;
    }
  }

  return {
    createInventoryLot: handleCreateInventoryLot,
    updateInventoryLot: handleUpdateInventoryLot,
  };
}
