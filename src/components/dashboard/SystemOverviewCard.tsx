import { Monitor } from "lucide-react";
import type { HardwareSummary } from "../../types/hardware";

interface SystemOverviewCardProps {
  summary: HardwareSummary;
}

function formatUptime(seconds: number): string {
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  if (days > 0) {
    return `${days} day${days !== 1 ? "s" : ""}, ${hours} hour${hours !== 1 ? "s" : ""}`;
  }
  const minutes = Math.floor((seconds % 3600) / 60);
  return `${hours} hour${hours !== 1 ? "s" : ""}, ${minutes} min`;
}

export function SystemOverviewCard({ summary }: SystemOverviewCardProps) {
  const { system, cpu } = summary;

  return (
    <div className="relative overflow-hidden rounded-xl border border-accent-muted bg-gradient-to-br from-bg-card to-bg-secondary shadow-elevated animate-fade-in">
      {/* Accent glow effect */}
      <div className="absolute inset-0 bg-gradient-to-br from-accent-glow to-transparent pointer-events-none" />

      <div className="relative px-6 py-5">
        <div className="flex items-center gap-3 mb-4">
          <div className="flex items-center justify-center w-10 h-10 rounded-lg bg-accent-muted">
            <Monitor className="w-5 h-5 text-accent" />
          </div>
          <div>
            <h2 className="text-lg font-semibold text-text-primary">System Overview</h2>
            <p className="text-xs text-text-muted">{system.hostname}</p>
          </div>
        </div>

        <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
          <InfoBlock label="Operating System" value={system.osVersion} sub={`Build ${system.osBuild}`} />
          <InfoBlock label="Processor" value={cpu.name} sub={`${cpu.cores}C/${cpu.threads}T @ ${cpu.maxClockMhz} MHz`} />
          <InfoBlock label="Memory" value={`${system.totalRamGb.toFixed(1)} GB`} sub={system.architecture} />
          <InfoBlock label="Uptime" value={formatUptime(system.uptimeSeconds)} sub="Since last boot" />
        </div>
      </div>
    </div>
  );
}

function InfoBlock({ label, value, sub }: { label: string; value: string; sub: string }) {
  return (
    <div>
      <p className="text-[11px] font-medium text-text-muted uppercase tracking-wider mb-1">{label}</p>
      <p className="text-sm font-medium text-text-primary truncate" title={value}>{value}</p>
      <p className="text-xs text-text-secondary mt-0.5">{sub}</p>
    </div>
  );
}
