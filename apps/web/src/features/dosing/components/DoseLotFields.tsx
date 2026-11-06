import type { InventoryLot, Product } from "@doseark/types";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";

import {
  FormField,
  Listbox,
  SelectInput,
  type FormFieldState,
  type ListboxOption,
} from "@/core/ui";
import { DoseLotOption } from "./DoseLotOption";
import { SelectedDoseLotInline } from "./SelectedDoseLotInline";

export function DoseLotFields(props: {
  products: Product[];
  productId: string;
  lotId: string;
  lots: InventoryLot[];
  selectedLot?: InventoryLot;
  productNameById: Map<string, string>;
  lotFieldState: FormFieldState;
  onProductChange: (productId: string) => void;
  onLotChange: (lotId: string) => void;
}) {
  const { t } = useTranslation("dosing");
  const lotOptions = useMemo<Array<ListboxOption<string>>>(
    () =>
      props.lots.map((lot) => {
        const productName = props.productNameById.get(lot.product_id) ?? "Unknown product";
        return {
          value: lot.id,
          label: productName,
          render: <DoseLotOption productName={productName} lot={lot} />,
        };
      }),
    [props.lots, props.productNameById],
  );

  return (
    <>
      <FormField label={t("form.productOptional")} state="neutral">
        <SelectInput value={props.productId} onChange={(event) => props.onProductChange(event.target.value)}>
          <option value="">{t("form.selectProduct")}</option>
          {props.products.map((product) => (
            <option key={product.id} value={product.id}>
              {props.productNameById.get(product.id) ?? product.name}
            </option>
          ))}
        </SelectInput>
      </FormField>

      <FormField
        label={t("form.lot")}
        state={props.lotFieldState}
        message={props.selectedLot ? undefined : t("validation.lotRequired")}
      >
        <Listbox
          value={props.lotId}
          placeholder={t("form.selectLot")}
          options={lotOptions}
          onChange={props.onLotChange}
        />
      </FormField>

      {props.selectedLot ? (
        <SelectedDoseLotInline
          productName={props.productNameById.get(props.selectedLot.product_id) ?? "Unknown product"}
          lot={props.selectedLot}
        />
      ) : null}
    </>
  );
}
