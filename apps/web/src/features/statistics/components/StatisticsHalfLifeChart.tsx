import { useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import type { NutrientTrendEvent } from "@doseark/types";

import {
  buildHalfLifeChartData,
  calculatePixelBucketHours,
  pathForHalfLife,
} from "../utils/halfLifeChart";

type Props = {
  events: NutrientTrendEvent[];
  unit: string;
  startDate: string;
  endDate: string;
  halfLifeHours: number;
  barWidthPixels: number;
  movingAverageDays: number;
  lineWidth: number;
  showBars: boolean;
  showLine: boolean;
  showAverage: boolean;
};

const WIDTH = 1400;
const HEIGHT = 430;
const PAD = { left: 64, right: 64, top: 44, bottom: 48 };

export function StatisticsHalfLifeChart({
  events,
  unit,
  startDate,
  endDate,
  halfLifeHours,
  barWidthPixels,
  movingAverageDays,
  lineWidth,
  showBars,
  showLine,
  showAverage,
}: Props) {
  const { t, i18n } = useTranslation("statistics");
  const svgRef = useRef<SVGSVGElement>(null);
  const [renderedWidth, setRenderedWidth] = useState(WIDTH);
  const innerW = WIDTH - PAD.left - PAD.right;
  const innerH = HEIGHT - PAD.top - PAD.bottom;

  useEffect(() => {
    const svg = svgRef.current;
    if (!svg) return;

    const updateWidth = () => {
      const width = svg.getBoundingClientRect().width;
      if (width > 0) setRenderedWidth(width);
    };

    updateWidth();
    if (typeof ResizeObserver === "undefined") return;

    const observer = new ResizeObserver(updateWidth);
    observer.observe(svg);
    return () => observer.disconnect();
  }, []);

  const bucketHours = useMemo(
    () => calculatePixelBucketHours(startDate, endDate, renderedWidth, WIDTH, innerW, barWidthPixels),
    [barWidthPixels, endDate, innerW, renderedWidth, startDate],
  );

  const data = useMemo(
    () => buildHalfLifeChartData(events, halfLifeHours, bucketHours, movingAverageDays, startDate, endDate),
    [bucketHours, endDate, events, halfLifeHours, movingAverageDays, startDate],
  );

  const number = useMemo(
    () => new Intl.NumberFormat(i18n.language, { maximumFractionDigits: 1 }),
    [i18n.language],
  );
  const windowNumber = useMemo(
    () => new Intl.NumberFormat(i18n.language, { maximumFractionDigits: bucketHours < 1 ? 2 : 1 }),
    [bucketHours, i18n.language],
  );
  const date = useMemo(
    () => new Intl.DateTimeFormat(i18n.language, { day: "2-digit", month: "short" }),
    [i18n.language],
  );
  const dateTime = useMemo(
    () => new Intl.DateTimeFormat(i18n.language, { day: "2-digit", month: "short", hour: "2-digit", minute: "2-digit" }),
    [i18n.language],
  );

  if (data.curve.length < 2) {
    return <div className="mt-statistics-chart-empty">{t("state.noChartData")}</div>;
  }

  const x = (value: number) => PAD.left + ((value - data.start) / Math.max(1, data.end - data.start)) * innerW;
  const yLine = (value: number) => PAD.top + innerH - (value / Math.max(data.lineMax, 1e-9)) * innerH;
  const yBar = (value: number) => PAD.top + innerH - (value / Math.max(data.barMax, 1e-9)) * innerH;
  const curvePath = pathForHalfLife(data.curve, x, yLine);
  const averagePath = pathForHalfLife(data.movingAverage, x, yLine);
  const yTicks = Array.from({ length: 5 }, (_, index) => (data.lineMax * index) / 4);
  const barTicks = Array.from({ length: 5 }, (_, index) => (data.barMax * index) / 4);
  const xTicks = Array.from({ length: 8 }, (_, index) => data.start + ((data.end - data.start) * index) / 7);
  const barWidth = Math.max(0.5, (Math.max(1, Math.round(barWidthPixels)) * WIDTH) / Math.max(1, renderedWidth));
  const bucketHalfMs = bucketHours * 1_800_000;
  const safeLineWidth = Math.max(0.5, Math.min(8, lineWidth));

  return (
    <div className="mt-statistics-half-life-wrap">
      <div className="mt-statistics-half-life__legend" aria-hidden="true">
        <span><i className="mt-statistics-half-life__legend-bar" />{t("halfLife.legendDoses", { hours: windowNumber.format(bucketHours) })}</span>
        <span><i className="mt-statistics-half-life__legend-line" style={{ borderTopWidth: safeLineWidth }} />{t("halfLife.legendRemaining", { hours: halfLifeHours })}</span>
        <span><i className="mt-statistics-half-life__legend-average" style={{ borderTopWidth: safeLineWidth }} />{t("halfLife.legendAverage", { days: movingAverageDays })}</span>
      </div>
      <svg
        ref={svgRef}
        className="mt-statistics-half-life"
        viewBox={`0 0 ${WIDTH} ${HEIGHT}`}
        role="img"
        aria-label={t("halfLife.chartLabel")}
      >
        <defs>
          <clipPath id="mt-statistics-half-life-plot">
            <rect x={PAD.left} y={PAD.top} width={innerW} height={innerH} />
          </clipPath>
        </defs>

        {yTicks.map((value, index) => (
          <g key={`yl-${index}`}>
            <line x1={PAD.left} x2={WIDTH - PAD.right} y1={yLine(value)} y2={yLine(value)} className="mt-statistics-half-life__grid" />
            <text x={PAD.left - 9} y={yLine(value) + 4} textAnchor="end" className="mt-statistics-half-life__axis-left">{number.format(value)}</text>
          </g>
        ))}
        {barTicks.map((value, index) => (
          <text key={`yr-${index}`} x={WIDTH - PAD.right + 9} y={yBar(value) + 4} textAnchor="start" className="mt-statistics-half-life__axis-right">{number.format(value)}</text>
        ))}
        {xTicks.map((value, index) => (
          <g key={`x-${index}`}>
            <line x1={x(value)} x2={x(value)} y1={PAD.top} y2={HEIGHT - PAD.bottom} className="mt-statistics-half-life__grid mt-statistics-half-life__grid--vertical" />
            <text x={x(value)} y={HEIGHT - 16} textAnchor="middle">{date.format(new Date(value))}</text>
          </g>
        ))}
        <text x={16} y={PAD.top + innerH / 2} transform={`rotate(-90 16 ${PAD.top + innerH / 2})`} textAnchor="middle" className="mt-statistics-half-life__axis-title">{t("halfLife.remainingAxis", { unit })}</text>
        <text x={WIDTH - 14} y={PAD.top + innerH / 2} transform={`rotate(90 ${WIDTH - 14} ${PAD.top + innerH / 2})`} textAnchor="middle" className="mt-statistics-half-life__axis-title mt-statistics-half-life__axis-right">{t("halfLife.doseAxis", { unit })}</text>

        <g clipPath="url(#mt-statistics-half-life-plot)">
          {showBars && data.bars.map((bar) => {
            const bx = x(bar.start + bucketHalfMs);
            const top = yBar(bar.amount);
            return (
              <rect
                key={bar.start}
                x={bx - barWidth / 2}
                y={top}
                width={barWidth}
                height={PAD.top + innerH - top}
                className="mt-statistics-half-life__bar"
              >
                <title>{dateTime.format(new Date(bar.start))} · {number.format(bar.amount)} {unit} · {bar.count}×</title>
              </rect>
            );
          })}
          {showLine ? <path d={curvePath} className="mt-statistics-half-life__line" style={{ strokeWidth: safeLineWidth }} /> : null}
          {showAverage ? <path d={averagePath} className="mt-statistics-half-life__average" style={{ strokeWidth: safeLineWidth }} /> : null}
        </g>
      </svg>
    </div>
  );
}
