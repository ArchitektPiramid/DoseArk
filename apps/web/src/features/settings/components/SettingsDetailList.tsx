import type { ReactNode } from "react";

type DetailRow = {
  label: string;
  value: ReactNode;
  note?: string;
};

export function SettingsDetailList({ rows }: { rows: DetailRow[] }) {
  return (
    <dl className="mt-settings-detail-list">
      {rows.map((row) => (
        <div key={row.label} className="mt-settings-detail-row">
          <dt>
            <span>{row.label}</span>
            {row.note ? <small>{row.note}</small> : null}
          </dt>
          <dd>{row.value}</dd>
        </div>
      ))}
    </dl>
  );
}
