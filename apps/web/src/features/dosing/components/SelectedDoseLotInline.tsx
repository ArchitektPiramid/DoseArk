import type { InventoryLot } from "@doseark/types";
import { CalendarDays, Package, Pill } from "lucide-react";
import { useTranslation } from "react-i18next";

import { formatLotStock } from "../utils/dosingFormat";

type Props = {
  productName: string;
  lot: InventoryLot;
};

export function SelectedDoseLotInline({ productName, lot }: Props) {
  const { t } = useTranslation("dosing");

  return (
    <div className="mt-dose-lot-inline">
      <InlineInfo icon={<Package size={13} />} label={t("lotInfo.product")} value={productName} />
      <InlineInfo
        icon={<Pill size={13} />}
        label={t("lotInfo.stock")}
        value={formatLotStock(lot.units_remaining, lot.units_initial)}
      />
      <InlineInfo
        icon={<CalendarDays size={13} />}
        label={t("lotInfo.expires")}
        value={lot.expires_at ?? "-"}
      />
    </div>
  );
}

function InlineInfo(props: { icon: React.ReactNode; label: string; value: string }) {
  return (
    <div className="mt-dose-lot-inline__item">
      <div className="mt-dose-lot-inline__icon">{props.icon}</div>
      <div className="mt-min-w-0">
        <div className="mt-dose-lot-inline__label">{props.label}</div>
        <div className="mt-dose-lot-inline__value">{props.value}</div>
      </div>
    </div>
  );
}
