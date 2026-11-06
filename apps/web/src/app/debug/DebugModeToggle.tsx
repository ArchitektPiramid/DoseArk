import { Button } from "../../core/ui";
import { useDebugMode } from "./DebugModeProvider";

type DebugModeToggleProps = {
  compact?: boolean;
};

export function DebugModeToggle({ compact = false }: DebugModeToggleProps) {
  const debug = useDebugMode();
  const label = `DEBUG: ${debug.enabled ? "ON" : "OFF"}`;

  return (
    <Button
      variant={debug.enabled ? "primary" : "default"}
      className={compact ? "mt-sidebar__compact-button" : undefined}
      onClick={debug.toggleDebugMode}
      title={compact ? label : undefined}
      aria-label={compact ? label : undefined}
    >
      {compact ? <span className="mt-mono">D</span> : label}
    </Button>
  );
}
