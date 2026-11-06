import type { Product } from "@doseark/types";

import { formatNumber } from "@/core/format";
import { Badge, Panel } from "@/core/ui";

import { ProductVisualChip } from "./ProductVisualChip";
import { getImportanceBadgeTone } from "../utils/productVisual";

type Props = {
  product?: Product;
  brandName?: string;
};

export function ProductDetailsPanel({ product, brandName }: Props) {
  if (!product) {
    return (
      <Panel title="Product details">
        <div className="mt-empty-state">Select or create a product.</div>
      </Panel>
    );
  }

  return (
    <Panel title="Selected product">
      <div className="mt-detail-list">
        <div className="mt-product-details-header">
          <ProductVisualChip product={product} />

          <div className="mt-min-w-0">
            <div className="mt-detail-title">
              {product.name}
            </div>
            <div className="mt-detail-meta">{product.id}</div>
          </div>
        </div>

        <div className="mt-separator" />

        <Detail label="Brand" value={brandName ?? "-"} />
        <Detail label="Category" value={product.category} />
        <Detail label="Dosage form" value={product.dosage_form} />
        <Detail
          label="Serving"
          value={`${formatNumber(product.serving_size_units, { maxDecimals: 3 })} ${
            product.serving_unit
          }`}
        />
        <Detail
          label="Unit weight"
          value={
            product.unit_weight_mg === null
              ? "-"
              : `${formatNumber(product.unit_weight_mg, { maxDecimals: 2 })} mg`
          }
        />
        <Detail label="Barcode" value={product.barcode ?? "-"} />
        <Detail label="Icon" value={product.ui_icon ?? "default"} />
        <Detail label="Color" value={product.ui_color ?? "default"} />

        <div className="mt-detail-row">
          <div className="mt-label">Importance</div>
          <div>
            <Badge tone={getImportanceBadgeTone(product.user_importance)}>
              {product.user_importance}
            </Badge>
          </div>
        </div>

        <Detail label="Archived" value={product.archived ? "yes" : "no"} />

        {product.description ? (
          <>
            <div className="mt-separator" />
            <div>
              <div className="mt-label">Description</div>
              <div className="mt-detail-description">{product.description}</div>
            </div>
          </>
        ) : null}
      </div>
    </Panel>
  );
}

function Detail(props: { label: string; value: string }) {
  return (
    <div className="mt-detail-row">
      <div className="mt-label">{props.label}</div>
      <div className="mt-detail-value">{props.value}</div>
    </div>
  );
}
