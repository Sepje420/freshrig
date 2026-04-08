import { Wifi } from "lucide-react";
import { HardwareCard } from "./HardwareCard";
import type { NetworkAdapter } from "../../types/hardware";

interface NetworkCardProps {
  adapters: NetworkAdapter[];
}

export function NetworkCard({ adapters }: NetworkCardProps) {
  const hasDisconnected = adapters.some(
    (a) => a.connectionStatus !== "Connected" && a.connectionStatus !== "Unknown"
  );

  return (
    <HardwareCard title="Network" icon={Wifi} status={hasDisconnected ? "warning" : "good"}>
      <div className="space-y-3">
        {adapters.map((adapter, i) => (
          <div key={i} className="space-y-1.5">
            <p className="text-sm text-text-primary truncate" title={adapter.name}>{adapter.name}</p>

            <div className="grid grid-cols-2 gap-x-4 gap-y-1 text-xs">
              <div className="flex justify-between">
                <span className="text-text-muted">Status</span>
                <span
                  className={`font-medium ${
                    adapter.connectionStatus === "Connected"
                      ? "text-success"
                      : "text-text-secondary"
                  }`}
                >
                  {adapter.connectionStatus}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-text-muted">Speed</span>
                <span className="text-text-secondary font-mono">
                  {adapter.speedMbps > 0 ? `${adapter.speedMbps} Mbps` : "Unknown"}
                </span>
              </div>
              <div className="flex justify-between col-span-2">
                <span className="text-text-muted">MAC</span>
                <span className="text-text-secondary font-mono">{adapter.macAddress}</span>
              </div>
            </div>

            {i < adapters.length - 1 && <div className="border-t border-border/50 mt-2" />}
          </div>
        ))}
        {adapters.length === 0 && (
          <p className="text-xs text-text-muted">No physical adapters detected</p>
        )}
      </div>
    </HardwareCard>
  );
}
