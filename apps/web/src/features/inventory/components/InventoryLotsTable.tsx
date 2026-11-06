import type { InventoryLot, InventoryLotStatus, StorageLocation } from "@doseark/types";
import { useMemo, useState } from "react";

import { formatYearMonthLocal } from "@/core/format";
import { Panel, Table, TableEmptyRow, TableFilterToggle, TableRowCount } from "@/core/ui";

import { InventoryLocationChip } from "./InventoryLocationChip";
import { InventoryLotRemainingCell } from "./InventoryLotRemainingCell";
import { InventoryLotStatusBadge } from "./InventoryLotStatusBadge";
import { InventoryLotStockAlertIcon } from "./InventoryLotStockAlertIcon";
import {
  getInventoryLotStatusLabel,
  INVENTORY_LOT_STATUS_OPTIONS,
} from "../utils/inventoryLotStatus";
import { getInventoryLotAlertLevel } from "../utils/inventoryLotStockLevel";
import {
  sortInventoryLots,
  type InventoryLotSortKey,
  type InventoryLotSortState,
} from "../utils/inventoryLotSort";

type Props = {
  lots: InventoryLot[];
  selectedLotId?: string | null;
  loading?: boolean;
  productNameById: Map<string, string>;
  storageLocationById: Map<string, StorageLocation>;
  onSelect: (id: string) => void;
};

export function InventoryLotsTable({
  lots,
  selectedLotId,
  loading,
  productNameById,
  storageLocationById,
  onSelect,
}: Props) {
  const [visibleStatuses, setVisibleStatuses] = useState<Set<InventoryLotStatus>>(
    () => new Set(INVENTORY_LOT_STATUS_OPTIONS),
  );
  const [sort, setSort] = useState<InventoryLotSortState>({ key: "status", direction: "asc" });

  const statusCounts = useMemo(() => {
    const counts = new Map<InventoryLotStatus, number>();

    for (const lot of lots) {
      counts.set(lot.status, (counts.get(lot.status) ?? 0) + 1);
    }

    return counts;
  }, [lots]);

  const visibleLots = useMemo(
    () =>
      sortInventoryLots(
        lots.filter((lot) => visibleStatuses.has(lot.status)),
        sort,
        { productNameById, storageLocationById },
      ),
    [lots, productNameById, sort, storageLocationById, visibleStatuses],
  );

  function toggleStatus(status: InventoryLotStatus) {
    setVisibleStatuses((current) => {
      const next = new Set(current);

      if (next.has(status)) {
        next.delete(status);
      } else {
        next.add(status);
      }

      return next;
    });
  }


  return (
    <Panel
      className="mt-inventory-list-panel"
      title={
        <div className="mt-inventory-lots-title">
          <span>Inventory lots</span>
          <div className="mt-table-filters" aria-label="Inventory status filters">
            {INVENTORY_LOT_STATUS_OPTIONS.map((status) => {
              const count = statusCounts.get(status) ?? 0;
              if (count === 0) return null;

              return (
                <TableFilterToggle
                  key={status}
                  checked={visibleStatuses.has(status)}
                  label={getInventoryLotStatusLabel(status)}
                  count={count}
                  onChange={() => toggleStatus(status)}
                />
              );
            })}
          </div>
        </div>
      }
      action={<TableRowCount count={visibleLots.length} total={lots.length} loading={loading} />}
    >
      <div className="mt-inventory-lots-table">
        <Table<InventoryLotSortKey>
          columns={[
            { key: "product", label: "Product", sortKey: "product" },
            { key: "lot", label: "Lot", sortKey: "lot" },
            { key: "remaining", label: "Remaining", sortKey: "remaining" },
            { key: "status", label: "Status", sortKey: "status" },
            { key: "expires", label: "Expires", sortKey: "expires" },
            { key: "storage", label: "Storage", sortKey: "storage" },
          ]}
          sort={sort}
          onSortChange={setSort}
        >
          {visibleLots.length === 0 ? (
            <TableEmptyRow colSpan={6}>No inventory lots match selected filters.</TableEmptyRow>
          ) : (
            visibleLots.map((lot) => {
              const location = lot.storage_location_id
                ? storageLocationById.get(lot.storage_location_id)
                : undefined;
              const alertLevel = getInventoryLotAlertLevel(lot);

              return (
                <tr
                  key={lot.id}
                  className={[
                    "mt-table-row--interactive",
                    lot.id === selectedLotId ? "is-selected" : "",
                    lot.status !== "active" ? "mt-table-row--muted" : "",
                  ]
                    .filter(Boolean)
                    .join(" ")}
                  title="Click to edit inventory lot"
                  onClick={() => onSelect(lot.id)}
                >
                  <td>
                    <div className="mt-inventory-product-cell">
                      <InventoryLotStockAlertIcon level={alertLevel} />
                      <div className="mt-inventory-product-cell__name mt-table-cell-strong">
                        {productNameById.get(lot.product_id) ?? "Unknown product"}
                      </div>
                    </div>
                  </td>
                  <td className="mt-inventory-lots-table__lot" title={lot.lot_number ?? undefined}>{lot.lot_number ?? "-"}</td>
                  <td>
                    <InventoryLotRemainingCell
                      remaining={lot.units_remaining}
                      initial={lot.units_initial}
                    />
                  </td>
                  <td>
                    <InventoryLotStatusBadge status={lot.status} />
                  </td>
                  <td className="mt-inventory-lots-table__expires">
                    {formatYearMonthLocal(lot.expires_at)}
                  </td>
                  <td className="mt-inventory-lots-table__storage" title={location?.name ?? lot.storage_location ?? undefined}>
                    <InventoryLocationChip lot={lot} location={location} />
                  </td>
                </tr>
              );
            })
          )}
        </Table>
      </div>
    </Panel>
  );
}
