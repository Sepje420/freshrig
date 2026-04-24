// Copyright (c) 2026 Seppe Willemsens (ZIPREX420). MIT License.
import { useState, useEffect, useCallback } from "react";
import { Toaster } from "sonner";
import { ErrorBoundary } from "react-error-boundary";
import { useHotkeys } from "react-hotkeys-hook";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { MotionConfig, AnimatePresence, motion } from "framer-motion";
import { AppLayout } from "./components/layout/AppLayout";
import { UpdateBanner } from "./components/layout/UpdateBanner";
import { WhatsNewModal } from "./components/layout/WhatsNewModal";
import { Dashboard } from "./components/dashboard/Dashboard";
import { DriversPage } from "./components/drivers/DriversPage";
import { AppsPage } from "./components/apps/AppsPage";
import { ProfilesPage } from "./components/profiles/ProfilesPage";
import { OptimizePage } from "./components/optimize/OptimizePage";
import { StartupPage } from "./components/startup/StartupPage";
import { CleanupPage } from "./components/cleanup/CleanupPage";
import { PrivacyPage } from "./components/privacy/PrivacyPage";
import { NetworkPage } from "./components/network/NetworkPage";
import { ContextMenuPage } from "./components/context_menu/ContextMenuPage";
import { ServicesPage } from "./components/services/ServicesPage";
import { SettingsPage } from "./components/settings/SettingsPage";
import { AboutPage } from "./components/about/AboutPage";
import { OnboardingWizard } from "./components/onboarding/OnboardingWizard";
import { CommandPalette } from "./components/ui/CommandPalette";
import { ShortcutHelp } from "./components/ui/ShortcutHelp";
import { ErrorFallback } from "./components/ErrorFallback";
import { useSettingsStore } from "./stores/settingsStore";
import { useUpdateStore } from "./stores/updateStore";
import { useLicenseStore } from "./stores/licenseStore";
import { APP_VERSION } from "./config/app";

function App() {
  const [currentView, setCurrentView] = useState("dashboard");
  const [showWhatsNew, setShowWhatsNew] = useState(false);
  const [showCommandPalette, setShowCommandPalette] = useState(false);
  const [showShortcuts, setShowShortcuts] = useState(false);
  const { loadSettings, settings, setSetting, loaded } = useSettingsStore();

  const navigate = useCallback((view: string) => setCurrentView(view), []);

  // Keyboard shortcuts
  useHotkeys("ctrl+1", () => navigate("dashboard"), { preventDefault: true });
  useHotkeys("ctrl+2", () => navigate("drivers"), { preventDefault: true });
  useHotkeys("ctrl+3", () => navigate("apps"), { preventDefault: true });
  useHotkeys("ctrl+4", () => navigate("profiles"), { preventDefault: true });
  useHotkeys("ctrl+5", () => navigate("optimize"), { preventDefault: true });
  useHotkeys("ctrl+6", () => navigate("startup"), { preventDefault: true });
  useHotkeys("ctrl+7", () => navigate("cleanup"), { preventDefault: true });
  useHotkeys("ctrl+8", () => navigate("privacy"), { preventDefault: true });
  useHotkeys("ctrl+9", () => navigate("network"), { preventDefault: true });
  useHotkeys("ctrl+comma", () => navigate("settings"), { preventDefault: true });
  useHotkeys("ctrl+k", () => setShowCommandPalette((v) => !v), { preventDefault: true });
  useHotkeys("ctrl+shift+/", () => setShowShortcuts((v) => !v), { preventDefault: true });

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

  // Background license revalidation — 6h interval, initial check after 5s.
  useEffect(() => {
    if (!(window as unknown as Record<string, unknown>).__TAURI_INTERNALS__) return;
    const revalidate = () => useLicenseStore.getState().revalidate();
    const initial = setTimeout(revalidate, 5000);
    const interval = setInterval(revalidate, 6 * 60 * 60 * 1000);
    return () => {
      clearTimeout(initial);
      clearInterval(interval);
    };
  }, []);

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

  const handleCompleteOnboarding = useCallback(() => {
    setSetting("hasCompletedOnboarding", true);
  }, [setSetting]);

  return (
    <MotionConfig transition={{ type: "spring", stiffness: 380, damping: 30, mass: 0.8 }}>
      <ErrorBoundary FallbackComponent={ErrorFallback} onReset={() => window.location.reload()}>
        <UpdateBanner />
        <AppLayout currentView={currentView} onNavigate={navigate} onShowShortcuts={() => setShowShortcuts(true)}>
          <ErrorBoundary FallbackComponent={ErrorFallback} onReset={() => window.location.reload()}>
            <AnimatePresence mode="wait" initial={false}>
              <motion.div
                key={currentView}
                initial={{ opacity: 0, y: 6 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0 }}
                transition={{ duration: 0.15 }}
              >
                {currentView === "dashboard" && <Dashboard />}
                {currentView === "drivers" && <DriversPage />}
                {currentView === "apps" && <AppsPage />}
                {currentView === "profiles" && <ProfilesPage />}
                {currentView === "optimize" && <OptimizePage />}
                {currentView === "startup" && <StartupPage />}
                {currentView === "cleanup" && <CleanupPage />}
                {currentView === "privacy" && <PrivacyPage />}
                {currentView === "network" && <NetworkPage />}
                {currentView === "contextMenu" && <ContextMenuPage />}
                {currentView === "services" && <ServicesPage />}
                {currentView === "settings" && <SettingsPage onNavigate={navigate} />}
                {currentView === "about" && <AboutPage />}
              </motion.div>
            </AnimatePresence>
          </ErrorBoundary>
        </AppLayout>
        {loaded && !settings.hasCompletedOnboarding && (
          <OnboardingWizard onComplete={handleCompleteOnboarding} />
        )}
        {showCommandPalette && (
          <CommandPalette onClose={() => setShowCommandPalette(false)} onNavigate={(v) => { navigate(v); setShowCommandPalette(false); }} />
        )}
        {showShortcuts && <ShortcutHelp onClose={() => setShowShortcuts(false)} />}
        {showWhatsNew && <WhatsNewModal onClose={handleCloseWhatsNew} />}
        <Toaster
          theme="dark"
          position="bottom-right"
          toastOptions={{
            style: {
              background: "var(--bg-elevated)",
              border: "1px solid var(--border)",
              color: "var(--text-primary)",
            },
            className: "freshrig-toast",
          }}
        />
      </ErrorBoundary>
    </MotionConfig>
  );
}

export default App;
