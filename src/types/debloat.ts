export type TweakTier = "safe" | "moderate" | "expert";
export type TweakCategory = "privacy" | "bloatware" | "performance" | "appearance";
export type TweakType = "registrySet" | "appxRemove" | "serviceDisable" | "scheduledTask";

export interface DebloatTweak {
  id: string;
  name: string;
  description: string;
  tier: TweakTier;
  category: TweakCategory;
  tweakType: TweakType;
  isApplied: boolean;
  isReversible: boolean;
  warning: string | null;
  minWindowsBuild: number | null;
  incompatible: boolean;
}

export interface DebloatResult {
  tweakId: string;
  success: boolean;
  message: string;
}
