import type { ReactNode } from "react";

type Props = {
  label: ReactNode;
  children: ReactNode;
};

export function Field({ label, children }: Props) {
  return (
    <label className="mt-field">
      <span className="mt-label">{label}</span>
      {children}
    </label>
  );
}
