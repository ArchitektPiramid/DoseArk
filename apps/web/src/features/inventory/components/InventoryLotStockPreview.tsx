import { formatRemainingPercent, formatStockQuantity } from "@/core/format";

export function InventoryLotStockPreview({
  initial,
  remaining,
}: {
  initial: number | null;
  remaining: number | null;
}) {
  const valid =
    initial !== null &&
    initial > 0 &&
    remaining !== null &&
    remaining >= 0 &&
    remaining <= initial;
  const used = valid ? Math.max(0, initial - remaining) : null;

  return (
    <div className="mt-inventory-stock-preview" aria-label="Stock preview">
      <div className="mt-inventory-stock-preview__item">
        <span>Initial</span>
        <strong className="mt-mono">{formatStockQuantity(initial)}</strong>
      </div>
      <div className="mt-inventory-stock-preview__item">
        <span>Remaining</span>
        <strong className="mt-mono">{formatStockQuantity(remaining)}</strong>
      </div>
      <div className="mt-inventory-stock-preview__item">
        <span>Used</span>
        <strong className="mt-mono">{formatStockQuantity(used)}</strong>
      </div>
      <div className="mt-inventory-stock-preview__item">
        <span>Remaining</span>
        <strong className="mt-mono">
          {valid ? formatRemainingPercent(remaining, initial) : "-"}
        </strong>
      </div>
    </div>
  );
}
