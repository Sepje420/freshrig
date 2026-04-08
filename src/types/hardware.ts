export interface SystemInfo {
  hostname: string;
  osVersion: string;
  osBuild: string;
  architecture: string;
  totalRamGb: number;
  uptimeSeconds: number;
}

export interface CpuInfo {
  name: string;
  manufacturer: string;
  cores: number;
  threads: number;
  maxClockMhz: number;
}

export interface GpuInfo {
  name: string;
  manufacturer: string;
  driverVersion: string;
  driverDate: string;
  vramMb: number;
  pnpDeviceId: string;
  status: number;
}

export interface DiskInfo {
  model: string;
  sizeGb: number;
  mediaType: string;
  interfaceType: string;
  serialNumber: string;
}

export interface NetworkAdapter {
  name: string;
  manufacturer: string;
  macAddress: string;
  connectionStatus: string;
  speedMbps: number;
}

export interface AudioDevice {
  name: string;
  manufacturer: string;
  status: string;
}

export interface MotherboardInfo {
  manufacturer: string;
  product: string;
  serialNumber: string;
  biosVersion: string;
}

export interface HardwareSummary {
  system: SystemInfo;
  cpu: CpuInfo;
  gpus: GpuInfo[];
  disks: DiskInfo[];
  networkAdapters: NetworkAdapter[];
  audioDevices: AudioDevice[];
  motherboard: MotherboardInfo;
}

export interface DriverIssue {
  deviceName: string;
  deviceId: string;
  hardwareId: string[];
  errorCode: number;
  errorDescription: string;
}
