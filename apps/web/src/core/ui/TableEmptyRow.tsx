import type { ReactNode } from "react";

type Props = {
  colSpan: number;
  children: ReactNode;
};

export function TableEmptyRow({ colSpan, children }: Props) {
  return (
    <tr>
      <td colSpan={colSpan} className="mt-table-empty">
        {children}
      </td>
    </tr>
  );
}
