import { useState, useEffect } from "react";
import { Toaster } from "sonner";
import { ErrorBoundary } from "react-error-boundary";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { AppLayout } from "./components/layout/AppLayout";
import { Dashboard } from "./components/dashboard/Dashboard";
import { DriversPage } from "./components/drivers/DriversPage";
import { AppsPage } from "./components/apps/AppsPage";
import { ProfilesPage } from "./components/profiles/ProfilesPage";
import { SettingsPage } from "./components/settings/SettingsPage";
import { AboutPage } from "./components/about/AboutPage";
import { ErrorFallback } from "./components/ErrorFallback";
import { useSettingsStore } from "./stores/settingsStore";

function App() {
  const [currentView, setCurrentView] = useState("dashboard");
  const { loadSettings, settings } = useSettingsStore();

  // Load settings on startup
  useEffect(() => {
    loadSettings();
  }, [loadSettings]);

  // Override window close → minimize to tray (only in Tauri)
  useEffect(() => {
    if (!(window as unknown as Record<string, unknown>).__TAURI_INTERNALS__) return;
    const appWindow = getCurrentWindow();
    let unlisten: (() => void) | undefined;

    appWindow.onCloseRequested(async (event) => {
      if (settings.minimizeToTray) {
        event.preventDefault();
        await appWindow.hide();
      }
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      unlisten?.();
    };
  }, [settings.minimizeToTray]);

  return (
    <ErrorBoundary FallbackComponent={ErrorFallback} onReset={() => window.location.reload()}>
      <AppLayout currentView={currentView} onNavigate={setCurrentView}>
        <ErrorBoundary FallbackComponent={ErrorFallback} onReset={() => window.location.reload()}>
          {currentView === "dashboard" && <Dashboard />}
          {currentView === "drivers" && <DriversPage />}
          {currentView === "apps" && <AppsPage />}
          {currentView === "profiles" && <ProfilesPage />}
          {currentView === "settings" && <SettingsPage onNavigate={setCurrentView} />}
          {currentView === "about" && <AboutPage />}
        </ErrorBoundary>
      </AppLayout>
      <Toaster theme="dark" position="bottom-right" richColors />
    </ErrorBoundary>
  );
}

export default App;
