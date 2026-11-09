import { useTranslation } from "react-i18next";

import { Button, Field, SelectInput } from "@/core/ui";
import { buildStatisticsNutrientKey } from "../utils/statisticsSelection";
import type { StatisticsToolbarProps } from "./StatisticsToolbar";

type Props = Pick<
  StatisticsToolbarProps,
  | "source"
  | "products"
  | "nutrients"
  | "selectedProductId"
  | "selectedNutrientKey"
  | "loading"
  | "onSourceChange"
  | "onProductChange"
  | "onNutrientChange"
>;

/** Renders the product/nutrient selector without touching date range state. */
export function StatisticsSubjectControls(props: Props) {
  const { t } = useTranslation("statistics");

  return (
    <div className="mt-statistics-toolbar__subject">
      <div className="mt-statistics-segmented" role="group" aria-label={t("subject.source")}>
        {(["product", "nutrient"] as const).map((source) => (
          <Button
            key={source}
            type="button"
            variant={props.source === source ? "primary" : "quiet"}
            aria-pressed={props.source === source}
            disabled={props.loading}
            onClick={() => props.onSourceChange(source)}
          >
            {t(`subject.${source}`)}
          </Button>
        ))}
      </div>
      <div className="mt-statistics-toolbar__subject-select">
        {props.source === "product" ? (
          <Field label={t("layout.selectedProduct")}>
            <SelectInput
              value={props.selectedProductId ?? ""}
              disabled={props.loading || props.products.length === 0}
              onChange={(event) => props.onProductChange(event.target.value)}
            >
              {props.products.length === 0 ? (
                <option value="">{t("subject.noProducts")}</option>
              ) : null}
              {props.products.map((product) => (
                <option key={product.id} value={product.id}>
                  {product.name} · {product.serving_unit}
                </option>
              ))}
            </SelectInput>
          </Field>
        ) : (
          <Field label={t("layout.selectedNutrient")}>
            <SelectInput
              value={props.selectedNutrientKey ?? ""}
              disabled={props.loading || props.nutrients.length === 0}
              onChange={(event) => props.onNutrientChange(event.target.value)}
            >
              {props.nutrients.length === 0 ? (
                <option value="">{t("subject.noNutrients")}</option>
              ) : null}
              {props.nutrients.map((nutrient) => {
                const key = buildStatisticsNutrientKey({
                  nutrientId: nutrient.nutrient_id,
                  unit: nutrient.unit,
                });
                return (
                  <option key={key} value={key}>
                    {nutrient.nutrient_display_name} · {nutrient.unit}
                  </option>
                );
              })}
            </SelectInput>
          </Field>
        )}
      </div>
    </div>
  );
}
