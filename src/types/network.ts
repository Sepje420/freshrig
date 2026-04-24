export interface NetworkInterface {
  name: string;
  index: number;
}

export interface WifiProfile {
  ssid: string;
  password: string | null;
  authType: string;
}

export interface DnsPreset {
  name: string;
  primary: string;
  secondary: string;
  description: string;
}

export const DNS_PRESETS: DnsPreset[] = [
  {
    name: "Cloudflare",
    primary: "1.1.1.1",
    secondary: "1.0.0.1",
    description: "Fast & privacy-focused",
  },
  {
    name: "Google",
    primary: "8.8.8.8",
    secondary: "8.8.4.4",
    description: "Reliable global DNS",
  },
  {
    name: "Quad9",
    primary: "9.9.9.9",
    secondary: "149.112.112.112",
    description: "Malware filtering",
  },
  {
    name: "AdGuard",
    primary: "94.140.14.14",
    secondary: "94.140.15.15",
    description: "Ads & trackers blocked",
  },
  {
    name: "OpenDNS",
    primary: "208.67.222.222",
    secondary: "208.67.220.220",
    description: "Family-safe filtering",
  },
];
