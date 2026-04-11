import { Download, X, Loader2 } from "lucide-react";
import { useUpdateStore } from "../../stores/updateStore";

export function UpdateBanner() {
  const { status, newVersion, progress, total, dismiss, downloadAndInstall } = useUpdateStore();

  if (status === "available" || status === "downloading" || status === "installing") {
    const progressPercent = total > 0 ? Math.round((progress / total) * 100) : 0;
    const downloadedMB = (progress / 1024 / 1024).toFixed(1);
    const totalMB = (total / 1024 / 1024).toFixed(1);

    return (
      <div className="relative bg-accent/10 border-b border-accent/20 px-4 py-2 animate-fade-in">
        <div className="flex items-center justify-center gap-3 text-sm">
          {status === "available" && (
            <>
              <Download className="w-4 h-4 text-accent" />
              <span className="text-text-primary">
                FreshRig <span className="font-semibold text-accent">v{newVersion}</span> is
                available
              </span>
              <button
                onClick={downloadAndInstall}
                className="px-3 py-1 rounded-md text-xs font-semibold bg-accent text-bg-primary hover:bg-accent-hover transition-colors"
              >
                Update Now
              </button>
              <button
                onClick={dismiss}
                className="p-1 rounded hover:bg-bg-tertiary text-text-muted hover:text-text-primary transition-colors"
              >
                <X className="w-3.5 h-3.5" />
              </button>
            </>
          )}
          {status === "downloading" && (
            <>
              <Loader2 className="w-4 h-4 text-accent animate-spin" />
              <span className="text-text-secondary">
                Downloading update... {progressPercent}%{" "}
                {total > 0 && (
                  <span className="text-text-muted">
                    ({downloadedMB} / {totalMB} MB)
                  </span>
                )}
              </span>
              <div className="flex-1 max-w-xs h-1.5 bg-bg-tertiary rounded-full overflow-hidden">
                <div
                  className="h-full bg-accent rounded-full transition-all duration-300"
                  style={{ width: `${progressPercent}%` }}
                />
              </div>
            </>
          )}
          {status === "installing" && (
            <>
              <Loader2 className="w-4 h-4 text-accent animate-spin" />
              <span className="text-text-secondary">Installing update... App will restart.</span>
            </>
          )}
        </div>
      </div>
    );
  }

  return null;
}
