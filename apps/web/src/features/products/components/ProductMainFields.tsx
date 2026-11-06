import type { Brand, DosageForm, ProductCategory } from "@doseark/types";

import { Field, FormSection, RequiredLabel, SelectInput, TextInput } from "@/core/ui";

import type { ProductFormValues } from "../utils/productFormModel";
import { dosageFormOptions, productCategoryOptions } from "../utils/productOptions";

type Props = {
  values: ProductFormValues;
  brands: Brand[];
  invalid: {
    nameMissing: boolean;
    unitMissing: boolean;
    servingSizeInvalid: boolean;
  };
  onChange: (patch: Partial<ProductFormValues>) => void;
};

/** Fields needed for every product, kept visible while optional settings stay collapsed. */
export function ProductMainFields({ values, brands, invalid, onChange }: Props) {
  return (
    <>
      <FormSection title="Product details" className="mt-product-form-primary">
        <div className="mt-form-grid mt-product-form-grid--main">
          <div className="mt-product-form-field--wide">
            <Field label={<RequiredLabel label="Name" invalid={invalid.nameMissing} />}>
              <TextInput
                value={values.name}
                placeholder="Product name"
                required
                autoFocus
                aria-invalid={invalid.nameMissing}
                onChange={(event) => onChange({ name: event.target.value })}
              />
            </Field>
          </div>
          <Field label="Brand">
            <SelectInput value={values.brandId} onChange={(event) => onChange({ brandId: event.target.value })}>
              <option value="">No brand</option>
              {brands.map((brand) => (
                <option key={brand.id} value={brand.id}>{brand.name}</option>
              ))}
            </SelectInput>
          </Field>
          <Field label={<RequiredLabel label="Category" />}>
            <SelectInput
              value={values.category}
              required
              onChange={(event) => onChange({ category: event.target.value as ProductCategory })}
            >
              {productCategoryOptions.map((option) => (
                <option key={option} value={option}>{option}</option>
              ))}
            </SelectInput>
          </Field>
          <Field label={<RequiredLabel label="Dosage form" />}>
            <SelectInput
              value={values.dosageForm}
              required
              onChange={(event) => onChange({ dosageForm: event.target.value as DosageForm })}
            >
              {dosageFormOptions.map((option) => (
                <option key={option} value={option}>{option}</option>
              ))}
            </SelectInput>
          </Field>
          <Field label={<RequiredLabel label="Status" />}>
            <SelectInput
              value={values.archived ? "archived" : "active"}
              required
              onChange={(event) => onChange({ archived: event.target.value === "archived" })}
            >
              <option value="active">active</option>
              <option value="archived">archived</option>
            </SelectInput>
          </Field>
        </div>
      </FormSection>

      <FormSection title="Serving" className="mt-product-form-primary">
        <div className="mt-form-grid mt-product-form-grid--serving">
          <Field label={<RequiredLabel label="Serving unit" invalid={invalid.unitMissing} />}>
            <TextInput
              value={values.servingUnit}
              required
              aria-invalid={invalid.unitMissing}
              onChange={(event) => onChange({ servingUnit: event.target.value })}
            />
          </Field>
          <Field label={<RequiredLabel label="Serving size" invalid={invalid.servingSizeInvalid} />}>
            <TextInput
              type="number"
              min={0.01}
              step={0.01}
              value={values.servingSize}
              required
              aria-invalid={invalid.servingSizeInvalid}
              onChange={(event) => onChange({ servingSize: event.target.value })}
            />
          </Field>
        </div>
      </FormSection>
    </>
  );
}
