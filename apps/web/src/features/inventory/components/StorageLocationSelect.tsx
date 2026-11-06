import type { StorageLocation } from "@doseark/types";
import { useMemo } from "react";

import { Listbox, type ListboxOption } from "@/core/ui";
import { VisualIcon } from "@/core/visualTags";

type Props = {
  value: string;
  locations: StorageLocation[];
  placeholder?: string;
  onChange: (locationId: string) => void;
};

export function StorageLocationSelect({
  value,
  locations,
  placeholder = "No location",
  onChange,
}: Props) {
  const options = useMemo<Array<ListboxOption<string>>>(() => {
    return locations
      .filter((location) => !location.is_archived || location.id === value)
      .map((location) => ({
        value: location.id,
        label: location.name,
        render: (
          <div className="mt-inventory-storage-option">
            <div
              className="mt-inventory-storage-option__icon"
              style={{ color: location.color ?? undefined }}
            >
              <VisualIcon name={location.icon ?? "box"} />
            </div>

            <div className="mt-inventory-storage-option__main">
              <div className="mt-inventory-storage-option__title">{location.name}</div>
              {location.is_archived ? (
                <div className="mt-inventory-storage-option__meta">archived</div>
              ) : null}
            </div>
          </div>
        ),
      }));
  }, [locations, value]);

  return (
    <Listbox
      value={value}
      placeholder={placeholder}
      options={options}
      onChange={onChange}
    />
  );
}
