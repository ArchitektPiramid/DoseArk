import type { DashboardDoseActivityPoint } from "../utils/dashboardActivity";
import { useMemo } from "react";
import { Link } from "react-router";
import { Activity } from "lucide-react";

import { Panel } from "@/core/ui";

import { DashboardChart } from "./DashboardChart";
import { createDashboardDoseActivityOption } from "../utils/dashboardCharts";

/** Displays a single useful trend, hiding the chart when no doses exist in the period. */
export function DashboardAnalytics({
  activity,
  truncated,
}: {
  activity: DashboardDoseActivityPoint[];
  truncated: boolean;
}) {
  const count = activity.reduce((sum, day) => sum + day.count, 0);
  const option = useMemo(() => createDashboardDoseActivityOption(activity), [activity]);

  return (
    <Panel
      title="Dose activity"
      action={<Link className="mt-dashboard-panel-link" to="/statistics">Detailed statistics</Link>}
    >
      <div className="mt-dashboard-chart-heading">
        <span>Daily recorded doses · last 14 days</span>
        <strong>{truncated ? "At least " : ""}{count} doses</strong>
      </div>
      {count > 0 ? (
        <>
          <DashboardChart option={option} />
          {truncated ? <p className="mt-dashboard-data-note">Some older entries could not be included in this overview.</p> : null}
        </>
      ) : (
        <div className="mt-dashboard-activity-empty">
          <Activity size={24} aria-hidden="true" />
          <strong>No doses in the last 14 days</strong>
          <span>Your earlier dose history is still available.</span>
          <Link to="/dosing" className="mt-dashboard-panel-link">Record a dose</Link>
        </div>
      )}
    </Panel>
  );
}
