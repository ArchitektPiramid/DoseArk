import type { ButtonHTMLAttributes } from "react";

import { cx } from "./utils";

export type ButtonVariant = "default" | "primary" | "success" | "warning" | "danger" | "quiet";

type Props = ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: ButtonVariant;
};

export function Button({ variant = "default", className, ...props }: Props) {
  const variantClass =
    variant === "primary"
      ? "mt-button--primary"
      : variant === "success"
        ? "mt-button--success"
        : variant === "warning"
          ? "mt-button--warning"
          : variant === "danger"
            ? "mt-button--danger"
            : variant === "quiet"
              ? "mt-button--quiet"
              : "";

  return <button {...props} className={cx("mt-button", variantClass, className)} />;
}
