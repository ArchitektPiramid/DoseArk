import type { Product } from "@doseark/types";
import { useTranslation } from "react-i18next";

import { SelectInput } from "@/core/ui";
import { cx } from "@/core/ui/utils";

type Props = {
  products: Product[];
  productNameById: Map<string, string>;
  historyProductId: string;
  pageSize: number;
  pageSizeOptions: readonly number[];
  compact?: boolean;
  onHistoryProductChange: (productId: string) => void;
  onPageSizeChange: (pageSize: number) => void;
};

export function DoseHistoryToolbar({
  products,
  productNameById,
  historyProductId,
  pageSize,
  pageSizeOptions,
  compact,
  onHistoryProductChange,
  onPageSizeChange,
}: Props) {
  const { t } = useTranslation("dosing");

  return (
    <div className={cx("mt-dose-history-toolbar", compact && "mt-bulk-dose-history-toolbar")}>
      <label className="mt-dose-history-control">
        <span className="mt-label">{t("table.filterProduct")}</span>
        <SelectInput
          value={historyProductId}
          onChange={(event) => onHistoryProductChange(event.target.value)}
        >
          <option value="">{t("table.allProducts")}</option>
          {products.map((product) => (
            <option key={product.id} value={product.id}>
              {productNameById.get(product.id) ?? product.name}
            </option>
          ))}
        </SelectInput>
      </label>

      <label className="mt-dose-history-control mt-dose-history-control--compact">
        <span className="mt-label">{t("table.perPage")}</span>
        <SelectInput
          value={String(pageSize)}
          onChange={(event) => onPageSizeChange(Number(event.target.value))}
        >
          {pageSizeOptions.map((option) => (
            <option key={option} value={option}>
              {option}
            </option>
          ))}
        </SelectInput>
      </label>
    </div>
  );
}
