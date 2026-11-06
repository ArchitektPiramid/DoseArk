import type { ReactNode } from "react";

import { cx } from "./utils";

type Props = {
  title?: ReactNode;
  action?: ReactNode;
  className?: string;
  children: ReactNode;
};

export function Panel({ title, action, className, children }: Props) {
  return (
    <section className={cx("mt-panel", className)}>
      {title || action ? (
        <header className="mt-panel__header">
          {title ? <div className="mt-panel__title">{title}</div> : <div />}
          {action ? <div className="mt-panel__action">{action}</div> : null}
        </header>
      ) : null}

      <div className="mt-panel__body">{children}</div>
    </section>
  );
}
