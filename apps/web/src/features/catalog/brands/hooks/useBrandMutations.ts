import type { CreateBrandInput, UpdateBrandInput } from "@doseark/types";
import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";
import type { BrandsStatus } from "./useBrandsLoader";

const logger = createLogger("brands");

type Params = {
  setStatus: (status: BrandsStatus) => void;
  setError: (error: string | null) => void;
  reload: () => Promise<unknown>;
  setSelectedBrandId: (id: string | null) => void;
};

export function useBrandMutations(params: Params) {
  async function handleCreateBrand(input: CreateBrandInput) {
    try {
      params.setStatus("saving");
      params.setError(null);

      logger.info("Creating brand", { name: input.name });

      const brand = await apiClient.createBrand(input);
      params.setSelectedBrandId(brand.id);

      logger.success("Brand created", { id: brand.id, name: brand.name });

      await params.reload();
    } catch (err) {
      logger.error("Failed to create brand", err);
      params.setError(err instanceof Error ? err.message : "Unknown error");
      params.setStatus("error");
    }
  }

  async function handleUpdateBrand(id: string, input: UpdateBrandInput) {
    try {
      params.setStatus("saving");
      params.setError(null);

      logger.info("Updating brand", { id, name: input.name });

      const brand = await apiClient.updateBrand(id, input);
      params.setSelectedBrandId(brand.id);

      logger.success("Brand updated", { id: brand.id, name: brand.name });

      await params.reload();
    } catch (err) {
      logger.error("Failed to update brand", err);
      params.setError(err instanceof Error ? err.message : "Unknown error");
      params.setStatus("error");
    }
  }

  return {
    createBrand: handleCreateBrand,
    updateBrand: handleUpdateBrand,
  };
}
