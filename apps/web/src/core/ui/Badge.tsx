import type { ReactNode } from "react";

import { cx } from "./utils";

export type BadgeTone = "default" | "ok" | "warn" | "danger";

type Props = {
  tone?: BadgeTone;
  children: ReactNode;
};

export function Badge({ tone = "default", children }: Props) {
  const toneClass =
    tone === "ok"
      ? "mt-badge--ok"
      : tone === "warn"
        ? "mt-badge--warn"
        : tone === "danger"
          ? "mt-badge--danger"
          : "";

  return <span className={cx("mt-badge", toneClass)}>{children}</span>;
}
