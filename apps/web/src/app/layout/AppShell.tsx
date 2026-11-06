import { useState } from "react";
import { Outlet } from "react-router";

import { DevConsoleDrawer } from "../dev_console/DevConsoleDrawer";
import { Sidebar } from "./Sidebar";

const SIDEBAR_COLLAPSED_KEY = "doseark.ui.sidebar-collapsed";

export function AppShell() {
  const [sidebarCollapsed, setSidebarCollapsed] = useState(readInitialSidebarState);

  function toggleSidebar() {
    setSidebarCollapsed((current) => {
      const next = !current;
      window.localStorage.setItem(SIDEBAR_COLLAPSED_KEY, next ? "1" : "0");
      return next;
    });
  }

  return (
    <div className={`mt-app${sidebarCollapsed ? " mt-app--sidebar-collapsed" : ""}`}>
      <Sidebar collapsed={sidebarCollapsed} onToggle={toggleSidebar} />
      <div className="mt-shell">
        <main className="mt-main">
          <Outlet />
        </main>
      </div>

      <DevConsoleDrawer />
    </div>
  );
}

function readInitialSidebarState(): boolean {
  if (typeof window === "undefined") return false;
  return window.localStorage.getItem(SIDEBAR_COLLAPSED_KEY) === "1";
}
