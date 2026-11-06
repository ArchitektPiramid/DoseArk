import type { Nutrient } from "@doseark/types";
import { useEffect, useMemo, useState } from "react";

import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";
import {
  createEmptyCompositionDraftRow,
  toCompositionDraftRow,
  toSaveProductNutrientInput,
  type ProductCompositionDraftRow,
} from "../utils/productCompositionDraft";

const logger = createLogger("products");

type Status = "idle" | "loading" | "saving" | "error";

export function useProductComposition(productId?: string) {
  const [nutrients, setNutrients] = useState<Nutrient[]>([]);
  const [rows, setRows] = useState<ProductCompositionDraftRow[]>([]);
  const [status, setStatus] = useState<Status>("idle");
  const [error, setError] = useState<string | null>(null);

  const nutrientById = useMemo(() => {
    return new Map(nutrients.map((nutrient) => [nutrient.id, nutrient]));
  }, [nutrients]);

  async function loadComposition(nextProductId: string) {
    try {
      setStatus("loading");
      setError(null);

      logger.trace("Loading product composition", { productId: nextProductId });

      const [nutrientRows, compositionRows] = await Promise.all([
        apiClient.listNutrients(),
        apiClient.listProductNutrients(nextProductId),
      ]);

      setNutrients(nutrientRows);
      setRows(compositionRows.map(toCompositionDraftRow));

      logger.debug("Product composition loaded", {
        productId: nextProductId,
        nutrients: nutrientRows.length,
        composition: compositionRows.length,
      });

      setStatus("idle");
    } catch (err) {
      logger.error("Failed to load product composition", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatus("error");
    }
  }

  async function saveComposition() {
    if (!productId) return;

    try {
      setStatus("saving");
      setError(null);

      const input = rows
        .map(toSaveProductNutrientInput)
        .filter((row) => row !== null);

      logger.info("Saving product composition", {
        productId,
        items: input.length,
      });

      const saved = await apiClient.replaceProductNutrients(productId, input);
      setRows(saved.map(toCompositionDraftRow));

      logger.success("Product composition saved", {
        productId,
        items: saved.length,
      });

      setStatus("idle");
    } catch (err) {
      logger.error("Failed to save product composition", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatus("error");
    }
  }

  function addRow() {
    setRows((current) => [...current, createEmptyCompositionDraftRow()]);
  }

  function removeRow(localId: string) {
    setRows((current) => current.filter((row) => row.local_id !== localId));
  }

  function updateRow(localId: string, patch: Partial<ProductCompositionDraftRow>) {
    setRows((current) =>
      current.map((row) => (row.local_id === localId ? { ...row, ...patch } : row)),
    );
  }

  useEffect(() => {
    if (!productId) {
      setRows([]);
      return;
    }

    void loadComposition(productId);
  }, [productId]);

  return {
    nutrients,
    nutrientById,
    rows,
    status,
    error,
    addRow,
    removeRow,
    updateRow,
    saveComposition,
  };
}
