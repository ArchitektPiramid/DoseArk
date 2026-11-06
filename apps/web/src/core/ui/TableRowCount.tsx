type Props = {
  count: number;
  total?: number;
  loading?: boolean;
};

export function TableRowCount({ count, total, loading }: Props) {
  const value = total === undefined || total === count ? `${count} ROWS` : `${count}/${total} ROWS`;

  return <span className="mt-table-row-count">{loading ? "LOADING" : value}</span>;
}
