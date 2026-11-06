import { useTranslation } from "react-i18next";

import { Panel } from "@/core/ui";

import { DoseLogForm, type DoseLogFormProps } from "./DoseLogForm";

type Props = DoseLogFormProps & {
  formRevision: number;
  keepValues: boolean;
  onKeepValuesChange: (enabled: boolean) => void;
};

/** Displays the dose form inline; the debug bulk editor keeps its drawer. */
export function DoseLogEditorPanel({
  formRevision,
  keepValues,
  onKeepValuesChange,
  ...formProps
}: Props) {
  const { t } = useTranslation("dosing");
  const editing = formProps.editingDoseLog !== undefined;

  return (
    <Panel
      className="mt-dose-editor"
      title={t(editing ? "form.editTitle" : "form.createTitle")}
      action={
        !editing ? (
          <label className="mt-dose-editor__keep-values" title={t("page.keepValuesHint")}>
            <input
              type="checkbox"
              checked={keepValues}
              disabled={formProps.disabled}
              onChange={(event) => onKeepValuesChange(event.target.checked)}
            />
            <span>{t("page.keepValues")}</span>
          </label>
        ) : null
      }
    >
      <DoseLogForm
        {...formProps}
        inline
        key={editing ? `edit-${formProps.editingDoseLog?.id}` : `create-${formRevision}`}
      />
    </Panel>
  );
}
