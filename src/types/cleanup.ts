export type CleanupRisk = "Safe" | "Moderate" | "Expert";

export interface CleanupCategory {
  id: string;
  name: string;
  description: string;
  risk: CleanupRisk;
  fileCount: number;
  totalBytes: number;
  paths: string[];
  enabledByDefault: boolean;
}

export interface CleanupResult {
  categoryId: string;
  filesDeleted: number;
  bytesFreed: number;
  errors: string[];
}

export interface CleanupScanProgress {
  categoryId: string;
  fileCount: number;
  totalBytes: number;
}

export interface CleanupProgress {
  categoryId: string;
  filesDeleted: number;
  bytesFreed: number;
}
