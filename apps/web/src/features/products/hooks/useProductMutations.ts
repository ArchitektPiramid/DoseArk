import type { CreateBrandInput, CreateProductInput, UpdateProductInput } from "@doseark/types";

import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";
import type { LoadStatus } from "./useProductsLoader";

const logger = createLogger("products");

type Params = {
  setStatus: (status: LoadStatus) => void;
  setError: (error: string | null) => void;
  reload: () => Promise<unknown>;
  setSelectedProductId: (productId: string | null) => void;
};

export function useProductMutations(params: Params) {
  async function handleCreateBrand(input: CreateBrandInput) {
    try {
      params.setStatus("saving");
      params.setError(null);

      logger.info("Creating brand", { name: input.name });

      await apiClient.createBrand(input);
      await params.reload();
    } catch (err) {
      logger.error("Failed to create brand", err);
      params.setError(err instanceof Error ? err.message : "Unknown error");
      params.setStatus("error");
    }
  }

  async function handleCreateProduct(input: CreateProductInput): Promise<boolean> {
    try {
      params.setStatus("saving");
      params.setError(null);

      logger.info("Creating product", {
        name: input.name,
        brand_id: input.brand_id,
        ui_icon: input.ui_icon,
        ui_color: input.ui_color,
        user_importance: input.user_importance,
      });

      const product = await apiClient.createProduct(input);
      params.setSelectedProductId(product.id);

      logger.success("Product created", {
        id: product.id,
        name: product.name,
      });

      await params.reload();
      return true;
    } catch (err) {
      logger.error("Failed to create product", err);
      params.setError(err instanceof Error ? err.message : "Unknown error");
      params.setStatus("error");
      return false;
    }
  }

  async function handleUpdateProduct(
    id: string,
    input: UpdateProductInput,
  ): Promise<boolean> {
    try {
      params.setStatus("saving");
      params.setError(null);

      logger.info("Updating product", {
        id,
        name: input.name,
        brand_id: input.brand_id,
        ui_icon: input.ui_icon,
        ui_color: input.ui_color,
        user_importance: input.user_importance,
      });

      const product = await apiClient.updateProduct(id, input);
      params.setSelectedProductId(product.id);

      logger.success("Product updated", {
        id: product.id,
        name: product.name,
      });

      await params.reload();
      return true;
    } catch (err) {
      logger.error("Failed to update product", err);
      params.setError(err instanceof Error ? err.message : "Unknown error");
      params.setStatus("error");
      return false;
    }
  }

  return {
    createBrand: handleCreateBrand,
    createProduct: handleCreateProduct,
    updateProduct: handleUpdateProduct,
  };
}
