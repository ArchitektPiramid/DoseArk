import type { DoseLog } from "@doseark/types";

import { StockQuantityCell } from "@/core/ui";

import { formatChronologicalStock } from "../utils/dosingFormat";

export function DoseHistoryStockCell({ log }: { log: DoseLog }) {
  const transition = formatChronologicalStock(
    log.inventory_quantity_before,
    log.inventory_quantity_after,
  );

  return (
    <StockQuantityCell
      remaining={log.inventory_quantity_after}
      initial={log.inventory_units_initial}
      showUsedLabel={false}
      showBar={false}
      title={transition === "-" ? undefined : transition}
      className="mt-dose-history-stock__content"
    />
  );
}
