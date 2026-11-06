import { useTranslation } from "react-i18next";

import { formatDateTimeLocal, formatStockDelta, formatStockQuantity } from "@/core/format";
import { Badge, Panel, Table, TableEmptyRow, TableRowCount } from "@/core/ui";

import { useInventoryStockMovements } from "../hooks/useInventoryStockMovements";

type Props = {
  lotId?: string;
};

export function InventoryLotStockMovementsPanel({ lotId }: Props) {
  const { t } = useTranslation("inventory");
  const movements = useInventoryStockMovements(lotId);

  return (
    <Panel
      title={t("stockMovements.title")}
      action={<TableRowCount count={movements.rows.length} loading={movements.status === "loading"} />}
    >
      {movements.error ? (
        <pre className="mt-error-block">
          {movements.error}
        </pre>
      ) : null}

      <Table
        columns={[
          t("stockMovements.movementType"),
          t("stockMovements.quantityDelta"),
          t("stockMovements.quantityAfter"),
          t("stockMovements.occurredAt"),
          t("stockMovements.reason"),
        ]}
      >
        {movements.rows.length === 0 ? (
          <TableEmptyRow colSpan={5}>{t("stockMovements.empty")}</TableEmptyRow>
        ) : (
          movements.rows.map((row) => (
            <tr key={row.id}>
              <td>
                <Badge tone={row.movement_type === "manual_remove" ? "warn" : "ok"}>
                  {row.movement_type}
                </Badge>
              </td>
              <td className="mt-mono">{formatStockDelta(row.quantity_delta)}</td>
              <td className="mt-mono">{formatStockQuantity(row.quantity_after)}</td>
              <td className="mt-table-mono mt-table-small">{formatDateTimeLocal(row.occurred_at)}</td>
              <td>{row.reason ?? "-"}</td>
            </tr>
          ))
        )}
      </Table>
    </Panel>
  );
}
