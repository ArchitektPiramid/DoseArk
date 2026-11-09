import type { NutrientTrendOption, Product } from "@doseark/types";
import { useTranslation } from "react-i18next";

import type { StatisticsSource } from "../hooks/useStatistics";
import type { StatisticsDateRange } from "../utils/statisticsRange";
import { StatisticsPeriodControls } from "./StatisticsPeriodControls";
import { StatisticsSubjectControls } from "./StatisticsSubjectControls";

export type StatisticsToolbarProps = {
  range: StatisticsDateRange;
  source: StatisticsSource;
  products: Product[];
  nutrients: NutrientTrendOption[];
  selectedProductId: string | null;
  selectedNutrientKey: string | null;
  loading: boolean;
  showSubject?: boolean;
  showRefresh?: boolean;
  showRange?: boolean;
  onRangeChange: (range: StatisticsDateRange) => void;
  onSourceChange: (source: StatisticsSource) => void;
  onProductChange: (productId: string) => void;
  onNutrientChange: (key: string) => void;
  onRefresh: () => void;
};

/** Composes independent subject and date controls without owning their state. */
export function StatisticsToolbar(props: StatisticsToolbarProps) {
  const { t } = useTranslation("statistics");

  return (
    <div className="mt-statistics-toolbar">
      {props.showSubject !== false ? (
        <section className="mt-statistics-toolbar__section" aria-label={t("layout.subjectTitle")}>
          <div className="mt-statistics-toolbar__heading">
            <strong>{t("layout.subjectTitle")}</strong>
            <span>{t("layout.subjectHint")}</span>
          </div>
          <StatisticsSubjectControls {...props} />
        </section>
      ) : null}
      <StatisticsPeriodControls {...props} />
    </div>
  );
}
