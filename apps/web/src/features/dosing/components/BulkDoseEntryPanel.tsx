import type { CreateDoseLogInput, InventoryLot, Product } from "@doseark/types";
import {
  Check,
  CheckCircle2,
  Circle,
  ClipboardPaste,
  LoaderCircle,
  Trash2,
  XCircle,
} from "lucide-react";
import { useState } from "react";
import { useTranslation } from "react-i18next";

import { Button, Panel, SelectInput, TextArea } from "@/core/ui";

import { useBulkDoseEntry } from "../hooks/useBulkDoseEntry";

export type BulkDoseSaveStatus = "idle" | "saving" | "success" | "error";

type Props = {
  lots: InventoryLot[];
  products: Product[];
  productById: Map<string, Product>;
  productNameById: Map<string, string>;
  disabled: boolean;
  onSave: (
    inputs: CreateDoseLogInput[],
    onProgress: (index: number, status: Exclude<BulkDoseSaveStatus, "idle">) => void,
  ) => Promise<boolean>;
};

export function BulkDoseEntryPanel({
  lots,
  products,
  productById,
  productNameById,
  disabled,
  onSave,
}: Props) {
  const { t } = useTranslation("dosing");
  const entry = useBulkDoseEntry({ lots, products, productById });
  const [submitting, setSubmitting] = useState(false);
  const [saveStatusByKey, setSaveStatusByKey] = useState<Record<string, BulkDoseSaveStatus>>({});

  function resetSaveStatuses() {
    setSaveStatusByKey({});
  }

  async function saveBatch() {
    const batch = entry.buildBatchItems();
    if (batch.length === 0) return;

    const keys = batch.map((item) => item.key);
    setSaveStatusByKey(Object.fromEntries(keys.map((key) => [key, "idle"])));
    setSubmitting(true);

    try {
      await onSave(
        batch.map((item) => item.input),
        (index, status) => {
          const key = keys[index];
          if (!key) return;
          setSaveStatusByKey((current) => ({ ...current, [key]: status }));
        },
      );
    } finally {
      setSubmitting(false);
    }
  }

  function changeProduct(productId: string) {
    resetSaveStatuses();
    entry.changeProduct(productId);
  }

  function changeLot(lotId: string) {
    resetSaveStatuses();
    entry.changeLot(lotId);
  }

  function changeAmountMode(mode: "mg" | "units") {
    resetSaveStatuses();
    entry.setDefaultAmountMode(mode);
  }

  function changeSourceText(value: string) {
    resetSaveStatuses();
    entry.setSourceText(value);
  }

  function clearSourceText() {
    resetSaveStatuses();
    entry.clearSourceText();
  }

  const isBusy = disabled || submitting;
  const selectedUnit = entry.selectedProduct?.serving_unit ?? "unit";
  const remainingAfter = entry.selectedLot
    ? Math.max(0, entry.selectedLot.units_remaining - entry.totalUnits)
    : null;
  const hasSavedRows = entry.previewEntries.some((row) => saveStatusByKey[row.key] === "success");
  const allSaved =
    entry.previewEntries.length > 0 &&
    entry.previewEntries.every((row) => saveStatusByKey[row.key] === "success");

  return (
    <Panel title={t("bulk.panelTitle")} className="mt-bulk-dose-entry-panel">
      <div className="mt-bulk-dose-entry">
        <div className="mt-bulk-dose-selection">
          <label className="mt-bulk-dose-field">
            <span className="mt-label">{t("bulk.product")}</span>
            <SelectInput
              value={entry.productId}
              disabled={isBusy}
              onChange={(event) => changeProduct(event.target.value)}
            >
              <option value="">{t("bulk.selectProduct")}</option>
              {products.map((product) => (
                <option key={product.id} value={product.id}>
                  {productNameById.get(product.id) ?? product.name}
                </option>
              ))}
            </SelectInput>
          </label>

          <label className="mt-bulk-dose-field">
            <span className="mt-label">{t("bulk.lot")}</span>
            <SelectInput
              value={entry.lotId}
              disabled={isBusy || !entry.productId}
              onChange={(event) => changeLot(event.target.value)}
            >
              <option value="">{t("bulk.selectLot")}</option>
              {entry.filteredLots.map((lot) => (
                <option key={lot.id} value={lot.id}>
                  {(lot.lot_number?.trim() || "—") + ` · ${formatNumber(lot.units_remaining)} left`}
                </option>
              ))}
            </SelectInput>
          </label>
        </div>

        <div className="mt-bulk-dose-mode" role="group" aria-label={t("bulk.defaultAmountMode")}>
          <span className="mt-label">{t("bulk.defaultAmountMode")}</span>
          <div className="mt-bulk-dose-mode__buttons">
            <button
              type="button"
              className={entry.defaultAmountMode === "mg" ? "is-active" : ""}
              disabled={isBusy || !entry.selectedProduct?.unit_weight_mg}
              onClick={() => changeAmountMode("mg")}
            >
              mg
            </button>
            <button
              type="button"
              className={entry.defaultAmountMode === "units" ? "is-active" : ""}
              disabled={isBusy}
              onClick={() => changeAmountMode("units")}
            >
              {t("bulk.units")}
            </button>
          </div>
          {entry.selectedProduct?.unit_weight_mg ? (
            <span className="mt-bulk-dose-hint">
              {t("bulk.unitWeight", { value: formatNumber(entry.selectedProduct.unit_weight_mg) })}
            </span>
          ) : entry.productId ? (
            <span className="mt-bulk-dose-hint mt-bulk-dose-hint--warning">
              {t("bulk.noUnitWeight")}
            </span>
          ) : null}
        </div>

        <label className="mt-bulk-dose-field">
          <span className="mt-label mt-bulk-dose-field__label-with-icon">
            <ClipboardPaste size={14} aria-hidden="true" />
            {t("bulk.pasteInput")}
          </span>
          <TextArea
            className="mt-bulk-dose-source"
            value={entry.sourceText}
            disabled={isBusy}
            spellCheck={false}
            placeholder={t("bulk.placeholder")}
            onChange={(event) => changeSourceText(event.target.value)}
          />
        </label>

        {entry.parsed.errors.length > 0 ? (
          <div className="mt-bulk-dose-errors" role="alert">
            {entry.parsed.errors.map((error) => (
              <div key={`${error.line}-${error.message}`}>
                {t("bulk.lineError", { line: error.line, message: error.message })}
              </div>
            ))}
          </div>
        ) : null}

        {entry.conversionErrors.length > 0 ? (
          <div className="mt-bulk-dose-errors" role="alert">
            {entry.conversionErrors.map((row) => (
              <div key={row.key}>
                {t("bulk.lineError", { line: row.line, message: row.error })}
              </div>
            ))}
          </div>
        ) : null}

        <div className="mt-bulk-dose-preview-header">
          <span>{t("bulk.preview")}</span>
          <span className="mt-mono">{entry.previewEntries.length}</span>
        </div>

        <div className="mt-bulk-dose-preview-list">
          {entry.previewEntries.length === 0 ? (
            <div className="mt-bulk-dose-empty">{t("bulk.previewEmpty")}</div>
          ) : (
            entry.previewEntries.map((row) => (
              <BulkDosePreviewCard
                key={row.key}
                row={row}
                selectedUnit={selectedUnit}
                saveStatus={saveStatusByKey[row.key] ?? "idle"}
              />
            ))
          )}
        </div>

        <div className="mt-bulk-dose-summary">
          <div>
            <span>{t("bulk.entries")}</span>
            <strong className="mt-mono">{entry.previewEntries.length}</strong>
          </div>
          <div>
            <span>{t("bulk.total")}</span>
            <strong className="mt-mono">
              {formatNumber(entry.totalUnits)} {selectedUnit}
            </strong>
          </div>
          <div>
            <span>{t("bulk.stock")}</span>
            <strong className="mt-mono">
              {entry.selectedLot
                ? `${formatNumber(entry.selectedLot.units_remaining)} → ${formatNumber(remainingAfter ?? 0)}`
                : "—"}
            </strong>
          </div>
        </div>

        {entry.stockExceeded ? (
          <div className="mt-bulk-dose-errors" role="alert">
            {t("bulk.stockExceeded")}
          </div>
        ) : null}

        <div className="mt-bulk-dose-actions">
          <Button
            type="button"
            variant="quiet"
            disabled={isBusy || !entry.sourceText}
            onClick={clearSourceText}
          >
            <Trash2 size={15} />
            {t("bulk.clear")}
          </Button>

          <Button
            type="button"
            variant="success"
            disabled={isBusy || !entry.canSave || hasSavedRows}
            onClick={() => void saveBatch()}
          >
            <Check size={16} />
            {isBusy
              ? t("bulk.saving")
              : allSaved
                ? t("bulk.saved")
                : t("bulk.save", { count: entry.previewEntries.length })}
          </Button>
        </div>

        <p className="mt-bulk-dose-footnote">{t("bulk.saveHint")}</p>
      </div>
    </Panel>
  );
}

type PreviewCardProps = {
  row: ReturnType<typeof useBulkDoseEntry>["previewEntries"][number];
  selectedUnit: string;
  saveStatus: BulkDoseSaveStatus;
};

function BulkDosePreviewCard({ row, selectedUnit, saveStatus }: PreviewCardProps) {
  const { t } = useTranslation("dosing");
  const parsedOk = row.error === null;

  return (
    <div className={row.error ? "mt-bulk-dose-card is-invalid" : "mt-bulk-dose-card"}>
      <StatusIcon
        status={parsedOk ? "success" : "error"}
        title={parsedOk ? t("bulk.parsedOk") : t("bulk.parsedError")}
      />
      <div className="mt-bulk-dose-card__date mt-mono">{formatDate(row.date)}</div>
      <div className="mt-bulk-dose-card__time mt-mono">{row.time}</div>
      <div className="mt-bulk-dose-card__amount">
        <strong className="mt-mono">
          {formatNumber(row.amount)} {row.effectiveMode === "mg" ? "mg" : selectedUnit}
        </strong>
        {row.unitsTaken !== null && row.effectiveMode === "mg" ? (
          <span className="mt-mono">
            → {formatNumber(row.unitsTaken)} {selectedUnit}
          </span>
        ) : null}
      </div>
      <StatusIcon
        status={saveStatus}
        title={
          saveStatus === "success"
            ? t("bulk.savedOk")
            : saveStatus === "error"
              ? t("bulk.savedError")
              : saveStatus === "saving"
                ? t("bulk.savingRow")
                : t("bulk.notSaved")
        }
      />
    </div>
  );
}

function StatusIcon({ status, title }: { status: BulkDoseSaveStatus; title: string }) {
  const className = `mt-bulk-dose-card__status is-${status}`;

  if (status === "success") {
    return <span title={title}><CheckCircle2 className={className} size={16} aria-label={title} /></span>;
  }
  if (status === "error") {
    return <span title={title}><XCircle className={className} size={16} aria-label={title} /></span>;
  }
  if (status === "saving") {
    return (
      <span title={title}><LoaderCircle className={`${className} is-spinning`} size={16} aria-label={title} /></span>
    );
  }
  return <span title={title}><Circle className={className} size={15} aria-label={title} /></span>;
}

function formatNumber(value: number): string {
  if (!Number.isFinite(value)) return "—";
  return Number(value.toFixed(4)).toString();
}

function formatDate(value: string): string {
  const [year, month, day] = value.split("-");
  return `${year}.${month}.${day}`;
}
