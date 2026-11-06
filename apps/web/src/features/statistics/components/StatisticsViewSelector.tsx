import { Activity, BarChart3, CalendarDays, GitCompareArrows, TrendingUp } from "lucide-react";
import { useTranslation } from "react-i18next";

export type StatisticsView = "charts" | "comparison" | "heatmap" | "rollingAverage" | "halfLife";

type Props = {
  view: StatisticsView;
  onChange: (view: StatisticsView) => void;
};

const VIEWS = [
  { key: "charts", Icon: BarChart3 },
  { key: "comparison", Icon: GitCompareArrows },
  { key: "heatmap", Icon: CalendarDays },
  { key: "rollingAverage", Icon: TrendingUp },
  { key: "halfLife", Icon: Activity },
] as const;

/** Presents each analysis type as an explicit choice with a short explanation. */
export function StatisticsViewSelector({ view, onChange }: Props) {
  const { t } = useTranslation("statistics");

  return (
    <section className="mt-statistics-analysis-picker" aria-label={t("view.label")}>
      <div className="mt-statistics-analysis-picker__heading">
        <h2>{t("layout.analysisTitle")}</h2>
        <p>{t("layout.analysisHint")}</p>
      </div>
      <div className="mt-statistics-analysis-grid" role="group" aria-label={t("view.label")}>
        {VIEWS.map(({ key, Icon }) => (
          <button key={key} type="button"
            className={`mt-statistics-analysis-option${view === key ? " is-active" : ""}`}
            aria-pressed={view === key} onClick={() => onChange(key)}>
            <span className="mt-statistics-analysis-option__title">
              <Icon size={17} aria-hidden="true" />
              <strong>{t(`view.${key}`)}</strong>
            </span>
            <span className="mt-statistics-analysis-option__description">{t(`layout.viewHints.${key}`)}</span>
          </button>
        ))}
      </div>
    </section>
  );
}
