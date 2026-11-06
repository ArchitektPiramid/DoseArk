import type { DailyIntakeReport, InventoryLot, Product } from "@doseark/types";

import { getInventoryLotAlertLevel, getInventoryLotRemainingPercent } from "@/features/inventory/utils/inventoryLotStockLevel";

const EXPIRY_WINDOW_DAYS = 30;
const DAY_MS = 24 * 60 * 60 * 1000;

export type DashboardInventoryAttentionItem = {
  lot: InventoryLot;
  reason: string;
  severity: "warning" | "danger";
  sortKey: number;
};

export function countActiveProducts(products: Product[]): number {
  return products.filter((product) => !product.archived).length;
}

export function countTodayDoseLogs(report: DailyIntakeReport | null): number {
  return report?.dose_logs.filter((dose) => dose.status === "active").length ?? 0;
}

export function countTodayProducts(report: DailyIntakeReport | null): number {
  return new Set((report?.dose_logs ?? [])
    .filter((dose) => dose.status === "active")
    .map((dose) => dose.product_id)).size;
}

/** Produces one alert per affected lot, prioritizing the most urgent reason. */
export function getInventoryAttention(
  lots: InventoryLot[],
  now = new Date(),
  limit = 7,
): DashboardInventoryAttentionItem[] {
  const items: DashboardInventoryAttentionItem[] = [];

  for (const lot of lots) {
    const stockAlert = getInventoryLotAlertLevel(lot);
    const daysToExpiry = getDaysUntilExpiry(lot.expires_at, now);
    const remainingPercent = getInventoryLotRemainingPercent(lot.units_remaining, lot.units_initial);

    if (lot.status === "expired") {
      items.push({ lot, reason: "Expired", severity: "danger", sortKey: -10_000 });
      continue;
    }

    if (lot.status === "active" && lot.units_remaining <= 0) {
      items.push({ lot, reason: "Out of stock", severity: "danger", sortKey: -9_000 });
      continue;
    }

    if (daysToExpiry !== null && daysToExpiry < 0 && isTrackedLot(lot)) {
      items.push({ lot, reason: "Past expiry date", severity: "danger", sortKey: -8_000 });
      continue;
    }

    if (stockAlert) {
      items.push({
        lot,
        reason: remainingPercent === null ? "Low stock" : `${Math.max(0, remainingPercent).toFixed(1)}% remaining`,
        severity: stockAlert,
        sortKey: remainingPercent ?? 100,
      });
      continue;
    }

    if (daysToExpiry !== null && daysToExpiry >= 0 && daysToExpiry <= EXPIRY_WINDOW_DAYS && isTrackedLot(lot)) {
      items.push({
        lot,
        reason: daysToExpiry === 0 ? "Expires today" : `Expires in ${daysToExpiry}d`,
        severity: daysToExpiry <= 7 ? "danger" : "warning",
        sortKey: 100 + daysToExpiry,
      });
    }
  }

  return items
    .sort((left, right) => {
      if (left.severity !== right.severity) return left.severity === "danger" ? -1 : 1;
      return left.sortKey - right.sortKey;
    })
    .slice(0, limit);
}

function getDaysUntilExpiry(value: string | null, now: Date): number | null {
  if (!value) return null;
  const expiry = new Date(`${value.slice(0, 10)}T23:59:59`);
  if (Number.isNaN(expiry.getTime())) return null;
  return Math.ceil((expiry.getTime() - now.getTime()) / DAY_MS);
}

function isTrackedLot(lot: InventoryLot): boolean {
  return lot.status === "active" || lot.status === "reserve" || lot.status === "on_hold";
}

