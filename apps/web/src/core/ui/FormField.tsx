import type { ReactNode } from "react";
import { AlertTriangle, CheckCircle2, Info } from "lucide-react";

export type FormFieldState = "neutral" | "ok" | "warning" | "error";

type Props = {
  label: string;
  state?: FormFieldState;
  message?: string;
  children: ReactNode;
};

export function FormField({ label, state = "neutral", message, children }: Props) {
  const stateClass = state === "neutral" ? "" : `mt-control-state--${state}`;

  return (
    <label className={`mt-field ${stateClass}`}>
      <div className="mt-field__label-row">
        <span className="mt-label">{label}</span>

        {state !== "neutral" ? (
          <span className={`mt-field__hint mt-field__hint--${state}`}>
            <FieldStateIcon state={state} />
          </span>
        ) : null}
      </div>

      {children}

      {message ? (
        <div className={`mt-field__message mt-field__message--${state}`}>{message}</div>
      ) : null}
    </label>
  );
}

function FieldStateIcon({ state }: { state: FormFieldState }) {
  if (state === "ok") return <CheckCircle2 size={14} />;
  if (state === "warning" || state === "error") return <AlertTriangle size={14} />;

  return <Info size={14} />;
}
