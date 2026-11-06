import "@unocss/reset/tailwind.css";
import "uno.css";
import "@/styles/index.css";
import "@/core/i18n/i18n";

import React from "react";
import ReactDOM from "react-dom/client";

import { App } from "./app/App";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
