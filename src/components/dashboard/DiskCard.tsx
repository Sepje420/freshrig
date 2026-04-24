import { HardDrive } from "lucide-react";
import { HardwareCard } from "./HardwareCard";
import type { DiskInfo } from "../../types/hardware";

interface DiskCardProps {
  disks: DiskInfo[];
}

const mediaTypeStyles: Record<string, string> = {
  NVMe: "bg-accent-muted text-accent",
  SSD: "bg-blue-500/20 text-blue-400",
  HDD: "bg-bg-tertiary text-text-secondary",
};

export function DiskCard({ disks }: DiskCardProps) {
  return (
    <HardwareCard title="Storage" icon={HardDrive} status="good">
      <div className="space-y-3">
        {disks.map((disk, i) => (
          <div key={i} className="space-y-1.5">
            <div className="flex items-center gap-2">
              <span
                className={`text-[10px] font-semibold px-1.5 py-0.5 rounded ${
                  mediaTypeStyles[disk.mediaType] ?? mediaTypeStyles.HDD
                }`}
              >
                {disk.mediaType}
              </span>
              <p className="text-sm text-text-primary truncate" title={disk.model}>{disk.model}</p>
            </div>

            <div className="grid grid-cols-2 gap-x-4 gap-y-1 text-xs">
              <div className="flex justify-between">
                <span className="text-text-muted">Size</span>
                <span className="text-text-secondary font-mono tabular-nums">{disk.sizeGb.toFixed(0)} GB</span>
              </div>
              <div className="flex justify-between">
                <span className="text-text-muted">Interface</span>
                <span className="text-text-secondary font-mono">{disk.interfaceType}</span>
              </div>
            </div>

            {i < disks.length - 1 && <div className="border-t border-border/50 mt-2" />}
          </div>
        ))}
        {disks.length === 0 && (
          <p className="text-xs text-text-muted">No disks detected</p>
        )}
      </div>
    </HardwareCard>
  );
}
