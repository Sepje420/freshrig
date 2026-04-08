import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { AppEntry, AppCategory, InstallProgress } from "../types/apps";

interface AppState {
  catalog: AppEntry[];
  selectedIds: Set<string>;
  installProgress: Map<string, InstallProgress>;
  isInstalling: boolean;
  wingetAvailable: boolean | null;
  searchQuery: string;
  activeCategory: AppCategory | "all";
  loading: boolean;
  fetchCatalog: () => Promise<void>;
  checkWinget: () => Promise<void>;
  toggleApp: (id: string) => void;
  selectAll: (category: AppCategory | "all") => void;
  clearSelection: () => void;
  installSelected: () => Promise<void>;
  setSearchQuery: (q: string) => void;
  setActiveCategory: (cat: AppCategory | "all") => void;
}

let listenerInitialized = false;

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

    fetchCatalog: async () => {
      set({ loading: true });
      try {
        const catalog = await invoke<AppEntry[]>("get_app_catalog");
        set({ catalog, loading: false });
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
    },

    setActiveCategory: (cat: AppCategory | "all") => {
      set({ activeCategory: cat });
    },
  };
});
