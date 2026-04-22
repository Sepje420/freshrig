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
    <div className="relative overflow-hidden rounded-xl border border-[var(--border)] bg-[var(--bg-card)] shadow-[inset_0_1px_0_rgba(255,255,255,0.04)] animate-fade-in">
      {/* 3px accent left stripe */}
      <span
        aria-hidden="true"
        className="absolute left-0 top-4 bottom-4 w-[3px] rounded-full bg-[var(--accent)]"
      />
      {/* Subtle accent glow */}
      <div className="absolute inset-0 bg-gradient-to-br from-[var(--accent-subtle)] to-transparent pointer-events-none opacity-60" />

      <div className="relative px-6 py-5">
        <div className="flex items-center gap-3 mb-4">
          <div className="flex items-center justify-center w-10 h-10 rounded-lg bg-[var(--accent-subtle)] ring-1 ring-[var(--accent-ring)]">
            <Monitor className="w-5 h-5 text-[var(--accent)]" />
          </div>
          <div>
            <h2 className="text-lg font-semibold text-[var(--text-primary)]">System Overview</h2>
            <p className="text-xs text-[var(--text-muted)]">{system.hostname}</p>
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
      <p className="text-[10px] font-medium text-[var(--text-muted)] uppercase tracking-wide mb-1">{label}</p>
      <p className="text-sm font-semibold text-[var(--text-primary)] truncate tabular-nums" title={value}>{value}</p>
      <p className="text-xs text-[var(--text-secondary)] mt-0.5">{sub}</p>
    </div>
  );
}
