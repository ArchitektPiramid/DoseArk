import type { ReactNode } from "react";

import { cx } from "./utils";

type FormSectionProps = {
  title?: ReactNode;
  className?: string;
  children: ReactNode;
};

export function FormSection({ title, className, children }: FormSectionProps) {
  return (
    <section className={cx("mt-form-section", className)}>
      {title ? (
        <div className="mt-form-section__header">
          <div className="mt-form-section__title">{title}</div>
        </div>
      ) : null}
      {children}
    </section>
  );
}

type FormActionsProps = {
  sticky?: boolean;
  children: ReactNode;
};

export function FormActions({ sticky = false, children }: FormActionsProps) {
  return (
    <div className={sticky ? "mt-form-actions mt-form-actions--sticky" : "mt-form-actions"}>
      {children}
    </div>
  );
}

type RequiredLabelProps = {
  label: ReactNode;
  invalid?: boolean;
};

export function RequiredLabel({ label, invalid = false }: RequiredLabelProps) {
  return (
    <span className={invalid ? "mt-required-label is-invalid" : "mt-required-label"}>
      <span>{label}</span>
      <span className="mt-required-label__mark" aria-hidden="true">*</span>
    </span>
  );
}
