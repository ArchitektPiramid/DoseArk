import type { DailyIntakeReport } from "@doseark/types";
import { Link } from "react-router";

import { Panel } from "@/core/ui";

/** Summarizes nutrients logged today when the daily report contains rows. */
export function DashboardTodayIntake({ report }: { report: DailyIntakeReport | null }) {
  const rows = [...(report?.rows ?? [])]
    .sort((left, right) => right.dose_count - left.dose_count || right.amount_total - left.amount_total)
    .slice(0, 7);

  return (
    <Panel
      title="Nutrients today"
      action={<Link className="mt-dashboard-panel-link" to="/statistics">See statistics</Link>}
    >
      {rows.length === 0 ? (
        <div className="mt-empty-state">No nutrient intake recorded today.</div>
      ) : (
        <div className="mt-dashboard-list">
          {rows.map((row) => (
            <div key={`${row.nutrient_id}:${row.unit}`} className="mt-dashboard-list-row">
              <div className="mt-dashboard-list-row__main">
                <strong>{row.nutrient_display_name}</strong>
                <small>{row.dose_count} dose{row.dose_count === 1 ? "" : "s"}</small>
              </div>
              <div className="mt-dashboard-list-row__value">
                {formatAmount(row.amount_total)} {row.unit}
                {row.percent_of_reference_total !== null ? (
                  <small>{formatAmount(row.percent_of_reference_total)}% reference</small>
                ) : null}
              </div>
            </div>
          ))}
        </div>
      )}
    </Panel>
  );
}

function formatAmount(value: number): string {
  return new Intl.NumberFormat(undefined, { maximumFractionDigits: 2 }).format(value);
}
