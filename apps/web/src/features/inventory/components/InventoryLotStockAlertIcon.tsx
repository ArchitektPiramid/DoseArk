import { AlertTriangle, CircleAlert } from "lucide-react";

import type { InventoryLotStockLevel } from "../utils/inventoryLotStockLevel";

type Props = {
  level: Exclude<InventoryLotStockLevel, "normal"> | null;
};

export function InventoryLotStockAlertIcon({ level }: Props) {
  if (!level) {
    return <span className="mt-inventory-stock-alert mt-inventory-stock-alert--placeholder" />;
  }

  const Icon = level === "danger" ? CircleAlert : AlertTriangle;
  const label = level === "danger" ? "Critical stock" : "Low stock";

  return (
    <span
      className={`mt-inventory-stock-alert mt-inventory-stock-alert--${level}`}
      title={label}
      aria-label={label}
    >
      <Icon size={15} strokeWidth={2.4} />
    </span>
  );
}
