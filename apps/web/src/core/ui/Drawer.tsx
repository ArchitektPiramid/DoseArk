import { X } from "lucide-react";
import { useEffect, useId } from "react";
import type { ReactNode } from "react";
import { createPortal } from "react-dom";

import { Button } from "./Button";
import { cx } from "./utils";

export type DrawerSize = "compact" | "default";

type Props = {
  open: boolean;
  title: ReactNode;
  description?: ReactNode;
  tabs?: ReactNode;
  children: ReactNode;
  size?: DrawerSize;
  closeLabel: string;
  closeDisabled?: boolean;
  className?: string;
  onClose: () => void;
};

export function Drawer({
  open,
  title,
  description,
  tabs,
  children,
  size = "default",
  closeLabel,
  closeDisabled = false,
  className,
  onClose,
}: Props) {
  const titleId = useId();

  useEffect(() => {
    if (!open) return;

    const previousOverflow = document.body.style.overflow;
    document.body.style.overflow = "hidden";

    function handleKeyDown(event: KeyboardEvent) {
      if (event.key === "Escape" && !closeDisabled) onClose();
    }

    window.addEventListener("keydown", handleKeyDown);

    return () => {
      document.body.style.overflow = previousOverflow;
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [closeDisabled, onClose, open]);

  if (!open) return null;

  return createPortal(
    <div className="mt-drawer-layer">
      <button
        type="button"
        className="mt-drawer-backdrop"
        aria-label={closeLabel}
        aria-disabled={closeDisabled || undefined}
        onClick={() => {
          if (!closeDisabled) onClose();
        }}
      />

      <aside
        className={cx("mt-drawer", `mt-drawer--${size}`, className)}
        role="dialog"
        aria-modal="true"
        aria-labelledby={titleId}
      >
        <header className="mt-drawer__header">
          <div className="mt-drawer__heading">
            <h2 id={titleId} className="mt-drawer__title">{title}</h2>
            {description ? <p className="mt-drawer__description">{description}</p> : null}
          </div>

          <Button
            type="button"
            variant="quiet"
            className="mt-drawer__close"
            aria-label={closeLabel}
            disabled={closeDisabled}
            onClick={onClose}
          >
            <X size={18} />
          </Button>
        </header>

        {tabs ? <div className="mt-drawer__tabs">{tabs}</div> : null}
        <div className="mt-drawer__body">{children}</div>
      </aside>
    </div>,
    document.body,
  );
}
