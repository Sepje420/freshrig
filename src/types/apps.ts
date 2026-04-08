export type AppCategory =
  | "Browser"
  | "Gaming"
  | "Communication"
  | "Development"
  | "Media"
  | "Productivity"
  | "Utilities"
  | "Security"
  | "Runtime";

export type InstallStatus = "Pending" | "Installing" | "Completed" | "Failed" | "Skipped";

export interface AppEntry {
  id: string;
  name: string;
  description: string;
  category: AppCategory;
  iconName: string;
  isPopular: boolean;
}

export interface InstallProgress {
  appId: string;
  appName: string;
  status: InstallStatus;
  message: string;
}
