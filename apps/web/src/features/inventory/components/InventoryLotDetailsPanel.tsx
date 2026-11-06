import type { InventoryLot, StorageLocation } from "@doseark/types";
import type { ReactNode } from "react";

import { Panel } from "@/core/ui";
import { formatMoney, formatRemainingPercent, formatStockFraction } from "@/core/format";

import { InventoryLocationChip } from "./InventoryLocationChip";
import { InventoryLotStatusBadge } from "./InventoryLotStatusBadge";

type Props = {
  lot?: InventoryLot;
  productName?: string;
  storageLocation?: StorageLocation;
};

export function InventoryLotDetailsPanel({ lot, productName, storageLocation }: Props) {
  if (!lot) {
    return (
      <Panel title="Lot details">
        <div className="mt-empty-state">
          Select or create an inventory lot.
        </div>
      </Panel>
    );
  }

  return (
    <Panel title="Selected lot">
      <div className="mt-detail-list">
        <Detail label="Product" value={productName ?? "Unknown product"} />
        <Detail label="Lot number" value={lot.lot_number ?? "-"} />
        <DetailNode label="Storage">
          <InventoryLocationChip lot={lot} location={storageLocation} />
        </DetailNode>
        <DetailNode label="Status">
          <InventoryLotStatusBadge status={lot.status} />
        </DetailNode>
        <Detail label="Stock" value={formatStockFraction(lot.units_remaining, lot.units_initial)} />
        <Detail
          label="Remaining %"
          value={formatRemainingPercent(lot.units_remaining, lot.units_initial)}
        />
        <Detail label="Expires" value={lot.expires_at ?? "-"} />
        <Detail label="Price" value={formatMoney(lot.price_amount, lot.price_currency)} />
      </div>
    </Panel>
  );
}

function Detail(props: { label: string; value: string }) {
  return (
    <div className="mt-detail-row">
      <div className="mt-label">{props.label}</div>
      <div className="mt-detail-value">{props.value}</div>
    </div>
  );
}

function DetailNode(props: { label: string; children: ReactNode }) {
  return (
    <div className="mt-detail-row">
      <div className="mt-label">{props.label}</div>
      <div>{props.children}</div>
    </div>
  );
}
