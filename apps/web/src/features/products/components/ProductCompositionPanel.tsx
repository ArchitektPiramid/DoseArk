import { useEffect } from "react";

import { Button, Panel } from "@/core/ui";

import { ProductCompositionRow } from "./ProductCompositionRow";
import { useProductComposition } from "../hooks/useProductComposition";

type Props = {
  productId?: string;
  onRowsCountChange?: (count: number) => void;
};

/** Manages the selected product's nutrient composition without a wide table. */
export function ProductCompositionPanel({ productId, onRowsCountChange }: Props) {
  const data = useProductComposition(productId);

  useEffect(() => {
    onRowsCountChange?.(data.rows.length);
  }, [data.rows.length, onRowsCountChange]);

  if (!productId) {
    return (
      <Panel title="Composition">
        <div className="mt-empty-state">Select a product to edit composition.</div>
      </Panel>
    );
  }

  return (
    <Panel
      className="mt-product-composition"
      title="Composition"
      action={
        <>
          <Button disabled={data.status === "saving" || data.status === "loading"} onClick={data.addRow}>Add nutrient</Button>
          <Button
            variant="primary"
            disabled={data.status === "saving" || data.status === "loading"}
            onClick={() => void data.saveComposition()}
          >
            {data.status === "saving" ? "Saving..." : "Save"}
          </Button>
        </>
      }
    >
      {data.error ? <pre className="mt-error-block">{data.error}</pre> : null}

      {data.rows.length === 0 ? (
        <div className="mt-empty-state">No composition rows yet. Add a nutrient to get started.</div>
      ) : (
        <div className="mt-product-composition-list">
          {data.rows.map((row) => (
            <ProductCompositionRow
              key={row.local_id}
              row={row}
              nutrients={data.nutrients}
              nutrientById={data.nutrientById}
              onUpdate={data.updateRow}
              onRemove={data.removeRow}
            />
          ))}
        </div>
      )}
    </Panel>
  );
}
