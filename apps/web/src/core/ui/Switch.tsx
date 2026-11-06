type SwitchProps = {
  checked: boolean;
  onChange: (checked: boolean) => void;
  ariaLabel: string;
  disabled?: boolean;
};

export function Switch({ checked, onChange, ariaLabel, disabled = false }: SwitchProps) {
  return (
    <label className="mt-switch">
      <input
        type="checkbox"
        checked={checked}
        disabled={disabled}
        aria-label={ariaLabel}
        onChange={(event) => onChange(event.target.checked)}
      />
      <span className="mt-switch__track" aria-hidden="true">
        <span className="mt-switch__thumb" />
      </span>
    </label>
  );
}
