export type InstallerType = "nsis" | "innoSetup" | "msi" | "exe" | "unknown";

export interface CustomAppEntry {
  id: string;
  name: string;
  description: string | null;
  downloadUrl: string;
  installerType: InstallerType;
  silentArgs: string;
  expectedHash: string | null;
  createdAt: string;
  lastUsed: string | null;
}

export interface DownloadProgress {
  downloaded: number;
  total: number;
  filename: string;
}

export const INSTALLER_TYPE_LABELS: Record<InstallerType, string> = {
  nsis: "NSIS",
  innoSetup: "Inno Setup",
  msi: "MSI",
  exe: "Generic EXE",
  unknown: "Unknown",
};

export const INSTALLER_DEFAULT_ARGS: Record<InstallerType, string> = {
  nsis: "/S",
  innoSetup: "/VERYSILENT /SUPPRESSMSGBOXES /NORESTART",
  msi: "/qn /norestart",
  exe: "",
  unknown: "",
};

export function detectInstallerType(url: string): InstallerType {
  const filename = url.split("/").pop()?.toLowerCase() ?? "";
  if (filename.endsWith(".msi")) return "msi";
  if (filename.includes("setup") || filename.includes("install")) return "nsis";
  return "unknown";
}
