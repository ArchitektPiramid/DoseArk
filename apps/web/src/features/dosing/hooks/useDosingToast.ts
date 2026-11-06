import { useCallback, useEffect, useState } from "react";

import type { DosingToastMessage } from "../components/DosingToast";

const TOAST_DURATION_MS = 3_200;

export function useDosingToast() {
  const [toast, setToast] = useState<DosingToastMessage | null>(null);

  const showToast = useCallback(
    (variant: DosingToastMessage["variant"], message: string) => {
      setToast({ id: Date.now(), variant, message });
    },
    [],
  );

  const dismissToast = useCallback(() => {
    setToast(null);
  }, []);

  useEffect(() => {
    if (!toast) {
      return;
    }

    const timeoutId = window.setTimeout(dismissToast, TOAST_DURATION_MS);
    return () => window.clearTimeout(timeoutId);
  }, [dismissToast, toast]);

  return {
    toast,
    showToast,
    dismissToast,
  };
}
