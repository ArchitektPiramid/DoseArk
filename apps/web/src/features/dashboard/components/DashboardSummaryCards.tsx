import { AlertTriangle, CalendarCheck2, CalendarRange, Package, Boxes } from "lucide-react";
import { Link } from "react-router";

type Card = {
  to: string;
  icon: typeof Package;
  label: string;
  value: string;
  detail: string;
  tone?: "warning" | "success";
};

/** Keeps the five main dashboard counts together without a second metric strip. */
export function DashboardSummaryCards({
  todayDoseCount,
  todayProductCount,
  activityDoseCount,
  activeDays,
  activityTruncated,
  activeProductCount,
  productCount,
  activeLotCount,
  trackedProductCount,
  alertCount,
  urgentAlertCount,
}: {
  todayDoseCount: number | null;
  todayProductCount: number;
  activityDoseCount: number;
  activeDays: number;
  activityTruncated: boolean;
  activeProductCount: number;
  productCount: number;
  activeLotCount: number;
  trackedProductCount: number;
  alertCount: number;
  urgentAlertCount: number;
}) {
  const cards: Card[] = [
    {
      to: "/dosing",
      icon: CalendarCheck2,
      label: "Today",
      value: todayDoseCount === null ? "—" : `${todayDoseCount} doses`,
      detail: todayDoseCount === null ? "Daily report unavailable" : `${todayProductCount} products taken`,
      tone: todayDoseCount !== null && todayDoseCount > 0 ? "success" : undefined,
    },
    {
      to: "/statistics",
      icon: CalendarRange,
      label: "Last 14 days",
      value: `${activityTruncated ? "≥" : ""}${activityDoseCount} doses`,
      detail: `${activeDays}/14 active days${activityTruncated ? " · partial" : ""}`,
    },
    {
      to: "/products",
      icon: Package,
      label: "Catalog",
      value: `${activeProductCount} active`,
      detail: `${productCount} total products`,
    },
    {
      to: "/inventory",
      icon: Boxes,
      label: "Inventory",
      value: `${activeLotCount} active lots`,
      detail: `${trackedProductCount} products with stock`,
    },
    {
      to: "/inventory",
      icon: AlertTriangle,
      label: "Needs attention",
      value: `${alertCount} ${alertCount === 1 ? "lot" : "lots"}`,
      detail: alertCount === 0 ? "No stock alerts" : `${urgentAlertCount} urgent`,
      tone: alertCount > 0 ? "warning" : "success",
    },
  ];

  return (
    <div className="mt-dashboard-summary-grid" aria-label="Dashboard overview">
      {cards.map((card) => {
        const Icon = card.icon;
        return (
          <Link key={card.label} to={card.to} className="mt-dashboard-summary-card" data-status={card.tone}>
            <span className="mt-dashboard-summary-card__icon"><Icon size={19} aria-hidden="true" /></span>
            <span className="mt-dashboard-summary-card__content">
              <span className="mt-dashboard-summary-card__title">{card.label}</span>
              <strong>{card.value}</strong>
              <small>{card.detail}</small>
            </span>
          </Link>
        );
      })}
    </div>
  );
}
