import type { StorageLocation } from "@doseark/types";

import { Badge, Panel, Table, TableEmptyRow, TableRowCount } from "@/core/ui";
import { VisualIcon } from "@/core/visualTags";

type Props = {
  rows: StorageLocation[];
  selectedId?: string | null;
  onSelect: (id: string) => void;
};

export function StorageLocationsTable({ rows, selectedId, onSelect }: Props) {
  return (
    <Panel
      title="Storage locations"
      action={<TableRowCount count={rows.length} />}
    >
      <Table columns={["Name", "Icon", "Color", "Sort", "Status"]}>
        {rows.length === 0 ? (
          <TableEmptyRow colSpan={5}>No storage locations yet.</TableEmptyRow>
        ) : (
          rows.map((row) => (
            <tr
              key={row.id}
              className={row.id === selectedId ? "mt-table-row--interactive is-selected" : "mt-table-row--interactive"}
              onClick={() => onSelect(row.id)}
            >
              <td>
                <div className="mt-table-cell-strong">{row.name}</div>
                <div className="mt-table-secondary">
                  {row.description ?? "-"}
                </div>
              </td>

              <td>
                <div className="mt-visual-chip" style={{ color: row.color ?? undefined }}>
                  <VisualIcon name={row.icon} />
                </div>
              </td>

              <td>
                <div className="mt-color-preview">
                  <span
                    className="mt-color-preview__swatch"
                    style={{ backgroundColor: row.color ?? "#6f91b8" }}
                  />
                  <span className="mt-mono">{row.color ?? "-"}</span>
                </div>
              </td>

              <td className="mt-mono">{row.sort_order}</td>

              <td>
                <Badge tone={row.is_archived ? "warn" : "ok"}>
                  {row.is_archived ? "archived" : "active"}
                </Badge>
              </td>
            </tr>
          ))
        )}
      </Table>
    </Panel>
  );
}
