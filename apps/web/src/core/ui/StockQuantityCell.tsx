import { formatStockQuantity } from "@/core/format";

type StockLevel = "normal" | "warning" | "danger";

type Props = {
  remaining: number | null | undefined;
  initial: number | null | undefined;
  usedLabel?: string;
  showUsedLabel?: boolean;
  showBar?: boolean;
  title?: string;
  className?: string;
};

function getRemainingPercent(
  remaining: number | null | undefined,
  initial: number | null | undefined,
): number | null {
  if (
    remaining === null ||
    remaining === undefined ||
    initial === null ||
    initial === undefined ||
    !Number.isFinite(remaining) ||
    !Number.isFinite(initial) ||
    initial <= 0
  ) {
    return null;
  }

  return (remaining / initial) * 100;
}

function getStockLevel(percent: number | null): StockLevel {
  if (percent !== null && percent < 5) return "danger";
  if (percent !== null && percent < 15) return "warning";
  return "normal";
}

function getBarWidth(percent: number | null): number {
  if (percent === null) return 0;
  const clamped = Math.min(Math.max(percent, 0), 100);
  return clamped > 0 ? Math.max(clamped, 2) : 0;
}

export function StockQuantityCell({
  remaining,
  initial,
  usedLabel = "used",
  showUsedLabel = true,
  showBar = true,
  title,
  className,
}: Props) {
  const used =
    typeof remaining === "number" && typeof initial === "number"
      ? initial - remaining
      : null;
  const percent = getRemainingPercent(remaining, initial);
  const level = getStockLevel(percent);
  const widthPercent = getBarWidth(percent);

  return (
    <div className={["mt-stock-quantity", className].filter(Boolean).join(" ")} title={title}>
      <div className="mt-stock-quantity__value mt-mono">
        <span>{formatStockQuantity(remaining)}</span>
        <span className="mt-stock-quantity__separator">/</span>
        <span>{formatStockQuantity(initial)}</span>
        {used !== null ? (
          <span className="mt-stock-quantity__used">
            ({showUsedLabel ? `${usedLabel} ` : ""}{formatStockQuantity(used)})
          </span>
        ) : null}
      </div>

      {showBar ? (
        <div className={`mt-stock-quantity__bar mt-stock-quantity__bar--${level}`} aria-hidden="true">
          <span className="mt-stock-quantity__fill" style={{ width: `${widthPercent}%` }} />
        </div>
      ) : null}
    </div>
  );
}
