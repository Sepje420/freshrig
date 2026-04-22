import { useState } from "react";
import { ExternalLink, Download, Loader2, Check } from "lucide-react";
import { openUrl } from "@tauri-apps/plugin-opener";
import { toast } from "sonner";
import { useDriverStore } from "../../stores/driverStore";
import type { DriverRecommendation, DriverCategory, DriverStatus } from "../../types/drivers";

interface DriverCardProps {
  recommendation: DriverRecommendation;
}

const categoryLabels: Record<DriverCategory, string> = {
  Gpu: "GPU",
  Chipset: "Chipset",
  Network: "Network",
  Audio: "Audio",
  Other: "Other",
};

const categoryColors: Record<DriverCategory, string> = {
  Gpu: "bg-purple-500/20 text-purple-400",
  Chipset: "bg-blue-500/20 text-blue-400",
  Network: "bg-cyan-500/20 text-cyan-400",
  Audio: "bg-amber-500/20 text-amber-400",
  Other: "bg-bg-tertiary text-text-secondary",
};

const statusConfig: Record<DriverStatus, { label: string; color: string }> = {
  UpToDate: { label: "Up to Date", color: "bg-success/20 text-success" },
  UpdateAvailable: { label: "Update Available", color: "bg-warning/20 text-warning" },
  Missing: { label: "Missing", color: "bg-error/20 text-error" },
  Unknown: { label: "Check Manually", color: "bg-info/20 text-info" },
};

function getVendorColor(vendor: string): string {
  const lower = vendor.toLowerCase();
  if (lower.includes("nvidia")) return "#76b900";
  if (lower.includes("amd")) return "#ed1c24";
  if (lower.includes("intel")) return "#0071c5";
  return "#00d4aa";
}

export function DriverCard({ recommendation: rec }: DriverCardProps) {
  const vendorColor = getVendorColor(rec.vendor);
  const statusCfg = statusConfig[rec.status];
  const { installingDriverId, installedDriverIds, installDriver } = useDriverStore();
  const [hashFailureUrl, setHashFailureUrl] = useState<string | null>(null);

  const wingetId = rec.installAction.type === "Winget" ? rec.installAction.value : null;
  const isInstalling = !!wingetId && installingDriverId === wingetId;
  const isInstalled = !!wingetId && installedDriverIds.has(wingetId);

  const handleWingetInstall = async () => {
    if (!wingetId) return;
    setHashFailureUrl(null);
    const result = await installDriver(wingetId);
    if (result.success) {
      toast.success(`${rec.vendor} driver tool installed`);
    } else {
      toast.error(result.message || "Driver install failed");
      if (/hash mismatch/i.test(result.message) && rec.downloadPage) {
        setHashFailureUrl(rec.downloadPage);
      }
    }
  };

  const handleDirectDownload = async () => {
    if (rec.installAction.type === "DirectDownload") {
      await openUrl(rec.installAction.value);
    }
  };

  const handleSupport = async () => {
    if (rec.downloadPage) {
      await openUrl(rec.downloadPage);
    }
  };

  const handleHashFallback = async () => {
    if (hashFailureUrl) {
      await openUrl(hashFailureUrl);
    }
  };

  return (
    <div className="bg-bg-card rounded-lg border border-border shadow-card hover:bg-bg-card-hover hover:border-border-hover hover:shadow-elevated transition-all duration-200 animate-fade-in">
      <div className="p-5 space-y-3">
        {/* Header row */}
        <div className="flex items-start justify-between gap-2">
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 mb-1.5 flex-wrap">
              <span className={`text-[10px] font-semibold px-1.5 py-0.5 rounded ${categoryColors[rec.category]}`}>
                {categoryLabels[rec.category]}
              </span>
              <span className={`text-[10px] font-semibold px-1.5 py-0.5 rounded ${statusCfg.color}`}>
                {statusCfg.label}
              </span>
              {isInstalled && (
                <span className="text-[10px] font-semibold px-1.5 py-0.5 rounded bg-success/20 text-success flex items-center gap-1">
                  <Check className="w-3 h-3" />
                  Installed
                </span>
              )}
            </div>
            <h3 className="text-sm font-semibold text-text-primary truncate" title={rec.deviceName}>
              {rec.deviceName}
            </h3>
          </div>
        </div>

        {/* Vendor */}
        <div className="flex items-center gap-2">
          <div className="w-2 h-2 rounded-full" style={{ backgroundColor: vendorColor }} />
          <span className="text-xs font-medium" style={{ color: vendorColor }}>
            {rec.vendor}
          </span>
        </div>

        {/* Driver info */}
        <div className="space-y-1 text-xs">
          {rec.currentVersion && (
            <div className="flex justify-between">
              <span className="text-text-muted">Version</span>
              <span className="text-text-secondary font-mono">{rec.currentVersion}</span>
            </div>
          )}
          {rec.currentDate && (
            <div className="flex justify-between">
              <span className="text-text-muted">Date</span>
              <span className="text-text-secondary font-mono">{rec.currentDate}</span>
            </div>
          )}
        </div>

        {/* Actions */}
        <div className="flex gap-2 pt-1 flex-wrap">
          {rec.installAction.type === "Winget" ? (
            <button
              onClick={handleWingetInstall}
              disabled={isInstalling || isInstalled}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-accent text-bg-primary text-xs font-medium hover:bg-accent-hover transition-colors disabled:opacity-60 disabled:cursor-not-allowed"
              title={`winget install ${rec.installAction.value}`}
            >
              {isInstalling ? (
                <>
                  <Loader2 className="w-3.5 h-3.5 animate-spin" />
                  Installing...
                </>
              ) : isInstalled ? (
                <>
                  <Check className="w-3.5 h-3.5" />
                  Installed
                </>
              ) : (
                <>
                  <Download className="w-3.5 h-3.5" />
                  {rec.installLabel}
                </>
              )}
            </button>
          ) : (
            <button
              onClick={handleDirectDownload}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-md border border-border text-text-secondary text-xs font-medium hover:bg-bg-tertiary hover:text-text-primary transition-colors"
            >
              <ExternalLink className="w-3.5 h-3.5" />
              {rec.installLabel}
            </button>
          )}
          {hashFailureUrl && (
            <button
              onClick={handleHashFallback}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-md border border-warning/30 bg-warning/10 text-warning text-xs font-medium hover:bg-warning/20 transition-colors"
            >
              <ExternalLink className="w-3.5 h-3.5" />
              Open download page
            </button>
          )}
          {rec.installAction.type === "Winget" &&
            rec.downloadPage &&
            rec.downloadPage !== hashFailureUrl && (
              <button
                onClick={handleSupport}
                className="flex items-center gap-1.5 px-3 py-1.5 rounded-md border border-border text-text-secondary text-xs font-medium hover:bg-bg-tertiary hover:text-text-primary transition-colors"
              >
                <ExternalLink className="w-3.5 h-3.5" />
                Support
              </button>
            )}
        </div>
      </div>
    </div>
  );
}
