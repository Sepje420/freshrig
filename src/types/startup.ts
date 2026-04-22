export type StartupSource =
  | "RegistryRun"
  | "RegistryRunOnce"
  | "StartupFolder"
  | "TaskScheduler";

export type StartupScope = "CurrentUser" | "AllUsers";

export type StartupImpact = "High" | "Medium" | "Low" | "NotMeasured";

export interface StartupEntry {
  id: string;
  name: string;
  command: string;
  source: StartupSource;
  scope: StartupScope;
  enabled: boolean;
  publisher: string | null;
  impact: StartupImpact;
}
