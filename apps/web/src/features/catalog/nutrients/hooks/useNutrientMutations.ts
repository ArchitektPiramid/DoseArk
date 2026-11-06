import type { CreateNutrientInput, UpdateNutrientInput } from "@doseark/types";
import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";
import type { NutrientsStatus } from "./useNutrientsLoader";

const logger = createLogger("nutrients");

type Params = {
  setStatus: (status: NutrientsStatus) => void;
  setError: (error: string | null) => void;
  reload: () => Promise<unknown>;
  setSelectedNutrientId: (id: string | null) => void;
};

export function useNutrientMutations(params: Params) {
  async function handleCreateNutrient(input: CreateNutrientInput) {
    try {
      params.setStatus("saving");
      params.setError(null);

      logger.info("Creating nutrient", {
        canonical_name: input.canonical_name,
        display_name: input.display_name,
      });

      const nutrient = await apiClient.createNutrient(input);
      params.setSelectedNutrientId(nutrient.id);

      logger.success("Nutrient created", {
        id: nutrient.id,
        display_name: nutrient.display_name,
      });

      await params.reload();
    } catch (err) {
      logger.error("Failed to create nutrient", err);
      params.setError(err instanceof Error ? err.message : "Unknown error");
      params.setStatus("error");
    }
  }

  async function handleUpdateNutrient(id: string, input: UpdateNutrientInput) {
    try {
      params.setStatus("saving");
      params.setError(null);

      logger.info("Updating nutrient", {
        id,
        canonical_name: input.canonical_name,
        display_name: input.display_name,
      });

      const nutrient = await apiClient.updateNutrient(id, input);
      params.setSelectedNutrientId(nutrient.id);

      logger.success("Nutrient updated", {
        id: nutrient.id,
        display_name: nutrient.display_name,
      });

      await params.reload();
    } catch (err) {
      logger.error("Failed to update nutrient", err);
      params.setError(err instanceof Error ? err.message : "Unknown error");
      params.setStatus("error");
    }
  }

  return {
    createNutrient: handleCreateNutrient,
    updateNutrient: handleUpdateNutrient,
  };
}
