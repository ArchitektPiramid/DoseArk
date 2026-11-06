import { existsSync } from "node:fs";
import path from "node:path";
import react from "@vitejs/plugin-react";
import UnoCSS from "unocss/vite";
import { defineConfig } from "vite";

import { profileConfigPlugin } from "./vite/plugins/profileConfigPlugin";
import { terminalLoggerPlugin } from "./vite/plugins/terminalLoggerPlugin";

const localProfilesPath = path.resolve(__dirname, "../../config/profiles.json");
// A fresh clone uses the tracked example; local overrides remain optional.
const frontendProfilesPath = existsSync(localProfilesPath)
  ? localProfilesPath
  : path.resolve(__dirname, "../../config/profiles.example.json");

export default defineConfig({
  plugins: [profileConfigPlugin(frontendProfilesPath), terminalLoggerPlugin(), react(), UnoCSS()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "src"),
    },
  },
  server: {
    port: 5173,
    host: "127.0.0.1",
  },
});
