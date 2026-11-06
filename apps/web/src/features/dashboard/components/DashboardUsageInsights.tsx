import { useMemo, useState } from "react";
import { Link } from "react-router";
import { Clock3, TrendingUp } from "lucide-react";
import type { DoseLog } from "@doseark/types";

import { Panel } from "@/core/ui";
import { getDashboardUsageInsights } from "../utils/dashboardInsights";

type InsightView = "products" | "hours";

/** Combines product frequency and dose timing in one compact insight panel. */
export function DashboardUsageInsights({
  doses,
  totalEntries,
  productNameById,
}: {
  doses: DoseLog[];
  totalEntries: number;
  productNameById: Map<string, string>;
}) {
  const [view, setView] = useState<InsightView>("products");
  const insights = useMemo(() => getDashboardUsageInsights(doses), [doses]);
  const sampleIsPartial = totalEntries > doses.length;
  const maxCount = insights.topProducts[0]?.count ?? 0;
  const maxHourlyCount = Math.max(...insights.hours);

  return (
    <Panel
      title="Usage patterns"
      className="mt-dashboard-insights"
      action={<Link className="mt-dashboard-panel-link" to="/statistics">Explore statistics</Link>}
    >
      <div className="mt-dashboard-insight-toolbar">
        <div className="mt-dashboard-view-switch" role="group" aria-label="Usage pattern view">
          <button type="button" aria-pressed={view === "products"} onClick={() => setView("products")}>
            <TrendingUp size={13} aria-hidden="true" /> Top products
          </button>
          <button type="button" aria-pressed={view === "hours"} onClick={() => setView("hours")}>
            <Clock3 size={13} aria-hidden="true" /> By hour
          </button>
        </div>
      </div>

      <p className="mt-dashboard-insight-caption">
        {insights.activeDoses} active doses in recent history
        {sampleIsPartial ? ` · latest ${doses.length} of ${totalEntries} records` : ""}
      </p>

      {insights.activeDoses === 0 ? (
        <div className="mt-dashboard-insight-empty">
          No recorded doses to summarize yet.
        </div>
      ) : view === "products" ? (
        <ol className="mt-dashboard-ranking" aria-label="Most frequently logged products">
          {insights.topProducts.map((product, index) => (
            <li key={product.productId} className="mt-dashboard-ranking-row">
              <span className="mt-dashboard-ranking-index">{index + 1}</span>
              <div className="mt-dashboard-ranking-content">
                <div className="mt-dashboard-ranking-label">
                  <span title={productNameById.get(product.productId) ?? product.productId}>
                    {productNameById.get(product.productId) ?? product.productId}
                  </span>
                  <strong>{product.count}</strong>
                </div>
                <div className="mt-dashboard-ranking-track" aria-hidden="true">
                  <span style={{ width: `${maxCount ? (product.count / maxCount) * 100 : 0}%` }} />
                </div>
              </div>
            </li>
          ))}
        </ol>
      ) : (
        <div className="mt-dashboard-hours-view">
          <div className="mt-dashboard-hours-summary">
            <strong>{insights.peakHour === null ? "—" : `${String(insights.peakHour).padStart(2, "0")}:00`}</strong>
            <span>Most common dose hour (local time)</span>
          </div>
          <div className="mt-dashboard-hours-chart" role="img" aria-label="Doses recorded in each hour of the day">
            {insights.hours.map((count, hour) => (
              <div
                key={hour}
                className="mt-dashboard-hours-column"
                title={`${String(hour).padStart(2, "0")}:00 · ${count} doses`}
              >
                <span style={{ height: `${maxHourlyCount ? (count / maxHourlyCount) * 100 : 0}%` }} />
              </div>
            ))}
          </div>
          <div className="mt-dashboard-hours-axis" aria-hidden="true">
            <span>00:00</span><span>06:00</span><span>12:00</span><span>18:00</span><span>23:00</span>
          </div>
        </div>
      )}
    </Panel>
  );
}
