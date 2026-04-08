import type { AppCategory } from "./apps";

export interface RigProfile {
  configVersion: number;
  metadata: ProfileMetadata;
  apps: string[];
  categories: AppCategory[];
  notes?: string;
}

export interface ProfileMetadata {
  name: string;
  description?: string;
  author?: string;
  createdAt: string;
  updatedAt: string;
  appVersion: string;
  sourceHardware?: SourceHardware;
}

export interface SourceHardware {
  cpu?: string;
  gpu?: string;
  ramGb?: number;
  os?: string;
}

export interface ProfileSummary {
  filePath: string;
  name: string;
  description?: string;
  appCount: number;
  createdAt: string;
  updatedAt: string;
}
