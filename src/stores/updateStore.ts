import { create } from "zustand";
import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

interface UpdateState {
  status:
    | "idle"
    | "checking"
    | "available"
    | "downloading"
    | "installing"
    | "up-to-date"
    | "error";
  update: Update | null;
  newVersion: string | null;
  progress: number;
  total: number;
  error: string | null;
  dismissed: boolean;
  checkForUpdates: (silent?: boolean) => Promise<void>;
  downloadAndInstall: () => Promise<void>;
  dismiss: () => void;
}

export const useUpdateStore = create<UpdateState>((set, get) => ({
  status: "idle",
  update: null,
  newVersion: null,
  progress: 0,
  total: 0,
  error: null,
  dismissed: false,

  checkForUpdates: async (silent = true) => {
    set({ status: "checking", error: null, dismissed: false });
    try {
      const update = await check();
      if (update) {
        set({ status: "available", update, newVersion: update.version });
      } else {
        set({ status: "up-to-date" });
        if (!silent) {
          setTimeout(() => set({ status: "idle" }), 3000);
        }
      }
    } catch (err) {
      if (!silent) {
        set({ status: "error", error: String(err) });
      } else {
        set({ status: "idle" });
      }
    }
  },

  downloadAndInstall: async () => {
    const { update } = get();
    if (!update) return;
    set({ status: "downloading", progress: 0, total: 0 });

    try {
      await update.downloadAndInstall((event) => {
        if (event.event === "Started") {
          set({ total: event.data.contentLength ?? 0 });
        } else if (event.event === "Progress") {
          set((state) => ({ progress: state.progress + event.data.chunkLength }));
        } else if (event.event === "Finished") {
          set({ status: "installing" });
        }
      });
      await relaunch();
    } catch (err) {
      set({ status: "error", error: String(err) });
    }
  },

  dismiss: () => set({ status: "idle", dismissed: true }),
}));
