export type Grade = "A" | "B" | "C" | "D" | "F";

export interface SystemReport {
  hostname: string;
  osName: string;
  osBuild: string;
  uptimeHours: number;
  windowsActivated: boolean;
  windowsEdition: string;
}

export interface RamSlotReport {
  capacityGb: number;
  speedMhz: number;
  manufacturer: string;
  partNumber: string;
}

export interface HardwareReport {
  cpuName: string;
  cpuCores: number;
  cpuThreads: number;
  ramTotalGb: number;
  ramSlots: RamSlotReport[];
  gpus: string[];
  motherboard: string;
}

export type DriveHealth = "OK" | "Warning" | "Fail" | "Unknown";

export interface DriveSmartReport {
  model: string;
  sizeGb: number;
  healthStatus: DriveHealth;
  temperatureC: number | null;
  powerOnHours: number | null;
  wearPercentage: number | null;
  readErrorsTotal: number | null;
}

export interface BatteryReport {
  designCapacityMwh: number;
  fullChargeCapacityMwh: number;
  cycleCount: number;
  healthPercent: number;
}

export interface SecurityReport {
  antivirusName: string | null;
  antivirusEnabled: boolean;
  antivirusUpToDate: boolean;
  firewallEnabled: boolean;
  bitlockerStatus: string;
  tpmPresent: boolean;
  tpmEnabled: boolean;
}

export interface DriverSummaryReport {
  total: number;
  withErrors: number;
  errorDevices: string[];
}

export interface ReportData {
  generatedAt: string;
  appVersion: string;
  overallGrade: Grade;
  overallScore: number;
  system: SystemReport;
  hardware: HardwareReport;
  drives: DriveSmartReport[];
  battery: BatteryReport | null;
  security: SecurityReport;
  drivers: DriverSummaryReport;
  softwareCount: number;
  startupCount: number;
  startupEnabledCount: number;
  reliabilityIndex: number | null;
}
