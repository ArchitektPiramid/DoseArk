import { ChevronDown } from "lucide-react";

import { Field, TextArea, TextInput } from "@/core/ui";

import type { ProductFormValues } from "../utils/productFormModel";
import { ProductVisualFields } from "./ProductVisualFields";

type Props = {
  values: ProductFormValues;
  weightInvalid: boolean;
  onChange: (patch: Partial<ProductFormValues>) => void;
};

/** Optional settings stay out of the way, but their saved values remain visible in the headings. */
export function ProductOptionalFields({ values, weightInvalid, onChange }: Props) {
  const packageFields = [values.unitWeightMg, values.barcode, values.description]
    .filter((value) => value.trim() !== "").length;
  const customAppearance = Boolean(values.uiColor || values.uiIcon || values.userImportance !== "normal");

  return (
    <div className="mt-product-form-extras">
      <details className="mt-product-form-accordion">
        <summary>
          <span className="mt-product-form-accordion__title">Package & notes</span>
          <span className={weightInvalid ? "mt-product-form-accordion__meta is-invalid" : "mt-product-form-accordion__meta"}>
            {weightInvalid ? "Check unit weight" : packageFields ? `${packageFields} filled` : "Optional"}
          </span>
          <ChevronDown size={15} aria-hidden="true" />
        </summary>
        <div className="mt-product-form-accordion__body">
          <div className="mt-form-grid mt-product-form-grid--serving">
            <Field label="Unit weight (mg)">
              <TextInput
                type="number"
                min={0}
                step={0.1}
                value={values.unitWeightMg}
                aria-invalid={weightInvalid}
                onChange={(event) => onChange({ unitWeightMg: event.target.value })}
              />
            </Field>
            <Field label="Barcode">
              <TextInput
                value={values.barcode}
                onChange={(event) => onChange({ barcode: event.target.value })}
              />
            </Field>
          </div>
          <Field label="Description">
            <TextArea
              className="mt-product-form-description"
              value={values.description}
              placeholder="Optional notes about this product"
              onChange={(event) => onChange({ description: event.target.value })}
            />
          </Field>
        </div>
      </details>

      <details className="mt-product-form-accordion">
        <summary>
          <span className="mt-product-form-accordion__title">Appearance</span>
          <span className="mt-product-form-accordion__meta">
            {customAppearance ? "Customized" : "Default"}
          </span>
          <ChevronDown size={15} aria-hidden="true" />
        </summary>
        <div className="mt-product-form-accordion__body">
          <ProductVisualFields
            uiColor={values.uiColor}
            uiIcon={values.uiIcon}
            userImportance={values.userImportance}
            onColorChange={(uiColor) => onChange({ uiColor })}
            onIconChange={(uiIcon) => onChange({ uiIcon })}
            onImportanceChange={(userImportance) => onChange({ userImportance })}
          />
        </div>
      </details>
    </div>
  );
}
