import type { DoseLog, DoseLogPage } from "@doseark/types";

export type DashboardDoseActivityPoint = { date: string; count: number };

/** Returns the first local-calendar day of the displayed 14-day period. */
export function getActivityStart(now = new Date(), days = 14): Date {
  return new Date(now.getFullYear(), now.getMonth(), now.getDate() - (days - 1));
}

/** Groups active doses into local days, including days without a dose. */
export function getDoseActivity(
  doses: DoseLog[],
  days = 14,
  now = new Date(),
): DashboardDoseActivityPoint[] {
  const counts = new Map<string, number>();
  for (const dose of doses) {
    if (dose.status !== "active") continue;
    const date = new Date(dose.taken_at);
    if (!Number.isFinite(date.getTime())) continue;
    const key = toLocalDateKey(date);
    counts.set(key, (counts.get(key) ?? 0) + 1);
  }

  const start = getActivityStart(now, days);
  return Array.from({ length: days }, (_, index) => {
    const day = new Date(start);
    day.setDate(day.getDate() + index);
    const key = toLocalDateKey(day);
    return { date: key, count: counts.get(key) ?? 0 };
  });
}

/** Fetches only enough history to cover the period; marks capped results as partial. */
export async function loadActivityDoses(
  firstPage: DoseLogPage,
  since: Date,
  fetchNextPage: (offset: number) => Promise<DoseLogPage>,
  maxRows = 2_000,
): Promise<{ doses: DoseLog[]; truncated: boolean }> {
  const rows = [...firstPage.items];
  let offset = rows.length;
  let oldest = rows.at(-1);

  while (
    offset < firstPage.total &&
    rows.length < maxRows &&
    oldest &&
    new Date(oldest.taken_at).getTime() >= since.getTime()
  ) {
    const page = await fetchNextPage(offset);
    if (page.items.length === 0) break;
    rows.push(...page.items);
    offset += page.items.length;
    oldest = page.items.at(-1);
  }

  const truncated = offset < firstPage.total &&
    !!oldest && new Date(oldest.taken_at).getTime() >= since.getTime();
  const doses = rows.filter((dose) => {
    const timestamp = new Date(dose.taken_at).getTime();
    return dose.status === "active" && Number.isFinite(timestamp) && timestamp >= since.getTime();
  });
  return { doses, truncated };
}

function toLocalDateKey(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
}
