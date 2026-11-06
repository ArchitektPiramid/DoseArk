import type { CreateDoseLogInput, UpdateDoseLogInput } from "@doseark/types";
import { useCallback, useState } from "react";
import { useTranslation } from "react-i18next";

import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";

import type { DosingToastMessage } from "../components/DosingToast";

const logger = createLogger("dosing");

type UseDoseLogMutationsOptions = {
  reload: () => Promise<void>;
  cancelEditing: () => void;
  clearDoseLogState: (doseLogId: string) => void;
  showToast: (variant: DosingToastMessage["variant"], message: string) => void;
};

export function useDoseLogMutations({
  reload,
  cancelEditing,
  clearDoseLogState,
  showToast,
}: UseDoseLogMutationsOptions) {
  const { t } = useTranslation("dosing");
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const clearError = useCallback(() => {
    setError(null);
  }, []);

  const saveDose = useCallback(
    async (input: CreateDoseLogInput) => {
      try {
        setIsSaving(true);
        setError(null);

        logger.info("Creating dose log", {
          product_id: input.product_id,
          inventory_lot_id: input.inventory_lot_id,
          units_taken: input.units_taken,
        });

        await apiClient.createDoseLog(input);
        logger.success("Dose log created");

        await reload();
        showToast("success", t("toast.created"));
        return true;
      } catch (err) {
        logger.error("Failed to create dose log", err);
        const message = err instanceof Error ? err.message : "Unknown error";
        setError(message);
        showToast("error", `${t("toast.saveError")} ${message}`);
        return false;
      } finally {
        setIsSaving(false);
      }
    },
    [reload, showToast, t],
  );

  const saveDoseBatch = useCallback(
    async (
      inputs: CreateDoseLogInput[],
      onProgress?: (index: number, status: "saving" | "success" | "error") => void,
    ) => {
      if (inputs.length === 0) return false;

      let savedCount = 0;
      try {
        setIsSaving(true);
        setError(null);

        logger.warn("Creating debug bulk dose batch", { count: inputs.length });

        for (let index = 0; index < inputs.length; index += 1) {
          const input = inputs[index];
          if (!input) continue;

          onProgress?.(index, "saving");
          try {
            await apiClient.createDoseLog(input);
            savedCount += 1;
            onProgress?.(index, "success");
          } catch (err) {
            onProgress?.(index, "error");
            throw err;
          }
        }

        logger.success("Debug bulk dose batch created", { count: savedCount });
        await reload();
        showToast("success", t("toast.batchCreated", { count: savedCount }));
        return true;
      } catch (err) {
        logger.error("Debug bulk dose batch failed", {
          error: err,
          savedCount,
          totalCount: inputs.length,
        });

        const message = err instanceof Error ? err.message : "Unknown error";
        setError(message);

        try {
          await reload();
        } catch (reloadError) {
          logger.error("Could not reload dosing data after partial bulk save", reloadError);
        }

        showToast(
          "error",
          t("toast.batchSaveError", {
            saved: savedCount,
            total: inputs.length,
            message,
          }),
        );
        return false;
      } finally {
        setIsSaving(false);
      }
    },
    [reload, showToast, t],
  );

  const saveDoseUpdate = useCallback(
    async (id: string, input: UpdateDoseLogInput) => {
      try {
        setIsSaving(true);
        setError(null);

        logger.info("Updating dose log", {
          id,
          product_id: input.product_id,
          inventory_lot_id: input.inventory_lot_id,
          units_taken: input.units_taken,
        });

        await apiClient.updateDoseLog(id, input);
        cancelEditing();

        logger.success("Dose log updated", { id });

        await reload();
        showToast("success", t("toast.updated"));
        return true;
      } catch (err) {
        logger.error("Failed to update dose log", err);
        const message = err instanceof Error ? err.message : "Unknown error";
        setError(message);
        showToast("error", `${t("toast.updateError")} ${message}`);
        return false;
      } finally {
        setIsSaving(false);
      }
    },
    [cancelEditing, reload, showToast, t],
  );

  const cancelDose = useCallback(
    async (id: string) => {
      try {
        setIsSaving(true);
        setError(null);

        logger.warn("Cancelling dose log", { id });
        await apiClient.cancelDoseLog(id);

        clearDoseLogState(id);
        logger.success("Dose log cancelled", { id });

        await reload();
      } catch (err) {
        logger.error("Failed to cancel dose log", err);
        setError(err instanceof Error ? err.message : "Unknown error");
      } finally {
        setIsSaving(false);
      }
    },
    [clearDoseLogState, reload],
  );

  return {
    isSaving,
    error,
    clearError,
    saveDose,
    saveDoseBatch,
    saveDoseUpdate,
    cancelDose,
  };
}
