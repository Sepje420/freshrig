import { useState, useEffect } from "react";
import { X, Loader2, Check, AlertTriangle, ShieldAlert } from "lucide-react";
import type { DebloatTweak, DebloatResult, TweakTier } from "../../types/debloat";
import { useDebloatStore } from "../../stores/debloatStore";

interface ApplyConfirmDialogProps {
  selectedTweaks: DebloatTweak[];
  dryRunResults: DebloatResult[] | null;
  onClose: () => void;
}

const tierLabels: Record<TweakTier, string> = {
  safe: "Safe",
  moderate: "Moderate",
  risky: "Risky",
};

const tierColors: Record<TweakTier, string> = {
  safe: "text-success",
  moderate: "text-warning",
  risky: "text-error",
};

export function ApplyConfirmDialog({
  selectedTweaks,
  dryRunResults,
  onClose,
}: ApplyConfirmDialogProps) {
  const { createRestorePoint, applySelected, results } = useDebloatStore();
  const [step, setStep] = useState<"confirm" | "restore" | "applying" | "done">("confirm");
  const [confirmed, setConfirmed] = useState(false);
  const [restoreError, setRestoreError] = useState<string | null>(null);

  const hasRisky = selectedTweaks.some((t) => t.tier === "risky");
  const grouped = {
    safe: selectedTweaks.filter((t) => t.tier === "safe"),
    moderate: selectedTweaks.filter((t) => t.tier === "moderate"),
    risky: selectedTweaks.filter((t) => t.tier === "risky"),
  };

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [onClose]);

  const handleApply = async () => {
    setStep("restore");
    setRestoreError(null);
    const ok = await createRestorePoint();
    if (!ok) {
      setRestoreError(
        "Could not create restore point. Run FreshRig as administrator and try again."
      );
      return;
    }
    setStep("applying");
    await applySelected(false);
    setStep("done");
  };

  const successCount = results.filter((r) => r.success).length;
  const failCount = results.filter((r) => !r.success).length;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/60"
      onClick={onClose}
    >
      <div
        className="bg-bg-elevated border border-border rounded-xl shadow-elevated w-full max-w-lg mx-4 max-h-[80vh] flex flex-col animate-fade-in"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-border shrink-0">
          <div className="flex items-center gap-2">
            <ShieldAlert className="w-5 h-5 text-accent" />
            <h2 className="text-lg font-semibold text-text-primary">
              {step === "done" ? "Optimization Complete" : "Apply Changes"}
            </h2>
          </div>
          <button
            onClick={onClose}
            className="p-1.5 rounded-md hover:bg-bg-tertiary text-text-muted hover:text-text-primary transition-colors"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Body */}
        <div className="flex-1 overflow-y-auto px-6 py-4 space-y-4">
          {step === "confirm" && (
            <>
              <p className="text-sm text-text-secondary">
                The following changes will be applied to your system:
              </p>

              {(["safe", "moderate", "risky"] as TweakTier[]).map((tier) => {
                const items = grouped[tier];
                if (items.length === 0) return null;
                return (
                  <div key={tier}>
                    <h3
                      className={`text-xs font-semibold uppercase tracking-wider mb-2 ${tierColors[tier]}`}
                    >
                      {tierLabels[tier]} ({items.length})
                    </h3>
                    <div className="space-y-1">
                      {items.map((t) => (
                        <div key={t.id} className="flex items-start gap-2 text-sm">
                          <span className="text-text-muted">•</span>
                          <div>
                            <span className="text-text-primary">{t.name}</span>
                            {dryRunResults && (
                              <span className="text-xs text-text-muted ml-2">
                                — {dryRunResults.find((r) => r.tweakId === t.id)?.message}
                              </span>
                            )}
                            {t.warning && (
                              <p className="text-[11px] text-warning flex items-center gap-1 mt-0.5">
                                <AlertTriangle className="w-3 h-3" />
                                {t.warning}
                              </p>
                            )}
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                );
              })}

              {hasRisky && (
                <label className="flex items-start gap-2 pt-2 border-t border-border">
                  <input
                    type="checkbox"
                    checked={confirmed}
                    onChange={(e) => setConfirmed(e.target.checked)}
                    className="mt-0.5 accent-accent"
                  />
                  <span className="text-xs text-text-secondary">
                    I understand these changes modify Windows settings and some may be hard to
                    reverse
                  </span>
                </label>
              )}
            </>
          )}

          {step === "restore" && (
            <div className="flex flex-col items-center py-8 gap-3">
              {restoreError ? (
                <>
                  <AlertTriangle className="w-8 h-8 text-error" />
                  <p className="text-sm text-error text-center">{restoreError}</p>
                </>
              ) : (
                <>
                  <Loader2 className="w-8 h-8 text-accent animate-spin" />
                  <p className="text-sm text-text-secondary">Creating system restore point...</p>
                </>
              )}
            </div>
          )}

          {step === "applying" && (
            <div className="space-y-2">
              <div className="flex items-center gap-2 mb-3">
                <Loader2 className="w-4 h-4 text-accent animate-spin" />
                <p className="text-sm text-text-secondary">Applying changes...</p>
              </div>
              {results.map((r) => (
                <div key={r.tweakId} className="flex items-center gap-2 text-sm">
                  {r.success ? (
                    <Check className="w-3.5 h-3.5 text-success shrink-0" />
                  ) : (
                    <X className="w-3.5 h-3.5 text-error shrink-0" />
                  )}
                  <span className={r.success ? "text-text-secondary" : "text-error"}>
                    {r.message}
                  </span>
                </div>
              ))}
            </div>
          )}

          {step === "done" && (
            <div className="space-y-3">
              <div className="flex items-center gap-3 py-4 justify-center">
                <Check className="w-8 h-8 text-success" />
                <div>
                  <p className="text-sm font-semibold text-text-primary">
                    {successCount} tweak{successCount !== 1 ? "s" : ""} applied
                  </p>
                  {failCount > 0 && (
                    <p className="text-xs text-error">
                      {failCount} failed — check %APPDATA%\com.freshrig.app\debloat-log.txt
                    </p>
                  )}
                </div>
              </div>
              {results.map((r) => (
                <div key={r.tweakId} className="flex items-center gap-2 text-sm">
                  {r.success ? (
                    <Check className="w-3.5 h-3.5 text-success shrink-0" />
                  ) : (
                    <X className="w-3.5 h-3.5 text-error shrink-0" />
                  )}
                  <span className={r.success ? "text-text-secondary" : "text-error"}>
                    {r.message}
                  </span>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="px-6 py-4 border-t border-border shrink-0 flex items-center justify-end gap-2">
          {step === "confirm" && (
            <>
              <button
                onClick={onClose}
                className="px-4 py-2 rounded-lg text-sm text-text-secondary hover:text-text-primary hover:bg-bg-tertiary transition-colors"
              >
                Cancel
              </button>
              <button
                onClick={handleApply}
                disabled={hasRisky && !confirmed}
                className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all ${
                  hasRisky && !confirmed
                    ? "bg-bg-tertiary text-text-muted cursor-not-allowed"
                    : "bg-accent text-bg-primary hover:bg-accent-hover"
                }`}
              >
                Apply {selectedTweaks.length} Change{selectedTweaks.length !== 1 ? "s" : ""}
              </button>
            </>
          )}
          {step === "restore" && restoreError && (
            <button
              onClick={onClose}
              className="px-4 py-2 rounded-lg text-sm text-text-secondary hover:text-text-primary hover:bg-bg-tertiary transition-colors"
            >
              Close
            </button>
          )}
          {step === "done" && (
            <button
              onClick={onClose}
              className="px-4 py-2 rounded-lg text-sm font-semibold bg-accent text-bg-primary hover:bg-accent-hover transition-colors"
            >
              Done
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
