import type { ReactNode } from "react";

type Props = {
  checked: boolean;
  label: ReactNode;
  count?: number;
  onChange: () => void;
};

export function TableFilterToggle({ checked, label, count, onChange }: Props) {
  return (
    <label className="mt-table-filter">
      <input type="checkbox" checked={checked} onChange={onChange} />
      <span className="mt-table-filter__box" aria-hidden="true" />
      <span className="mt-table-filter__label">{label}</span>
      {count === undefined ? null : <span className="mt-table-filter__count mt-mono">{count}</span>}
    </label>
  );
}
