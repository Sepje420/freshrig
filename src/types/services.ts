export type ServiceStartType =
  | "Automatic"
  | "AutoDelayed"
  | "Manual"
  | "Disabled"
  | "Unknown";

export type ServiceState =
  | "Running"
  | "Stopped"
  | "StartPending"
  | "StopPending"
  | "Unknown";

export interface ServiceEntry {
  name: string;
  displayName: string;
  description: string;
  startType: ServiceStartType;
  currentState: ServiceState;
  isProtected: boolean;
}

export interface ServiceChange {
  serviceName: string;
  targetStartType: ServiceStartType;
  rationale: string;
}

export interface ServicePreset {
  id: string;
  name: string;
  description: string;
  changes: ServiceChange[];
}

export interface ServicePresetResult {
  serviceName: string;
  success: boolean;
  message: string;
}
