import { Listbox, type ListboxOption } from "@/core/ui";

import { visualColorOptions } from "./visualTagOptions";

type Props = {
  value: string;
  placeholder?: string;
  onChange: (value: string) => void;
};

export function VisualColorSelect({
  value,
  placeholder = "Select color",
  onChange,
}: Props) {
  const options: Array<ListboxOption<string>> = visualColorOptions.map((color) => ({
    value: color,
    label: color,
    render: (
      <div className="mt-visual-select-option">
        <span
          className="mt-visual-select-option__color"
          style={{ backgroundColor: color }}
        />
        <span className="mt-mono">{color}</span>
      </div>
    ),
  }));

  return (
    <Listbox
      value={value}
      placeholder={placeholder}
      options={options}
      onChange={onChange}
    />
  );
}
