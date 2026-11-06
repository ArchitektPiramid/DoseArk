import type { ReactNode } from "react";

import { Button } from "./Button";

type PageRange = {
  start: number;
  end: number;
  total: number;
};

type Props = {
  pageIndex: number;
  pageSize: number;
  totalCount: number;
  visibleCount: number;
  previousLabel: ReactNode;
  nextLabel: ReactNode;
  renderSummary: (range: PageRange) => ReactNode;
  onPageIndexChange: (pageIndex: number) => void;
};

export function TablePagination({
  pageIndex,
  pageSize,
  totalCount,
  visibleCount,
  previousLabel,
  nextLabel,
  renderSummary,
  onPageIndexChange,
}: Props) {
  if (totalCount <= 0) return null;

  const totalPages = Math.max(1, Math.ceil(totalCount / pageSize));
  const safePageIndex = Math.min(Math.max(pageIndex, 0), totalPages - 1);
  const pageStart = safePageIndex * pageSize;
  const pageEnd = Math.min(pageStart + visibleCount, totalCount);

  return (
    <div className="mt-table-pagination">
      <span className="mt-table-pagination__summary">
        {renderSummary({ start: pageStart + 1, end: pageEnd, total: totalCount })}
      </span>

      <div className="mt-table-pagination__actions">
        <Button
          variant="quiet"
          disabled={safePageIndex === 0}
          onClick={() => onPageIndexChange(safePageIndex - 1)}
        >
          {previousLabel}
        </Button>

        <span className="mt-table-pagination__page">
          {safePageIndex + 1} / {totalPages}
        </span>

        <Button
          variant="quiet"
          disabled={safePageIndex >= totalPages - 1}
          onClick={() => onPageIndexChange(safePageIndex + 1)}
        >
          {nextLabel}
        </Button>
      </div>
    </div>
  );
}
