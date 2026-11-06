import type { DoseLogNutrient } from "@doseark/types";
import { useTranslation } from "react-i18next";

import { Badge, Panel, Table, TableEmptyRow, TableRowCount } from "@/core/ui";
import { formatDoseNumber } from "@/features/dosing/utils/dosingFormat";

type Props = {
  selectedDoseLogId?: string | null;
  rows: DoseLogNutrient[];
  loading?: boolean;
};

export function DoseLogNutrientsPanel({ selectedDoseLogId, rows, loading }: Props) {
  const { t } = useTranslation("dosing");

  return (
    <Panel
      title={t("nutrients.title")}
      action={<TableRowCount count={rows.length} loading={loading} />}
    >
      {!selectedDoseLogId ? (
        <div className="mt-empty-state">{t("nutrients.selectDose")}</div>
      ) : (
        <Table
          columns={[
            t("nutrients.nutrient"),
            t("nutrients.amount"),
            t("nutrients.reference"),
            t("nutrients.source"),
          ]}
        >
          {rows.length === 0 ? (
            <TableEmptyRow colSpan={4}>{t("nutrients.empty")}</TableEmptyRow>
          ) : (
            rows.map((row) => (
              <tr key={row.id}>
                <td>
                  <div className="mt-table-cell-strong">{row.nutrient_display_name}</div>
                  <div className="mt-table-secondary mt-mono">
                    {row.nutrient_canonical_name}
                  </div>
                </td>

                <td className="mt-mono">
                  {formatDoseNumber(row.amount)} {row.unit}
                </td>

                <td className="mt-mono">
                  {row.percent_of_reference === null
                    ? "-"
                    : `${formatDoseNumber(row.percent_of_reference)}%`}
                </td>

                <td>
                  <Badge tone={row.reference_source ? "ok" : "warn"}>
                    {row.reference_source ?? "no_reference"}
                  </Badge>
                </td>
              </tr>
            ))
          )}
        </Table>
      )}
    </Panel>
  );
}
