import { AlertCircle, CheckCircle2, X } from "lucide-react";

export type DosingToastMessage = {
  id: number;
  variant: "success" | "error";
  message: string;
};

type Props = {
  toast: DosingToastMessage | null;
  onDismiss: () => void;
};

export function DosingToast({ toast, onDismiss }: Props) {
  if (!toast) {
    return null;
  }

  const Icon = toast.variant === "success" ? CheckCircle2 : AlertCircle;

  return (
    <div className={`mt-dose-toast mt-dose-toast--${toast.variant}`} role="status" aria-live="polite">
      <Icon size={19} aria-hidden="true" />
      <span>{toast.message}</span>
      <button type="button" className="mt-dose-toast__close" onClick={onDismiss} aria-label="Close">
        <X size={16} aria-hidden="true" />
      </button>
    </div>
  );
}
