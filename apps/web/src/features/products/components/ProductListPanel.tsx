import type { Product } from "@doseark/types";
import { useMemo, useState } from "react";

import { Badge, Panel, Table, TableEmptyRow, TableFilterToggle, TableRowCount } from "@/core/ui";

import { ProductVisualChip } from "./ProductVisualChip";
import { getImportanceBadgeTone } from "../utils/productVisual";

type ProductStatusFilter = "active" | "archived";

const PRODUCT_STATUS_FILTERS: Array<{
  value: ProductStatusFilter;
  label: string;
}> = [
  { value: "active", label: "active" },
  { value: "archived", label: "archived" },
];

type Props = {
  products: Product[];
  selectedProductId?: string | null;
  loading?: boolean;
  brandNameById: Map<string, string>;
  onSelect: (productId: string) => void;
};

export function ProductListPanel({
  products,
  selectedProductId,
  loading,
  brandNameById,
  onSelect,
}: Props) {
  const [visibleStatuses, setVisibleStatuses] = useState<Set<ProductStatusFilter>>(
    () => new Set(PRODUCT_STATUS_FILTERS.map((status) => status.value)),
  );

  const statusCounts = useMemo(() => {
    const counts = new Map<ProductStatusFilter, number>([
      ["active", 0],
      ["archived", 0],
    ]);

    for (const product of products) {
      const status: ProductStatusFilter = product.archived ? "archived" : "active";
      counts.set(status, (counts.get(status) ?? 0) + 1);
    }

    return counts;
  }, [products]);

  const visibleProducts = useMemo(() => {
    return products.filter((product) =>
      visibleStatuses.has(product.archived ? "archived" : "active"),
    );
  }, [products, visibleStatuses]);

  function toggleStatus(status: ProductStatusFilter) {
    setVisibleStatuses((current) => {
      const next = new Set(current);

      if (next.has(status)) {
        next.delete(status);
      } else {
        next.add(status);
      }

      return next;
    });
  }

  return (
    <Panel
      className="mt-product-list-panel"
      title={
        <div className="mt-product-list-title">
          <span>Product list</span>
          <div className="mt-table-filters" aria-label="Product status filters">
            {PRODUCT_STATUS_FILTERS.map((status) => (
              <TableFilterToggle
                key={status.value}
                checked={visibleStatuses.has(status.value)}
                label={status.label}
                count={statusCounts.get(status.value) ?? 0}
                onChange={() => toggleStatus(status.value)}
              />
            ))}
          </div>
        </div>
      }
      action={
        <TableRowCount count={visibleProducts.length} total={products.length} loading={loading} />
      }
    >
      <Table columns={["Tag", "Name", "Brand", "Category", "Form", "Status"]}>
        {visibleProducts.length === 0 ? (
          <TableEmptyRow colSpan={6}>No products match selected filters.</TableEmptyRow>
        ) : (
          visibleProducts.map((product) => {
            const brandName = product.brand_id
              ? (brandNameById.get(product.brand_id) ?? "Unknown brand")
              : "-";

            return (
              <tr
                key={product.id}
                className={
                  product.id === selectedProductId
                    ? "mt-table-row--interactive is-selected"
                    : "mt-table-row--interactive"
                }
                title="Click to edit product"
                onClick={() => onSelect(product.id)}
              >
                <td>
                  <ProductVisualChip product={product} />
                </td>
                <td className="mt-product-list-name" title={product.name}>
                  <div className="mt-table-cell-strong">{product.name}</div>
                </td>
                <td title={brandName}>{brandName}</td>
                <td>{product.category}</td>
                <td>{product.dosage_form}</td>
                <td>
                  <div className="mt-product-status-stack">
                    <Badge tone={product.archived ? "warn" : "ok"}>
                      {product.archived ? "archived" : "active"}
                    </Badge>
                    <Badge tone={getImportanceBadgeTone(product.user_importance)}>
                      {product.user_importance}
                    </Badge>
                  </div>
                </td>
              </tr>
            );
          })
        )}
      </Table>
    </Panel>
  );
}
