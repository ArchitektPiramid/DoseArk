import {
  formatNumber,
  formatPercentFromRatio,
  formatSignedNumber,
  normalizeNumber,
} from "./number";

export function formatStockQuantity(value: number | null | undefined): string {
  return formatNumber(value, {
    maxDecimals: 2,
    fallback: "-",
  });
}

export function formatStockDelta(value: number | null | undefined): string {
  return formatSignedNumber(value, {
    maxDecimals: 2,
    fallback: "-",
  });
}

export function formatStockFraction(
  remaining: number | null | undefined,
  initial: number | null | undefined,
): string {
  return `${formatStockQuantity(remaining)} / ${formatStockQuantity(initial)}`;
}

export function formatRemainingPercent(
  remaining: number | null | undefined,
  initial: number | null | undefined,
): string {
  if (
    remaining === null ||
    remaining === undefined ||
    initial === null ||
    initial === undefined ||
    !Number.isFinite(remaining) ||
    !Number.isFinite(initial) ||
    initial <= 0
  ) {
    return "-";
  }

  return formatPercentFromRatio(normalizeNumber(remaining) / normalizeNumber(initial), {
    maxDecimals: 1,
  });
}

export function formatMoney(
  amount: number | null | undefined,
  currency: string | null | undefined,
): string {
  if (amount === null || amount === undefined || !Number.isFinite(amount)) {
    return "-";
  }

  if (!currency) {
    return formatNumber(amount, { maxDecimals: 2 });
  }

  return `${formatNumber(amount, { maxDecimals: 2 })} ${currency}`;
}
