import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import type { DriverRecommendation } from "../types/drivers";

interface DriverState {
  recommendations: DriverRecommendation[];
  loading: boolean;
  error: string | null;
  fetchRecommendations: () => Promise<void>;
}

export const useDriverStore = create<DriverState>((set) => ({
  recommendations: [],
  loading: false,
  error: null,

  fetchRecommendations: async () => {
    set({ loading: true, error: null });
    try {
      const recommendations = await invoke<DriverRecommendation[]>("get_driver_recommendations");
      set({ recommendations, loading: false });
    } catch (err) {
      set({
        error: err instanceof Error ? err.message : String(err),
        loading: false,
      });
    }
  },
}));
