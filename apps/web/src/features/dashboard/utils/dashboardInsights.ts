import type { DoseLog } from "@doseark/types";

export type DashboardProductUsage = { productId: string; count: number };
export type DashboardUsageInsights = {
  activeDoses: number;
  topProducts: DashboardProductUsage[];
  hours: number[];
  peakHour: number | null;
};

/** Summarizes a bounded dose-log sample without treating it as lifetime history. */
export function getDashboardUsageInsights(doses: DoseLog[]): DashboardUsageInsights {
  const counts = new Map<string, number>();
  const hours = Array.from({ length: 24 }, () => 0);
  let activeDoses = 0;

  for (const dose of doses) {
    if (dose.status !== "active") continue;
    const timestamp = new Date(dose.taken_at);
    if (!Number.isFinite(timestamp.getTime())) continue;

    activeDoses += 1;
    counts.set(dose.product_id, (counts.get(dose.product_id) ?? 0) + 1);
    hours[timestamp.getHours()] += 1;
  }

  const topProducts = [...counts.entries()]
    .map(([productId, count]) => ({ productId, count }))
    .sort((left, right) => right.count - left.count || left.productId.localeCompare(right.productId))
    .slice(0, 5);
  const highestHourCount = Math.max(...hours);

  return {
    activeDoses,
    topProducts,
    hours,
    peakHour: highestHourCount ? hours.indexOf(highestHourCount) : null,
  };
}
