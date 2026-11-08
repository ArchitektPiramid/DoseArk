import {
  BarChart3,
  Box,
  Building2,
  ChevronLeft,
  ChevronRight,
  Home,
  Leaf,
  LayoutDashboard,
  ListPlus,
  Package,
  Pill,
  Settings as SettingsIcon,
  type LucideIcon,
} from "lucide-react";
import { NavLink } from "react-router";

import { type ConnectionStatus, useSystemStatus } from "@/app/system_status";
import {
  APP_NAME,
  APP_RELEASE_VERSION,
} from "@/core/version/appVersion";
import { DebugModeToggle } from "../debug/DebugModeToggle";
import { useDebugMode } from "../debug/DebugModeProvider";

type SidebarProps = {
  collapsed: boolean;
  onToggle: () => void;
};

type SidebarNavItem = {
  to: string;
  label: string;
  icon: LucideIcon;
  debugOnly?: boolean;
};

const NAV_ITEMS: SidebarNavItem[] = [
  { to: "/dashboard", label: "Dashboard", icon: LayoutDashboard },
  { to: "/products", label: "Products", icon: Package },
  { to: "/brands", label: "Brands", icon: Building2 },
  { to: "/storage-locations", label: "Storage Loc.", icon: Home },
  { to: "/nutrients", label: "Nutrients", icon: Leaf },
  { to: "/inventory", label: "Inventory", icon: Box },
  { to: "/dosing", label: "Dosing", icon: Pill },
  { to: "/debug/bulk-dosing", label: "Bulk dosing", icon: ListPlus, debugOnly: true },
  { to: "/statistics", label: "Statistics", icon: BarChart3 },
  { to: "/settings", label: "Settings", icon: SettingsIcon },
];

export function Sidebar({ collapsed, onToggle }: SidebarProps) {
  const system = useSystemStatus();
  const debug = useDebugMode();
  const databaseStatus = getDatabaseStatus(system.status, system.health?.database.status);

  return (
    <aside className="mt-sidebar" aria-label="Main navigation">
      <div className="mt-sidebar__inner">
        <div className="mt-sidebar__brand">
          <div className="mt-sidebar__brand-row">
            <div className="mt-sidebar__brand-mark" title={collapsed ? APP_NAME : undefined}>
              <Pill size={18} aria-hidden="true" />
            </div>

            <div className="mt-sidebar__brand-copy">
              <div className="mt-sidebar__brand-name">{APP_NAME}</div>
              <div className="mt-sidebar__version mt-mono">{APP_RELEASE_VERSION}</div>
            </div>

            <button
              type="button"
              className="mt-sidebar__collapse-button"
              onClick={onToggle}
              aria-label={collapsed ? "Expand sidebar" : "Collapse sidebar"}
              title={collapsed ? "Expand sidebar" : "Collapse sidebar"}
            >
              {collapsed ? <ChevronRight size={17} /> : <ChevronLeft size={17} />}
            </button>
          </div>
        </div>

        <nav className="mt-sidebar__nav">
          {NAV_ITEMS.filter((item) => !item.debugOnly || debug.enabled).map((item) => {
            const Icon = item.icon;

            return (
              <NavLink
                key={item.to}
                to={item.to}
                title={collapsed ? item.label : undefined}
                aria-label={collapsed ? item.label : undefined}
                className={({ isActive }) =>
                  ["mt-sidebar__nav-link", isActive ? "mt-sidebar__nav-link--active" : ""].join(
                    " ",
                  )
                }
              >
                <Icon className="mt-sidebar__nav-icon" size={18} strokeWidth={1.8} aria-hidden="true" />
                <span className="mt-sidebar__nav-label">{item.label}</span>
              </NavLink>
            );
          })}
        </nav>

        <div className="mt-sidebar__footer">
          <div className="mt-sidebar__debug-toggle">
            <DebugModeToggle compact={collapsed} />
          </div>

          {debug.enabled ? (
            <div className="mt-sidebar__status-inline mt-sidebar__status-inline--debug" aria-label="Debug runtime status">
              <div
                className={`mt-sidebar__status-inline-item mt-sidebar__status-inline-item--${system.status}`}
                title={`API — ${system.error ?? `last check ${formatCheckedAt(system.checkedAt)}`}`}
              >
                <span className="mt-status-dot" aria-hidden="true" />
                <span className="mt-sidebar__status-label">API v{system.health?.version ?? "—"}</span>
              </div>

              <div
                className={`mt-sidebar__status-inline-item mt-sidebar__status-inline-item--${databaseStatus}`}
                title={
                  system.health?.database.database_name
                    ? `DB — ${system.health.database.engine}: ${system.health.database.database_name}`
                    : `DB — ${system.error ?? "status requires a reachable API"}`
                }
              >
                <span className="mt-status-dot" aria-hidden="true" />
                <span className="mt-sidebar__status-label">DB v{system.health?.database.schema_version ?? "—"}</span>
              </div>
            </div>
          ) : null}
        </div>
      </div>
    </aside>
  );
}

function getDatabaseStatus(
  apiStatus: ConnectionStatus,
  databaseStatus: "ok" | "error" | undefined,
): ConnectionStatus {
  if (apiStatus === "checking") return "checking";
  if (apiStatus === "offline") return "offline";
  if (databaseStatus === "ok") return "ready";
  return "degraded";
}

function formatCheckedAt(value: Date | null): string {
  if (!value) return "not checked yet";
  return value.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", second: "2-digit" });
}
