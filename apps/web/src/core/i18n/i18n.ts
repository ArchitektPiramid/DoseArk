import i18n from "i18next";
import { initReactI18next } from "react-i18next";

import catalogEn from "./locales/en/catalog.json";
import commonEn from "./locales/en/common.json";
import inventoryEn from "./locales/en/inventory.json";
import catalogPl from "./locales/pl/catalog.json";
import commonPl from "./locales/pl/common.json";
import inventoryPl from "./locales/pl/inventory.json";
import dosingEn from "./locales/en/dosing.json";
import dosingPl from "./locales/pl/dosing.json";
import statisticsEn from "./locales/en/statistics.json";
import statisticsPl from "./locales/pl/statistics.json";

import { getInitialLanguage } from "./language";

void i18n.use(initReactI18next).init({
  resources: {
    en: {
      common: commonEn,
      catalog: catalogEn,
      inventory: inventoryEn,
      dosing: dosingEn,
      statistics: statisticsEn,
    },
    pl: {
      common: commonPl,
      catalog: catalogPl,
      inventory: inventoryPl,
      dosing: dosingPl,
      statistics: statisticsPl,
    },
  },
  lng: getInitialLanguage(),
  fallbackLng: "en",
  defaultNS: "common",
  ns: ["common", "catalog", "inventory", "dosing", "statistics"],
  interpolation: {
    escapeValue: false,
  },
});

export { i18n };
