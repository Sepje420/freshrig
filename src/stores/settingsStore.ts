import { create } from "zustand";
import { load, Store } from "@tauri-apps/plugin-store";
import type { AppCategory } from "../types/apps";

export interface AppSettings {
  // General
  defaultInstallBehavior: "silent" | "interactive";
  showHardwareInProfiles: boolean;
  checkForUpdates: boolean;
  // App Catalog
  defaultCategory: AppCategory | "all";
  showRuntimes: boolean;
  confirmBeforeInstalling: boolean;
  // Appearance
  accentColor: string;
  // System Tray
  minimizeToTray: boolean;
  startMinimized: boolean;
  // Updates
  lastSeenVersion: string;
}

const DEFAULT_SETTINGS: AppSettings = {
  defaultInstallBehavior: "silent",
  showHardwareInProfiles: true,
  checkForUpdates: true,
  defaultCategory: "all",
  showRuntimes: true,
  confirmBeforeInstalling: true,
  accentColor: "#00d4aa",
  minimizeToTray: true,
  startMinimized: false,
  lastSeenVersion: "0.2.0",
};

interface SettingsState {
  settings: AppSettings;
  loaded: boolean;
  store: Store | null;
  loadSettings: () => Promise<void>;
  setSetting: <K extends keyof AppSettings>(key: K, value: AppSettings[K]) => Promise<void>;
  resetSettings: () => Promise<void>;
}

function applyAccentColor(color: string) {
  const root = document.documentElement;
  root.style.setProperty("--color-accent", color);
  root.style.setProperty("--color-accent-hover", lightenColor(color, 15));
  root.style.setProperty("--color-accent-muted", color + "33");
  root.style.setProperty("--color-accent-glow", color + "22");
}

function lightenColor(hex: string, percent: number): string {
  const num = parseInt(hex.replace("#", ""), 16);
  const r = Math.min(255, ((num >> 16) & 0xff) + Math.round(255 * (percent / 100)));
  const g = Math.min(255, ((num >> 8) & 0xff) + Math.round(255 * (percent / 100)));
  const b = Math.min(255, (num & 0xff) + Math.round(255 * (percent / 100)));
  return `#${((r << 16) | (g << 8) | b).toString(16).padStart(6, "0")}`;
}

export const useSettingsStore = create<SettingsState>((set, get) => ({
  settings: { ...DEFAULT_SETTINGS },
  loaded: false,
  store: null,

  loadSettings: async () => {
    if (!(window as unknown as Record<string, unknown>).__TAURI_INTERNALS__) {
      set({ loaded: true });
      return;
    }
    try {
      const store = await load("settings.json", { autoSave: true, defaults: {} });
      const saved: Partial<AppSettings> = {};
      for (const key of Object.keys(DEFAULT_SETTINGS) as (keyof AppSettings)[]) {
        const val = await store.get(key);
        if (val !== null && val !== undefined) {
          (saved as Record<string, unknown>)[key] = val;
        }
      }
      const merged = { ...DEFAULT_SETTINGS, ...saved };
      applyAccentColor(merged.accentColor);
      set({ settings: merged, loaded: true, store });
    } catch {
      set({ loaded: true });
    }
  },

  setSetting: async (key, value) => {
    const { store } = get();
    set((state) => ({
      settings: { ...state.settings, [key]: value },
    }));
    if (key === "accentColor") {
      applyAccentColor(value as string);
    }
    if (store) {
      await store.set(key, value);
    }
  },

  resetSettings: async () => {
    const { store } = get();
    if (store) {
      await store.clear();
    }
    applyAccentColor(DEFAULT_SETTINGS.accentColor);
    set({ settings: { ...DEFAULT_SETTINGS } });
  },
}));
