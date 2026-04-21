// Copyright (c) 2026 Seppe Willemsens (sepje420). MIT License.
import { create } from "zustand";
import { persist, createJSONStorage } from "zustand/middleware";

export type LicenseTier = "free" | "pro";

/**
 * Validate license key format. Keys must match: FR-XXXXX-XXXXX
 * where X is [A-Z0-9]. This is a format-only check — not a cryptographic
 * verification — but defeats casual bypass attempts like "FR-test".
 */
export function isValidLicenseFormat(key: string): boolean {
  if (!key.startsWith("FR-")) return false;
  if (key.length < 10) return false;
  return /^FR-[A-Z0-9]{5}-[A-Z0-9]{5}$/.test(key);
}

interface LicenseState {
  tier: LicenseTier;
  licenseKey: string | null;
  validatedAt: string | null;
  expiresAt: string | null;
  isPro: () => boolean;
  setLicense: (key: string, tier: LicenseTier) => boolean;
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

      setLicense: (key, tier) => {
        if (!isValidLicenseFormat(key)) return false;
        set({
          licenseKey: key,
          tier,
          validatedAt: new Date().toISOString(),
        });
        return true;
      },

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
