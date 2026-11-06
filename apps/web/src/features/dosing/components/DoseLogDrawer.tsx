import type { CreateDoseLogInput, UpdateDoseLogInput } from "@doseark/types";
import { useTranslation } from "react-i18next";

import { Drawer } from "@/core/ui";

import { DoseLogForm, type DoseLogFormProps } from "./DoseLogForm";

type Props = Omit<DoseLogFormProps, "onClose" | "onCreate" | "onUpdate"> & {
  open: boolean;
  onClose: () => void;
  onCreate: (input: CreateDoseLogInput) => void | Promise<void>;
  onUpdate: (id: string, input: UpdateDoseLogInput) => void | Promise<void>;
};

export function DoseLogDrawer({ open, onClose, ...formProps }: Props) {
  const { t } = useTranslation("dosing");
  const isEditMode = formProps.editingDoseLog !== undefined;

  return (
    <Drawer
      open={open}
      size="compact"
      title={isEditMode ? t("form.editTitle") : t("drawer.title")}
      description={isEditMode ? t("drawer.editDescription") : t("drawer.description")}
      closeLabel={t("drawer.close")}
      closeDisabled={formProps.disabled}
      onClose={onClose}
    >
      <DoseLogForm
        {...formProps}
        onCreate={formProps.onCreate}
        onUpdate={formProps.onUpdate}
        onClose={onClose}
      />
    </Drawer>
  );
}
