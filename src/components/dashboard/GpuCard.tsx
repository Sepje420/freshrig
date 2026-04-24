import { MonitorDot } from "lucide-react";
import { HardwareCard } from "./HardwareCard";
import type { GpuInfo } from "../../types/hardware";

interface GpuCardProps {
  gpus: GpuInfo[];
}

function getVendorStyle(manufacturer: string): { label: string; className: string } {
  const lower = manufacturer.toLowerCase();
  if (lower.includes("nvidia")) {
    return { label: "NVIDIA", className: "bg-green-500/20 text-green-400" };
  }
  if (lower.includes("amd") || lower.includes("advanced micro")) {
    return { label: "AMD", className: "bg-red-500/20 text-red-400" };
  }
  if (lower.includes("intel")) {
    return { label: "Intel", className: "bg-blue-500/20 text-blue-400" };
  }
  return { label: manufacturer, className: "bg-bg-tertiary text-text-secondary" };
}

function getErrorDescription(code: number): string {
  switch (code) {
    case 1: return "Device not configured correctly";
    case 3: return "Driver may be corrupted";
    case 10: return "Device cannot start";
    case 22: return "Device is disabled";
    case 28: return "Drivers not installed";
    case 31: return "Device not working properly";
    default: return `Unknown issue (code ${code})`;
  }
}

export function GpuCard({ gpus }: GpuCardProps) {
  const hasIssue = gpus.some((gpu) => gpu.status !== 0);
  const status = hasIssue ? "warning" : "good";

  return (
    <HardwareCard title="Graphics" icon={MonitorDot} status={status}>
      <div className="space-y-3">
        {gpus.map((gpu, i) => {
          const vendor = getVendorStyle(gpu.manufacturer);

          return (
            <div key={i} className="space-y-1.5">
              <div className="flex items-center gap-2">
                <span className={`text-[10px] font-semibold px-1.5 py-0.5 rounded ${vendor.className}`}>
                  {vendor.label}
                </span>
                <p className="text-sm text-text-primary truncate" title={gpu.name}>{gpu.name}</p>
              </div>

              <div className="grid grid-cols-2 gap-x-4 gap-y-1 text-xs">
                <DataRow label="VRAM" value={`${gpu.vramMb} MB`} />
                <DataRow label="Driver" value={gpu.driverVersion} />
                <DataRow label="Driver Date" value={gpu.driverDate} />
              </div>

              {gpu.status !== 0 && (
                <div className="flex items-center gap-1.5 px-2 py-1.5 rounded bg-warning/10 border border-warning/20">
                  <div className="w-1.5 h-1.5 rounded-full bg-warning" />
                  <p className="text-xs text-warning">{getErrorDescription(gpu.status)}</p>
                </div>
              )}

              {i < gpus.length - 1 && <div className="border-t border-border/50 mt-2" />}
            </div>
          );
        })}
        {gpus.length === 0 && (
          <p className="text-xs text-text-muted">No GPU detected</p>
        )}
      </div>
    </HardwareCard>
  );
}

function DataRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex justify-between">
      <span className="text-text-muted">{label}</span>
      <span className="text-text-secondary font-mono tabular-nums">{value}</span>
    </div>
  );
}
