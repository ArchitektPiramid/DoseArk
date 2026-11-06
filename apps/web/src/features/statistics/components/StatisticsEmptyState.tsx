import { CalendarRange, LoaderCircle } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Button } from "@/core/ui";

type Props = {
  loading: boolean;
  onExtendRange?: () => void;
  description?: string;
};

/** Replaces several empty chart placeholders with one actionable explanation. */
export function StatisticsEmptyState({ loading, onExtendRange, description }: Props) {
  const { t } = useTranslation("statistics");

  return (
    <div className="mt-statistics-empty-state" role="status" aria-live="polite">
      {loading ? <LoaderCircle size={28} className="mt-statistics-empty-state__loading" aria-hidden="true" /> : <CalendarRange size={30} aria-hidden="true" />}
      <strong>{loading ? t("state.loading") : t("layout.emptyTitle")}</strong>
      {!loading ? (
        <>
          <p>{description ?? t("layout.emptyDescription")}</p>
          {onExtendRange ? <Button type="button" variant="primary" onClick={onExtendRange}>{t("layout.showLastYear")}</Button> : null}
        </>
      ) : null}
    </div>
  );
}
