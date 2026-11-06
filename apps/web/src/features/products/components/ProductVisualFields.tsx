import type { ProductUserImportance } from "@doseark/types";

import { Field, SelectInput } from "@/core/ui";
import { visualColorOptions } from "@/core/visualTags";

import { productImportanceOptions } from "../utils/productOptions";
import { ProductIcon } from "./ProductIcon";
import { ProductIconSelect } from "./ProductIconSelect";

type Props = {
  uiColor: string;
  uiIcon: string;
  userImportance: ProductUserImportance;
  onColorChange: (value: string) => void;
  onIconChange: (value: string) => void;
  onImportanceChange: (value: ProductUserImportance) => void;
};

/** Appearance uses the same selects as the main product form. */
export function ProductVisualFields({
  uiColor,
  uiIcon,
  userImportance,
  onColorChange,
  onIconChange,
  onImportanceChange,
}: Props) {
  const isCustomColor = uiColor !== "" && !visualColorOptions.includes(uiColor);

  return (
    <section className="mt-product-visual-fields">
      <div className="mt-form-grid mt-form-grid--3">
        <Field label="Icon">
          <ProductIconSelect value={uiIcon} placeholder="Default icon" onChange={onIconChange} />
        </Field>

        <Field label="Color">
          <SelectInput value={uiColor} onChange={(event) => onColorChange(event.target.value)}>
            <option value="">Default color</option>
            {isCustomColor ? <option value={uiColor}>{`Custom: ${uiColor}`}</option> : null}
            {visualColorOptions.map((color) => (
              <option key={color} value={color}>
                {color}
              </option>
            ))}
          </SelectInput>
        </Field>

        <Field label="Importance">
          <SelectInput
            value={userImportance}
            onChange={(event) =>
              onImportanceChange(event.target.value as ProductUserImportance)
            }
          >
            {productImportanceOptions.map((option) => (
              <option key={option} value={option}>
                {option}
              </option>
            ))}
          </SelectInput>
        </Field>
      </div>

      <div className="mt-product-visual-preview">
        <div className="mt-product-visual-preview__icon" style={{ color: uiColor || undefined }}>
          <ProductIcon name={uiIcon || "pill"} />
        </div>

        <div className="mt-min-w-0">
          <div className="mt-product-visual-preview__title">Product visual preview</div>
          <div className="mt-product-visual-preview__meta">
            importance: {userImportance}
          </div>
        </div>
      </div>
    </section>
  );
}
