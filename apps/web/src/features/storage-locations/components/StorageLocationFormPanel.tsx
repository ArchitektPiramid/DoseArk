import type {
  CreateStorageLocationInput,
  StorageLocation,
  UpdateStorageLocationInput,
} from "@doseark/types";
import { useEffect, useState } from "react";

import { Button, Field, FormActions, Panel, SelectInput, TextArea, TextInput } from "@/core/ui";
import {
  VisualColorSelect,
  VisualIcon,
  VisualIconSelect,
} from "@/core/visualTags";

type Props = {
  selected?: StorageLocation;
  disabled?: boolean;
  onCreate: (input: CreateStorageLocationInput) => void;
  onUpdate: (id: string, input: UpdateStorageLocationInput) => void;
  onClear: () => void;
};

export function StorageLocationFormPanel({
  selected,
  disabled,
  onCreate,
  onUpdate,
  onClear,
}: Props) {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [color, setColor] = useState("#6f91b8");
  const [icon, setIcon] = useState("box");
  const [sortOrder, setSortOrder] = useState("0");
  const [isArchived, setIsArchived] = useState(false);

  const isEdit = selected !== undefined;

  useEffect(() => {
    if (!selected) {
      setName("");
      setDescription("");
      setColor("#6f91b8");
      setIcon("box");
      setSortOrder("0");
      setIsArchived(false);
      return;
    }

    setName(selected.name);
    setDescription(selected.description ?? "");
    setColor(selected.color ?? "#6f91b8");
    setIcon(selected.icon ?? "box");
    setSortOrder(String(selected.sort_order));
    setIsArchived(selected.is_archived);
  }, [selected]);

  function submit() {
    const parsedSortOrder = Number(sortOrder);
    const normalizedSortOrder = Number.isFinite(parsedSortOrder) ? parsedSortOrder : 0;

    if (isEdit) {
      onUpdate(selected.id, {
        name: name.trim(),
        description: description.trim() || null,
        color,
        icon,
        sort_order: normalizedSortOrder,
        is_archived: isArchived,
      });
      return;
    }

    onCreate({
      name: name.trim(),
      description: description.trim() || null,
      color,
      icon,
      sort_order: normalizedSortOrder,
    });
  }

  return (
    <Panel title={isEdit ? "Edit storage location" : "Create storage location"}>
      <div className="mt-form">
        <Field label="Name">
          <TextInput value={name} onChange={(event) => setName(event.target.value)} />
        </Field>

        <Field label="Description">
          <TextArea
            value={description}
            onChange={(event) => setDescription(event.target.value)}
          />
        </Field>

        <div className="mt-form-grid mt-form-grid--3">
          <Field label="Icon">
            <VisualIconSelect value={icon} onChange={setIcon} />
          </Field>

          <Field label="Color">
            <VisualColorSelect value={color} onChange={setColor} />
          </Field>

          <Field label="Sort order">
            <TextInput
              type="number"
              value={sortOrder}
              onChange={(event) => setSortOrder(event.target.value)}
            />
          </Field>
        </div>

        <div className="mt-location-preview">
          <div className="mt-location-preview__icon" style={{ color }}>
            <VisualIcon name={icon} />
          </div>

          <div className="mt-min-w-0">
            <div className="mt-location-preview__title">{name || "Location name"}</div>
            <div className="mt-location-preview__description">
              {description || "Description"}
            </div>
          </div>
        </div>

        {isEdit ? (
          <Field label="Archived">
            <SelectInput
              value={isArchived ? "true" : "false"}
              onChange={(event) => setIsArchived(event.target.value === "true")}
            >
              <option value="false">No</option>
              <option value="true">Yes</option>
            </SelectInput>
          </Field>
        ) : null}

        <FormActions>
          <Button variant="quiet" onClick={onClear}>
            Clear
          </Button>

          <Button
            variant="primary"
            disabled={disabled || name.trim().length === 0}
            onClick={submit}
          >
            {isEdit ? "Save location" : "Create location"}
          </Button>
        </FormActions>
      </div>
    </Panel>
  );
}
