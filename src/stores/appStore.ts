import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { AppEntry, AppCategory, InstallProgress } from "../types/apps";

interface WingetSearchResult {
  name: string;
  id: string;
  version: string;
  source: string;
}

interface AppState {
  catalog: AppEntry[];
  selectedIds: Set<string>;
  installProgress: Map<string, InstallProgress>;
  isInstalling: boolean;
  wingetAvailable: boolean | null;
  searchQuery: string;
  activeCategory: AppCategory | "all";
  loading: boolean;
  // Winget search
  wingetResults: WingetSearchResult[];
  isSearchingWinget: boolean;
  // Installed detection
  installedAppIds: Set<string>;
  isCheckingInstalled: boolean;
  hideInstalled: boolean;
  // Actions
  fetchCatalog: () => Promise<void>;
  checkWinget: () => Promise<void>;
  toggleApp: (id: string) => void;
  selectAll: (category: AppCategory | "all") => void;
  clearSelection: () => void;
  installSelected: () => Promise<void>;
  setSearchQuery: (q: string) => void;
  setActiveCategory: (cat: AppCategory | "all") => void;
  searchWinget: (query: string) => Promise<void>;
  checkInstalledApps: () => Promise<void>;
  setHideInstalled: (hide: boolean) => void;
}

let listenerInitialized = false;
let searchDebounceTimer: ReturnType<typeof setTimeout> | null = null;

export const useAppStore = create<AppState>((set, get) => {
  // Set up event listener once
  if (!listenerInitialized) {
    listenerInitialized = true;
    listen<InstallProgress>("install-progress", (event) => {
      const progress = event.payload;
      set((state) => {
        const newMap = new Map(state.installProgress);
        newMap.set(progress.appId, progress);
        const allDone = [...newMap.values()].every(
          (p) => p.status === "Completed" || p.status === "Failed" || p.status === "Skipped"
        );
        return {
          installProgress: newMap,
          isInstalling: !allDone,
        };
      });
    });
  }

  return {
    catalog: [],
    selectedIds: new Set(),
    installProgress: new Map(),
    isInstalling: false,
    wingetAvailable: null,
    searchQuery: "",
    activeCategory: "all",
    loading: false,
    wingetResults: [],
    isSearchingWinget: false,
    installedAppIds: new Set(),
    isCheckingInstalled: false,
    hideInstalled: false,

    fetchCatalog: async () => {
      set({ loading: true });
      try {
        const catalog = await invoke<AppEntry[]>("get_app_catalog");
        set({ catalog, loading: false });
        // Auto-check installed apps after catalog loads
        get().checkInstalledApps();
      } catch {
        set({ loading: false });
      }
    },

    checkWinget: async () => {
      try {
        const available = await invoke<boolean>("check_winget_available");
        set({ wingetAvailable: available });
      } catch {
        set({ wingetAvailable: false });
      }
    },

    toggleApp: (id: string) => {
      set((state) => {
        const newSet = new Set(state.selectedIds);
        if (newSet.has(id)) {
          newSet.delete(id);
        } else {
          newSet.add(id);
        }
        return { selectedIds: newSet };
      });
    },

    selectAll: (category: AppCategory | "all") => {
      const { catalog } = get();
      const filtered =
        category === "all" ? catalog : catalog.filter((a) => a.category === category);
      set({ selectedIds: new Set(filtered.map((a) => a.id)) });
    },

    clearSelection: () => {
      set({ selectedIds: new Set() });
    },

    installSelected: async () => {
      const { selectedIds } = get();
      if (selectedIds.size === 0) return;

      const appIds = [...selectedIds];

      // Initialize progress as Pending for all
      const initialProgress = new Map<string, InstallProgress>();
      const { catalog } = get();
      for (const id of appIds) {
        const app = catalog.find((a) => a.id === id);
        initialProgress.set(id, {
          appId: id,
          appName: app?.name ?? id,
          status: "Pending",
          message: "Waiting...",
        });
      }

      set({ isInstalling: true, installProgress: initialProgress });

      try {
        await invoke("install_apps", { appIds });
      } catch (err) {
        console.error("Install failed:", err);
        set({ isInstalling: false });
      }
    },

    setSearchQuery: (q: string) => {
      set({ searchQuery: q });

      // Debounced winget search
      if (searchDebounceTimer) clearTimeout(searchDebounceTimer);
      if (q.trim().length >= 2) {
        searchDebounceTimer = setTimeout(() => {
          get().searchWinget(q);
        }, 300);
      } else {
        set({ wingetResults: [], isSearchingWinget: false });
      }
    },

    setActiveCategory: (cat: AppCategory | "all") => {
      set({ activeCategory: cat });
    },

    searchWinget: async (query: string) => {
      if (query.trim().length < 2) {
        set({ wingetResults: [], isSearchingWinget: false });
        return;
      }
      set({ isSearchingWinget: true });
      try {
        const results = await invoke<WingetSearchResult[]>("search_winget_packages", { query });
        set({ wingetResults: results, isSearchingWinget: false });
      } catch {
        set({ wingetResults: [], isSearchingWinget: false });
      }
    },

    checkInstalledApps: async () => {
      const { catalog } = get();
      if (catalog.length === 0) return;

      set({ isCheckingInstalled: true });
      try {
        const wingetIds = catalog.map((a) => a.id);
        const catalogNames = catalog.map((a) => a.name);
        const foundIds = await invoke<string[]>("check_apps_installed", {
          wingetIds,
          catalogNames,
        });
        set({ installedAppIds: new Set(foundIds), isCheckingInstalled: false });
      } catch {
        set({ isCheckingInstalled: false });
      }
    },

    setHideInstalled: (hide: boolean) => {
      set({ hideInstalled: hide });
    },
  };
});
