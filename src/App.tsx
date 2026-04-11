import { useState, useEffect } from "react";
import { Toaster } from "sonner";
import { ErrorBoundary } from "react-error-boundary";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { AppLayout } from "./components/layout/AppLayout";
import { UpdateBanner } from "./components/layout/UpdateBanner";
import { WhatsNewModal } from "./components/layout/WhatsNewModal";
import { Dashboard } from "./components/dashboard/Dashboard";
import { DriversPage } from "./components/drivers/DriversPage";
import { AppsPage } from "./components/apps/AppsPage";
import { ProfilesPage } from "./components/profiles/ProfilesPage";
import { SettingsPage } from "./components/settings/SettingsPage";
import { AboutPage } from "./components/about/AboutPage";
import { ErrorFallback } from "./components/ErrorFallback";
import { useSettingsStore } from "./stores/settingsStore";
import { useUpdateStore } from "./stores/updateStore";
import { APP_VERSION } from "./config/app";

function App() {
  const [currentView, setCurrentView] = useState("dashboard");
  const [showWhatsNew, setShowWhatsNew] = useState(false);
  const { loadSettings, settings, setSetting } = useSettingsStore();

  // Load settings on startup
  useEffect(() => {
    loadSettings();
  }, [loadSettings]);

  // Check for updates on startup (after a short delay)
  useEffect(() => {
    if (!settings.checkForUpdates) return;
    if (!(window as unknown as Record<string, unknown>).__TAURI_INTERNALS__) return;
    const timer = setTimeout(() => {
      useUpdateStore.getState().checkForUpdates(true);
    }, 3000);
    return () => clearTimeout(timer);
  }, [settings.checkForUpdates]);

  // Show "What's New" modal if version changed
  useEffect(() => {
    const { loaded } = useSettingsStore.getState();
    if (!loaded) return;
    if (settings.lastSeenVersion !== APP_VERSION) {
      setShowWhatsNew(true);
    }
  }, [settings.lastSeenVersion]);

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

  const handleCloseWhatsNew = () => {
    setShowWhatsNew(false);
    setSetting("lastSeenVersion", APP_VERSION);
  };

  return (
    <ErrorBoundary FallbackComponent={ErrorFallback} onReset={() => window.location.reload()}>
      <UpdateBanner />
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
      {showWhatsNew && <WhatsNewModal onClose={handleCloseWhatsNew} />}
      <Toaster theme="dark" position="bottom-right" richColors />
    </ErrorBoundary>
  );
}

export default App;
