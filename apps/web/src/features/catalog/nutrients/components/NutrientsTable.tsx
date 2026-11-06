import type { Nutrient } from "@doseark/types";
import { Badge, Panel, Table, TableEmptyRow, TableRowCount } from "@/core/ui";

type Props = {
  nutrients: Nutrient[];
  selectedNutrientId?: string | null;
  loading?: boolean;
  onSelect: (id: string) => void;
};

export function NutrientsTable({ nutrients, selectedNutrientId, loading, onSelect }: Props) {
  return (
    <Panel
      title="Nutrients"
      action={<TableRowCount count={nutrients.length} loading={loading} />}
    >
      <Table columns={["Display name", "Canonical name", "Kind", "Unit", "Updated"]}>
        {nutrients.length === 0 ? (
          <TableEmptyRow colSpan={5}>No nutrients yet.</TableEmptyRow>
        ) : (
          nutrients.map((nutrient) => (
            <tr
              key={nutrient.id}
              className={nutrient.id === selectedNutrientId ? "mt-table-row--interactive is-selected" : "mt-table-row--interactive"}
              onClick={() => onSelect(nutrient.id)}
            >
              <td>
                <div className="mt-table-cell-strong">{nutrient.display_name}</div>
                <div className="mt-table-secondary mt-mono">{nutrient.id}</div>
              </td>
              <td className="mt-table-mono mt-table-small">{nutrient.canonical_name}</td>
              <td>
                <Badge>{nutrient.kind}</Badge>
              </td>
              <td>{nutrient.default_unit}</td>
              <td className="mt-table-mono mt-table-small">{nutrient.updated_at}</td>
            </tr>
          ))
        )}
      </Table>
    </Panel>
  );
}
