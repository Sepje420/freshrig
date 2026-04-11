import { create } from "zustand";
import { persist, createJSONStorage } from "zustand/middleware";

export type LicenseTier = "free" | "pro";

interface LicenseState {
  tier: LicenseTier;
  licenseKey: string | null;
  validatedAt: string | null;
  expiresAt: string | null;
  isPro: () => boolean;
  setLicense: (key: string, tier: LicenseTier) => void;
  clearLicense: () => void;
}

export const useLicenseStore = create<LicenseState>()(
  persist(
    (set, get) => ({
      tier: "free" as LicenseTier,
      licenseKey: null,
      validatedAt: null,
      expiresAt: null,

      isPro: () => get().tier === "pro",

      setLicense: (key, tier) =>
        set({
          licenseKey: key,
          tier,
          validatedAt: new Date().toISOString(),
        }),

      clearLicense: () =>
        set({
          tier: "free" as LicenseTier,
          licenseKey: null,
          validatedAt: null,
          expiresAt: null,
        }),
    }),
    {
      name: "freshrig-license",
      storage: createJSONStorage(() => localStorage),
    },
  ),
);
