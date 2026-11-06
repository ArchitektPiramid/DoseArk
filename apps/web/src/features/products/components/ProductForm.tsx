import type { Brand, CreateProductInput, Product, UpdateProductInput } from "@doseark/types";
import { useEffect, useState } from "react";

import { Button, FormActions } from "@/core/ui";

import { ProductMainFields } from "./ProductMainFields";
import { ProductOptionalFields } from "./ProductOptionalFields";
import {
  buildProductInput,
  productFormValues,
  validateProductForm,
  type ProductFormValues,
} from "../utils/productFormModel";

export type ProductFormProps = {
  mode: "create" | "edit";
  product?: Product;
  brands: Brand[];
  disabled?: boolean;
  onCancel: () => void;
  onSaved: () => void;
  onCreate: (input: CreateProductInput) => boolean | Promise<boolean>;
  onUpdate: (id: string, input: UpdateProductInput) => boolean | Promise<boolean>;
};

/** Owns the product draft; field groups only render and update it. */
export function ProductForm({
  mode, product, brands, disabled, onCancel, onSaved, onCreate, onUpdate,
}: ProductFormProps) {
  const [values, setValues] = useState<ProductFormValues>(() => productFormValues(product));

  useEffect(() => {
    setValues(productFormValues(mode === "edit" ? product : undefined));
  }, [mode, product]);

  const validation = validateProductForm(values);

  function patchValues(patch: Partial<ProductFormValues>) {
    setValues((current) => ({ ...current, ...patch }));
  }

  /** Writes the draft through the existing create/update callbacks. */
  async function submit() {
    if (disabled) return;
    const input = buildProductInput(values);
    if (!input) return;

    const saved = mode === "edit" && product
      ? await onUpdate(product.id, input)
      : await onCreate(input);
    if (saved) onSaved();
  }

  return (
    <form
      className="mt-form mt-product-form"
      onSubmit={(event) => {
        event.preventDefault();
        void submit();
      }}
    >
      <ProductMainFields
        values={values}
        brands={brands}
        invalid={validation}
        onChange={patchValues}
      />
      <ProductOptionalFields
        values={values}
        weightInvalid={validation.weightInvalid}
        onChange={patchValues}
      />
      <FormActions>
        <Button type="button" onClick={onCancel} disabled={disabled}>
          {mode === "edit" ? "Cancel edit" : "Clear"}
        </Button>
        <Button type="submit" variant="primary" disabled={!validation.valid || disabled}>
          {disabled ? "Saving..." : mode === "edit" ? "Save product" : "Create product"}
        </Button>
      </FormActions>
    </form>
  );
}
