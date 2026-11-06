import type { DoseLog } from "@doseark/types";
import { Link } from "react-router";

import { formatDateLocal, formatTimeLocal } from "@/core/format";
import { Panel } from "@/core/ui";
import { formatDoseAmountCompact } from "@/features/dosing/utils/dosingFormat";

/** Shows the most recent recorded doses, including entries outside the chart's period. */
export function DashboardRecentDoses({
  doses,
  productNameById,
}: {
  doses: DoseLog[];
  productNameById: Map<string, string>;
}) {
  return (
    <Panel title="Recent doses" action={<Link className="mt-dashboard-panel-link" to="/dosing">Dose history</Link>}>
      {doses.length === 0 ? (
        <div className="mt-dashboard-list-empty">No recorded doses yet.</div>
      ) : (
        <div className="mt-dashboard-list">
          {doses.map((dose) => (
            <div key={dose.id} className="mt-dashboard-list-row" data-muted={dose.status !== "active" || undefined}>
              <div className="mt-dashboard-list-row__main">
                <strong title={productNameById.get(dose.product_id) ?? dose.product_id}>
                  {productNameById.get(dose.product_id) ?? dose.product_id}
                </strong>
                <small>{formatDateLocal(dose.taken_at)} · {formatTimeLocal(dose.taken_at)}</small>
              </div>
              <div className="mt-dashboard-list-row__value">
                {formatDoseAmountCompact(dose.units_taken, dose.unit)}
                {dose.status !== "active" ? <small>{dose.status}</small> : null}
              </div>
            </div>
          ))}
        </div>
      )}
    </Panel>
  );
}
