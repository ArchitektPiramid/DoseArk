import type { ReactNode } from "react";
import { ChevronDown } from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";

export type ListboxOption<T extends string> = {
  value: T;
  label: string;
  disabled?: boolean;
  render?: ReactNode;
};

type Props<T extends string> = {
  value: T | "";
  placeholder: string;
  options: Array<ListboxOption<T>>;
  onChange: (value: T) => void;
};

export function Listbox<T extends string>({ value, placeholder, options, onChange }: Props<T>) {
  const rootRef = useRef<HTMLDivElement | null>(null);
  const [open, setOpen] = useState(false);

  const selected = useMemo(
    () => options.find((option) => option.value === value),
    [options, value],
  );

  useEffect(() => {
    function onPointerDown(event: PointerEvent) {
      if (!rootRef.current?.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    document.addEventListener("pointerdown", onPointerDown);
    return () => document.removeEventListener("pointerdown", onPointerDown);
  }, []);

  function select(option: ListboxOption<T>) {
    if (option.disabled) return;

    onChange(option.value);
    setOpen(false);
  }

  return (
    <div ref={rootRef} className="mt-listbox">
      <button
        type="button"
        className="mt-listbox__button"
        onClick={() => setOpen((current) => !current)}
      >
        <span className="mt-listbox__button-main">
          {selected?.render ?? selected?.label ?? (
            <span className="mt-listbox__placeholder">{placeholder}</span>
          )}
        </span>

        <ChevronDown className="mt-listbox__chevron" size={16} />
      </button>

      {open ? (
        <div className="mt-listbox__popup" role="listbox">
          {options.map((option) => (
            <button
              key={option.value}
              type="button"
              className={
                option.value === value ? "mt-listbox__option is-selected" : "mt-listbox__option"
              }
              disabled={option.disabled}
              onClick={() => select(option)}
            >
              {option.render ?? option.label}
            </button>
          ))}
        </div>
      ) : null}
    </div>
  );
}
