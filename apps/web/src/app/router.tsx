import { Navigate, Route, Routes } from "react-router";
import { BrandsPage } from "@/features/catalog/brands";
import { NutrientsPage } from "@/features/catalog/nutrients";
import { DashboardPage } from "@/features/dashboard";
import { BulkDosingDebugPage, DosingPage } from "@/features/dosing";
import { InventoryPage } from "@/features/inventory";
import { ProductsPage } from "@/features/products";
import { SettingsPage } from "@/features/settings";
import { StatisticsPage } from "@/features/statistics";
import { StorageLocationsPage } from "@/features/storage-locations";
import { AppShell } from "./layout/AppShell";

export function AppRouter() {
  return (
    <Routes>
      <Route element={<AppShell />}>
        <Route index element={<Navigate to="/dashboard" replace />} />
        <Route path="dashboard" element={<DashboardPage />} />
        <Route path="products" element={<ProductsPage />} />
        <Route path="brands" element={<BrandsPage />} />
        <Route path="storage-locations" element={<StorageLocationsPage />} />
        <Route path="nutrients" element={<NutrientsPage />} />
        <Route path="inventory" element={<InventoryPage />} />
        <Route path="dosing" element={<DosingPage />} />
        <Route path="debug/bulk-dosing" element={<BulkDosingDebugPage />} />
        <Route path="statistics" element={<StatisticsPage />} />
        <Route path="reports" element={<Navigate to="/statistics" replace />} />
        <Route path="statistics-v2" element={<Navigate to="/statistics" replace />} />
        <Route path="settings" element={<SettingsPage />} />
        <Route path="*" element={<Navigate to="/dashboard" replace />} />
      </Route>
    </Routes>
  );
}
