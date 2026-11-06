import { defineConfig, presetAttributify, presetIcons, presetTypography, presetUno } from "unocss";

export default defineConfig({
  presets: [presetUno(), presetAttributify(), presetTypography(), presetIcons()],
  theme: {
    colors: {
      brand: {
        50: "#eff6ff",
        100: "#dbeafe",
        500: "#3b82f6",
        600: "#2563eb",
        700: "#1d4ed8",
      },
    },
  },
});
