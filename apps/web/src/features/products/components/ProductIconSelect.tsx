import { SelectInput } from "@/core/ui";

import { productIconOptions } from "../utils/productIconOptions";

type Props = {
  value: string;
  placeholder?: string;
  onChange: (value: string) => void;
};

/** Uses the same native select as the other product fields. */
export function ProductIconSelect({
  value,
  placeholder = "Select product icon",
  onChange,
}: Props) {
  const isCustomIcon = value !== "" && !productIconOptions.some((option) => option.value === value);

  return (
    <SelectInput value={value} onChange={(event) => onChange(event.target.value)}>
      <option value="">{placeholder}</option>
      {isCustomIcon ? <option value={value}>{`Custom: ${value}`}</option> : null}
      {productIconOptions.map((option) => (
        <option key={option.value} value={option.value}>
          {option.label}
        </option>
      ))}
    </SelectInput>
  );
}
