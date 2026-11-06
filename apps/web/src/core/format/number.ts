export type FormatNumberOptions = {
  maxDecimals?: number;
  minDecimals?: number;
  fallback?: string;
};

export function normalizeNumber(value: number, decimals = 6): number {
  if (!Number.isFinite(value)) {
    return value;
  }

  const scale = 10 ** decimals;
  const rounded = Math.round((value + Number.EPSILON) * scale) / scale;

  if (Math.abs(rounded) < 1 / scale) {
    return 0;
  }

  return rounded;
}

export function formatNumber(
  value: number | null | undefined,
  options: FormatNumberOptions = {},
): string {
  const fallback = options.fallback ?? "-";

  if (value === null || value === undefined || !Number.isFinite(value)) {
    return fallback;
  }

  const normalized = normalizeNumber(value);
  const maxDecimals = options.maxDecimals ?? 2;
  const minDecimals = options.minDecimals ?? 0;

  return normalized.toLocaleString("en-US", {
    minimumFractionDigits: minDecimals,
    maximumFractionDigits: maxDecimals,
  });
}

export function formatSignedNumber(
  value: number | null | undefined,
  options: FormatNumberOptions = {},
): string {
  if (value === null || value === undefined || !Number.isFinite(value)) {
    return options.fallback ?? "-";
  }

  const normalized = normalizeNumber(value);
  const formatted = formatNumber(Math.abs(normalized), options);

  if (normalized > 0) {
    return `+${formatted}`;
  }

  if (normalized < 0) {
    return `-${formatted}`;
  }

  return formatted;
}

export function formatPercent(
  value: number | null | undefined,
  options: FormatNumberOptions = {},
): string {
  if (value === null || value === undefined || !Number.isFinite(value)) {
    return options.fallback ?? "-";
  }

  return `${formatNumber(value, {
    maxDecimals: options.maxDecimals ?? 1,
    minDecimals: options.minDecimals ?? 0,
  })}%`;
}

export function formatPercentFromRatio(
  value: number | null | undefined,
  options: FormatNumberOptions = {},
): string {
  if (value === null || value === undefined || !Number.isFinite(value)) {
    return options.fallback ?? "-";
  }

  return formatPercent(value * 100, options);
}

export function formatEditableNumber(value: number | null | undefined, maxDecimals = 6): string {
  if (value === null || value === undefined || !Number.isFinite(value)) {
    return "";
  }

  return formatNumber(value, {
    maxDecimals,
    minDecimals: 0,
  }).replaceAll(",", "");
}

export function parseNumberInput(value: string): number | null {
  const normalized = value.trim().replace(",", ".");

  if (normalized.length === 0) {
    return null;
  }

  const parsed = Number(normalized);

  if (!Number.isFinite(parsed)) {
    return null;
  }

  return parsed;
}
