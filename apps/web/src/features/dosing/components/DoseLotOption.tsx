import type { InventoryLot } from "@doseark/types";
import { CalendarDays, Package, Pill } from "lucide-react";

import { formatLotStock } from "../utils/dosingFormat";

type Props = {
  productName: string;
  lot: InventoryLot;
};

export function DoseLotOption({ productName, lot }: Props) {
  return (
    <div className="mt-listbox-option-row">
      <div className="mt-listbox-option-row__icon">
        <Package size={15} />
      </div>

      <div className="mt-listbox-option-row__main">
        <div className="mt-listbox-option-row__title">{productName}</div>

        <div className="mt-listbox-option-row__meta">
          <span>{lot.lot_number ? `Lot ${lot.lot_number}` : "No lot number"}</span>

          <span className="mt-listbox-option-row__pill">
            <Pill size={12} />
            {formatLotStock(lot.units_remaining, lot.units_initial)}
          </span>

          <span
            className={
              lot.expires_at
                ? "mt-listbox-option-row__pill"
                : "mt-listbox-option-row__pill mt-listbox-option-row__pill--warn"
            }
          >
            <CalendarDays size={12} />
            {lot.expires_at ?? "-"}
          </span>
        </div>
      </div>
    </div>
  );
}
