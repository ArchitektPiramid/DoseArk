import { useMemo } from "react";
import { Plus, RefreshCw } from "lucide-react";
import { Link } from "react-router";

import { ApiErrorNotice } from "@/app/system_status";
import { Button, PageHeader } from "@/core/ui";

import { DashboardAnalytics } from "../components/DashboardAnalytics";
import { DashboardInventoryAttention } from "../components/DashboardInventoryAttention";
import { DashboardRecentDoses } from "../components/DashboardRecentDoses";
import { DashboardSummaryCards } from "../components/DashboardSummaryCards";
import { DashboardTodayIntake } from "../components/DashboardTodayIntake";
import { DashboardUsageInsights } from "../components/DashboardUsageInsights";
import { useDashboardData } from "../hooks/useDashboardData";
import { getDoseActivity } from "../utils/dashboardActivity";
import {
  countActiveProducts,
  countTodayDoseLogs,
  countTodayProducts,
  getInventoryAttention,
} from "../utils/dashboardSummary";

/** Presents useful counts, two complementary trends and actionable recent records. */
export function DashboardPage() {
  const data = useDashboardData();
  const activity = useMemo(() => getDoseActivity(data.activityDoses), [data.activityDoses]);
  const inventoryAttention = useMemo(
    () => getInventoryAttention(data.lots, new Date(), data.lots.length),
    [data.lots],
  );
  const activityDoseCount = activity.reduce((sum, row) => sum + row.count, 0);
  const activeDays = activity.filter((row) => row.count > 0).length;
  const urgentAlertCount = inventoryAttention.filter((item) => item.severity === "danger").length;
  const activeLots = data.lots.filter((lot) => lot.status === "active" && lot.units_remaining > 0);
  const stockedProducts = new Set(activeLots.map((lot) => lot.product_id)).size;
  const hasNutrientIntake = (data.todayReport?.rows.length ?? 0) > 0;

  return (
    <div className="mt-page mt-dashboard-page">
      <PageHeader
        title="Dashboard"
        description="Your dosing activity, usage patterns and inventory in one place."
        action={
          <div className="mt-dashboard-header-actions">
            <Link to="/dosing" className="mt-dashboard-primary-action">
              <Plus size={15} aria-hidden="true" /> Record dose
            </Link>
            <Button onClick={() => void data.load()} disabled={data.isLoading}>
              <RefreshCw size={15} className={data.isLoading ? "mt-spin" : undefined} aria-hidden="true" />
              Refresh
            </Button>
          </div>
        }
      />

      {data.error ? <ApiErrorNotice error={data.error} onRetry={() => void data.load()} /> : null}

      {data.hasLoaded ? (
        <>
          <DashboardSummaryCards
            todayDoseCount={data.todayReport ? countTodayDoseLogs(data.todayReport) : null}
            todayProductCount={countTodayProducts(data.todayReport)}
            activityDoseCount={activityDoseCount}
            activeDays={activeDays}
            activityTruncated={data.activityTruncated}
            activeProductCount={countActiveProducts(data.products)}
            productCount={data.products.length}
            activeLotCount={activeLots.length}
            trackedProductCount={stockedProducts}
            alertCount={inventoryAttention.length}
            urgentAlertCount={urgentAlertCount}
          />

          <div className="mt-dashboard-section-heading">
            <h2>Activity &amp; patterns</h2>
            <span>Two-week trend and your recent dosing habits</span>
          </div>
          <div className="mt-dashboard-activity-grid">
            <DashboardAnalytics activity={activity} truncated={data.activityTruncated} />
            <DashboardUsageInsights
              doses={data.insightDoses}
              totalEntries={data.insightTotal}
              productNameById={data.productNameById}
            />
          </div>

          <div className="mt-dashboard-section-heading">
            <h2>Recent records</h2>
            <span>Latest entries and stock items worth checking</span>
          </div>
          <div className="mt-dashboard-overview-grid" data-has-intake={hasNutrientIntake}>
            <DashboardRecentDoses doses={data.recentDoses} productNameById={data.productNameById} />
            <DashboardInventoryAttention items={inventoryAttention} productNameById={data.productNameById} />
            {hasNutrientIntake ? <DashboardTodayIntake report={data.todayReport} /> : null}
          </div>
        </>
      ) : !data.error ? (
        <div className="mt-dashboard-loading">Loading dashboard overview…</div>
      ) : null}
    </div>
  );
}
