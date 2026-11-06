import { Listbox, type ListboxOption } from "@/core/ui";

import { VisualIcon } from "./VisualIcon";
import { visualIconOptions, type VisualIconName } from "./visualTagOptions";

type Props = {
  value: string;
  placeholder?: string;
  onChange: (value: string) => void;
};

export function VisualIconSelect({
  value,
  placeholder = "Select icon",
  onChange,
}: Props) {
  const options: Array<ListboxOption<VisualIconName>> = visualIconOptions.map((option) => ({
    value: option.value,
    label: option.label,
    render: (
      <div className="mt-visual-select-option">
        <span className="mt-visual-select-option__icon">
          <VisualIcon name={option.value} />
        </span>
        <span>{option.label}</span>
      </div>
    ),
  }));

  return (
    <Listbox
      value={value as VisualIconName | ""}
      placeholder={placeholder}
      options={options}
      onChange={onChange}
    />
  );
}
