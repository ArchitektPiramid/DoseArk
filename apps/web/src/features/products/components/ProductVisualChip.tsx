import type { Product } from "@doseark/types";

import { Badge } from "@/core/ui";

import {
  getImportanceBadgeTone,
  getProductVisualColor,
  getProductVisualIcon,
} from "../utils/productVisual";
import { ProductIcon } from "./ProductIcon";

type Props = {
  product?: Product | null;
  showImportance?: boolean;
};

export function ProductVisualChip({ product, showImportance = false }: Props) {
  const icon = getProductVisualIcon(product);
  const color = getProductVisualColor(product);

  return (
    <div className="mt-product-visual-chip-wrap">
      <span
        className="mt-visual-chip"
        style={{ color }}
        title={product?.user_importance ?? "normal"}
      >
        <ProductIcon name={icon} />
      </span>

      {showImportance && product ? (
        <Badge tone={getImportanceBadgeTone(product.user_importance)}>
          {product.user_importance}
        </Badge>
      ) : null}
    </div>
  );
}
