import type { ReactNode } from "react";

export type TableSortDirection = "asc" | "desc";

export type TableSortState<TKey extends string = string> = {
  key: TKey;
  direction: TableSortDirection;
};

export type TableColumn<TKey extends string = string> =
  | string
  | {
      key: string;
      label: ReactNode;
      className?: string;
      sortKey?: TKey;
    };

type Props<TKey extends string = string> = {
  columns: TableColumn<TKey>[];
  children: ReactNode;
  sort?: TableSortState<TKey>;
  onSortChange?: (sort: TableSortState<TKey>) => void;
};

export function Table<TKey extends string = string>({
  columns,
  children,
  sort,
  onSortChange,
}: Props<TKey>) {
  return (
    <div className="mt-table-wrap">
      <table className="mt-table">
        <thead>
          <tr>
            {columns.map((column) => {
              if (typeof column === "string") {
                return <th key={column}>{column}</th>;
              }

              const sortable = column.sortKey !== undefined && onSortChange !== undefined;
              const active = sortable && sort?.key === column.sortKey;
              const direction = active ? sort?.direction : undefined;

              return (
                <th
                  key={column.key}
                  className={column.className}
                  aria-sort={
                    sortable
                      ? active
                        ? direction === "asc"
                          ? "ascending"
                          : "descending"
                        : "none"
                      : undefined
                  }
                >
                  {sortable ? (
                    <button
                      type="button"
                      className="mt-table-sort"
                      onClick={() =>
                        onSortChange({
                          key: column.sortKey as TKey,
                          direction: active && direction === "asc" ? "desc" : "asc",
                        })
                      }
                    >
                      <span>{column.label}</span>
                      <span
                        className={active ? "mt-table-sort__direction is-active" : "mt-table-sort__direction"}
                        aria-hidden="true"
                      >
                        {direction === "asc" ? "▲" : direction === "desc" ? "▼" : "↕"}
                      </span>
                    </button>
                  ) : (
                    column.label
                  )}
                </th>
              );
            })}
          </tr>
        </thead>

        <tbody>{children}</tbody>
      </table>
    </div>
  );
}
