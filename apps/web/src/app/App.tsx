import { BrowserRouter } from "react-router";
import { ApiProfilesProvider } from "./api_profiles";
import { AppRouter } from "./router";
import { DebugModeProvider } from "./debug/DebugModeProvider";
import { SystemStatusProvider } from "./system_status";
import { FrontendEventBridge } from "./frontend_events/FrontendEventBridge";

export function App() {
  return (
    <BrowserRouter>
      <ApiProfilesProvider>
        <SystemStatusProvider>
          <DebugModeProvider>
            <FrontendEventBridge />
            <AppRouter />
          </DebugModeProvider>
        </SystemStatusProvider>
      </ApiProfilesProvider>
    </BrowserRouter>
  );
}
