import type { ReactNode } from "react";

type Props = {
  title: string;
  description?: string;
  action?: ReactNode;
};

export function PageHeader({ title, description, action }: Props) {
  return (
    <header className="mt-page-header">
      <div>
        <h1 className="mt-page-header__title">{title}</h1>
        {description ? <p className="mt-page-header__description">{description}</p> : null}
      </div>

      {action ? <div className="mt-page-header__action">{action}</div> : null}
    </header>
  );
}
