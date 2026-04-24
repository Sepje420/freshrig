export type PrivacyCategory =
  | "Telemetry"
  | "Permissions"
  | "Advertising"
  | "Activity"
  | "AiCopilot"
  | "Search"
  | "Suggestions";

export type PrivacyRisk = "Recommended" | "Limited" | "Advanced";

export interface PrivacySetting {
  id: string;
  name: string;
  description: string;
  category: PrivacyCategory;
  risk: PrivacyRisk;
  currentValue: boolean;
  recommended: boolean;
}

export interface AppPermission {
  appName: string;
  appPath: string | null;
  capability: string;
  allowed: boolean;
  lastUsed: string | null;
  isActiveNow: boolean;
}
