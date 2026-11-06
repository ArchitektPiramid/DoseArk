import { CheckCircle2 } from "lucide-react";
import { Link } from "react-router";

import { formatStockFraction } from "@/core/format";
import { Badge, Panel } from "@/core/ui";
import type { DashboardInventoryAttentionItem } from "../utils/dashboardSummary";

const VISIBLE_ALERTS = 5;

/** Lists actionable inventory problems, rather than all inventory statuses. */
export function DashboardInventoryAttention({
  items,
  productNameById,
}: {
  items: DashboardInventoryAttentionItem[];
  productNameById: Map<string, string>;
}) {
  return (
    <Panel title="Inventory attention" action={<Link className="mt-dashboard-panel-link" to="/inventory">View inventory</Link>}>
      {items.length === 0 ? (
        <div className="mt-dashboard-positive-empty">
          <CheckCircle2 size={20} aria-hidden="true" />
          <span>No low-stock or near-expiry lots.</span>
        </div>
      ) : (
        <div className="mt-dashboard-list">
          {items.slice(0, VISIBLE_ALERTS).map(({ lot, reason, severity }) => (
            <div key={lot.id} className="mt-dashboard-list-row">
              <div className="mt-dashboard-list-row__main">
                <strong title={productNameById.get(lot.product_id) ?? lot.product_id}>
                  {productNameById.get(lot.product_id) ?? lot.product_id}
                </strong>
                <small>{formatStockFraction(lot.units_remaining, lot.units_initial)}</small>
              </div>
              <Badge tone={severity === "danger" ? "danger" : "warn"}>{reason}</Badge>
            </div>
          ))}
          {items.length > VISIBLE_ALERTS ? (
            <Link className="mt-dashboard-more-link" to="/inventory">
              +{items.length - VISIBLE_ALERTS} more lots need attention
            </Link>
          ) : null}
        </div>
      )}
    </Panel>
  );
}
