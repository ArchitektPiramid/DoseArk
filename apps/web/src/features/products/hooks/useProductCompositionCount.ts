import { useEffect, useState } from "react";

import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("products");

export function useProductCompositionCount(productId?: string) {
  const [compositionCount, setCompositionCount] = useState<number | null>(null);

  useEffect(() => {
    let isCurrent = true;

    async function loadCompositionCount(nextProductId: string) {
      try {
        setCompositionCount(null);

        const rows = await apiClient.listProductNutrients(nextProductId);

        if (!isCurrent) return;
        setCompositionCount(rows.length);
      } catch (err) {
        if (!isCurrent) return;

        logger.error("Failed to load product composition count", err);
        setCompositionCount(null);
      }
    }

    if (!productId) {
      setCompositionCount(null);
      return () => {
        isCurrent = false;
      };
    }

    void loadCompositionCount(productId);

    return () => {
      isCurrent = false;
    };
  }, [productId]);

  return {
    compositionCount,
    setCompositionCount,
  };
}
