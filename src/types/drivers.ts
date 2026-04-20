export type DriverCategory = "Gpu" | "Chipset" | "Network" | "Audio" | "Other";

export type DriverStatus = "UpToDate" | "UpdateAvailable" | "Missing" | "Unknown";

export type DriverInstallAction = "Winget" | "OpenUrl";

export interface DriverRecommendation {
  deviceName: string;
  category: DriverCategory;
  vendor: string;
  currentVersion: string | null;
  currentDate: string | null;
  downloadUrl: string;
  downloadPage: string;
  status: DriverStatus;
  wingetId?: string | null;
  installAction: DriverInstallAction;
}
