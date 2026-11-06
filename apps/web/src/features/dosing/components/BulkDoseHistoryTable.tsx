import type { DoseLog, Product } from "@doseark/types";
import { useTranslation } from "react-i18next";

import { Button, Panel, Table, TableEmptyRow, TablePagination } from "@/core/ui";
import { cx } from "@/core/ui/utils";

import { formatDateLocal, formatDoseAmountCompact, formatTimeLocal } from "../utils/dosingFormat";
import { DoseHistoryStockCell } from "./DoseHistoryStockCell";
import { DoseHistoryToolbar } from "./DoseHistoryToolbar";

const PAGE_SIZE_OPTIONS = [25, 50, 100, 200] as const;

type Props = {
  doseLogs: DoseLog[];
  totalCount: number;
  pageSize: number;
  pageIndex: number;
  products: Product[];
  productNameById: Map<string, string>;
  selectedDoseLogId?: string | null;
  historyProductId: string;
  onHistoryProductChange: (productId: string) => void;
  onPageSizeChange: (pageSize: number) => void;
  onPageIndexChange: (pageIndex: number) => void;
  onSelectDoseLog: (doseLogId: string) => void;
  onEditDoseLog: (doseLogId: string) => void;
  onCancelDoseLog: (doseLogId: string) => void;
};

export function BulkDoseHistoryTable({
  doseLogs,
  totalCount,
  pageSize,
  pageIndex,
  products,
  productNameById,
  selectedDoseLogId,
  historyProductId,
  onHistoryProductChange,
  onPageSizeChange,
  onPageIndexChange,
  onSelectDoseLog,
  onEditDoseLog,
  onCancelDoseLog,
}: Props) {
  const { t } = useTranslation("dosing");
  return (
    <Panel
      title={t("table.title")}
      className="mt-dose-history-panel"
      action={
        <DoseHistoryToolbar
          products={products}
          productNameById={productNameById}
          historyProductId={historyProductId}
          pageSize={pageSize}
          pageSizeOptions={PAGE_SIZE_OPTIONS}
          compact
          onHistoryProductChange={onHistoryProductChange}
          onPageSizeChange={onPageSizeChange}
        />
      }
    >
      <div className="mt-dose-history-table mt-bulk-dose-history-table">
        <Table
          columns={[
            t("table.date"),
            t("table.time"),
            t("table.product"),
            t("table.amount"),
            t("table.stock"),
            t("table.actions"),
          ]}
        >
          {doseLogs.length === 0 ? (
            <TableEmptyRow colSpan={6}>{t("table.empty")}</TableEmptyRow>
          ) : (
            doseLogs.map((log, index) => {
              const dateLabel = formatDateLocal(log.taken_at);
              const previousDateLabel = index > 0 ? formatDateLocal(doseLogs[index - 1].taken_at) : dateLabel;
              const startsNewDay = index > 0 && dateLabel !== previousDateLabel;
              const productName = productNameById.get(log.product_id) ?? "Unknown product";

              return (
                <tr
                  key={log.id}
                  className={cx(
                    "mt-table-row--interactive",
                    log.id === selectedDoseLogId && "is-selected",
                    startsNewDay && "mt-dose-history-row--new-day",
                  )}
                  onClick={() => onSelectDoseLog(log.id)}
                >
                  <td className="mt-mono mt-dose-history-date" data-label={t("table.date")}>
                    {dateLabel}
                  </td>
                  <td className="mt-mono mt-dose-history-time" data-label={t("table.time")}>
                    {formatTimeLocal(log.taken_at)}
                  </td>
                  <td
                    className="mt-dose-history-product"
                    data-label={t("table.product")}
                    title={productName}
                  >
                    {productName}
                  </td>
                  <td className="mt-mono mt-dose-history-amount" data-label={t("table.amount")}>
                    {formatDoseAmountCompact(log.units_taken, log.unit)}
                  </td>
                  <td className="mt-mono" data-label={t("table.stock")}>
                    <DoseHistoryStockCell log={log} />
                  </td>
                  <td className="mt-dose-history-actions-cell" data-label={t("table.actions")}>
                    <div className="mt-dose-history-actions">
                      <Button
                        variant="quiet"
                        onClick={(event) => {
                          event.stopPropagation();
                          onEditDoseLog(log.id);
                        }}
                      >
                        {t("table.edit")}
                      </Button>
                      <Button
                        variant="danger"
                        onClick={(event) => {
                          event.stopPropagation();
                          onCancelDoseLog(log.id);
                        }}
                      >
                        {t("table.delete")}
                      </Button>
                    </div>
                  </td>
                </tr>
              );
            })
          )}
        </Table>
      </div>

      <TablePagination
        pageIndex={pageIndex}
        pageSize={pageSize}
        totalCount={totalCount}
        visibleCount={doseLogs.length}
        previousLabel={t("table.previous")}
        nextLabel={t("table.next")}
        renderSummary={({ start, end, total }) =>
          t("table.pageSummary", { start, end, total })
        }
        onPageIndexChange={onPageIndexChange}
      />
    </Panel>
  );
}
