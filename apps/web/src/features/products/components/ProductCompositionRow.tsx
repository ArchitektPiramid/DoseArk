import type { Nutrient } from "@doseark/types";

import { Badge, Button, Field, SelectInput, TextInput } from "@/core/ui";

import {
  getReferenceInfo,
  type ProductCompositionDraftRow,
} from "../utils/productCompositionDraft";

type Props = {
  row: ProductCompositionDraftRow;
  nutrients: Nutrient[];
  nutrientById: Map<string, Nutrient>;
  onUpdate: (localId: string, patch: Partial<ProductCompositionDraftRow>) => void;
  onRemove: (localId: string) => void;
};

/** Edits one nutrient in a compact card that fits the inline product editor. */
export function ProductCompositionRow({
  row,
  nutrients,
  nutrientById,
  onUpdate,
  onRemove,
}: Props) {
  const nutrient = nutrientById.get(row.nutrient_id);
  const reference = getReferenceInfo(row, nutrient);

  return (
    <div className="mt-product-composition-row">
      <div className="mt-product-composition-row__fields">
        <Field label="Nutrient">
          <SelectInput
            value={row.nutrient_id}
            onChange={(event) => {
              const selected = nutrientById.get(event.target.value);
              onUpdate(row.local_id, {
                nutrient_id: event.target.value,
                unit: selected?.default_unit ?? row.unit,
              });
            }}
          >
            <option value="">Select nutrient</option>
            {nutrients.map((item) => (
              <option key={item.id} value={item.id}>
                {item.display_name}
              </option>
            ))}
          </SelectInput>
        </Field>

        <Field label="Amount / serving">
          <TextInput
            type="number"
            min={0.000001}
            step={0.01}
            value={row.amount_per_serving}
            onChange={(event) =>
              onUpdate(row.local_id, { amount_per_serving: event.target.value })
            }
          />
        </Field>

        <Field label="Unit">
          <TextInput
            value={row.unit}
            onChange={(event) => onUpdate(row.local_id, { unit: event.target.value })}
          />
        </Field>

        <Field label="Label %DV">
          <TextInput
            type="number"
            min={0}
            step={0.1}
            value={row.label_daily_value_percent}
            onChange={(event) =>
              onUpdate(row.local_id, { label_daily_value_percent: event.target.value })
            }
          />
        </Field>
      </div>

      <div className="mt-product-composition-row__footer">
        <Badge tone={reference.tone}>{reference.label}</Badge>
        <Button variant="danger" onClick={() => onRemove(row.local_id)}>Remove</Button>
      </div>
    </div>
  );
}
