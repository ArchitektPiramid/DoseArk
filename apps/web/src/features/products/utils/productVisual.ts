import type { Product, ProductUserImportance } from "@doseark/types";

export function getProductVisualColor(product?: Product | null): string | undefined {
  return product?.ui_color ?? undefined;
}

export function getProductVisualIcon(product?: Product | null): string {
  return product?.ui_icon ?? "pill";
}

export function getImportanceBadgeTone(
  importance: ProductUserImportance | null | undefined,
): "default" | "ok" | "warn" {
  if (importance === "critical" || importance === "high") {
    return "warn";
  }

  if (importance === "normal") {
    return "ok";
  }

  return "default";
}
