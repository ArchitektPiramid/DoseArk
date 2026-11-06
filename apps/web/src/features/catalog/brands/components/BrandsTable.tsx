import type { Brand } from "@doseark/types";
import { Panel, Table, TableEmptyRow, TableRowCount } from "@/core/ui";

type Props = {
  brands: Brand[];
  selectedBrandId?: string | null;
  loading?: boolean;
  onSelect: (id: string) => void;
};

export function BrandsTable({ brands, selectedBrandId, loading, onSelect }: Props) {
  return (
    <Panel
      title="Brands"
      action={<TableRowCount count={brands.length} loading={loading} />}
    >
      <Table columns={["Name", "Country", "Website", "Updated"]}>
        {brands.length === 0 ? (
          <TableEmptyRow colSpan={4}>No brands yet.</TableEmptyRow>
        ) : (
          brands.map((brand) => (
            <tr
              key={brand.id}
              className={brand.id === selectedBrandId ? "mt-table-row--interactive is-selected" : "mt-table-row--interactive"}
              onClick={() => onSelect(brand.id)}
            >
              <td>
                <div className="mt-table-cell-strong">{brand.name}</div>
                <div className="mt-table-secondary mt-mono">{brand.id}</div>
              </td>
              <td>{brand.country ?? "-"}</td>
              <td>{brand.website ?? "-"}</td>
              <td className="mt-table-mono mt-table-small">{brand.updated_at}</td>
            </tr>
          ))
        )}
      </Table>
    </Panel>
  );
}
