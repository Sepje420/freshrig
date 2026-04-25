import { useEffect, useMemo, useState } from "react";
import { Sparkles, AlertTriangle, Eye, Crown, Check, X, ChevronDown, ChevronUp } from "lucide-react";
import { useDebloatStore } from "../../stores/debloatStore";
import { useLicenseStore } from "../../stores/licenseStore";
import { TweakCard } from "./TweakCard";
import { ApplyConfirmDialog } from "./ApplyConfirmDialog";
import { SkeletonRow } from "../ui/Skeleton";
import type { TweakTier, TweakCategory, DebloatResult } from "../../types/debloat";

const TIER_FILTERS: { value: TweakTier | "all"; label: string; color: string; tooltip: string }[] = [
  { value: "all", label: "All", color: "", tooltip: "" },
  { value: "safe", label: "Safe", color: "text-success", tooltip: "Low-risk, easily reversible changes" },
  { value: "moderate", label: "Moderate", color: "text-warning", tooltip: "Changes some Windows defaults — review each item before selecting" },
  { value: "expert", label: "Expert", color: "text-error", tooltip: "Advanced tweaks that can break functionality — read descriptions carefully" },
];

const CATEGORY_FILTERS: { value: TweakCategory | "all"; label: string }[] = [
  { value: "all", label: "All" },
  { value: "privacy", label: "Privacy" },
  { value: "bloatware", label: "Bloatware" },
  { value: "performance", label: "Performance" },
  { value: "appearance", label: "Appearance" },
];

export function OptimizePage() {
  const isPro = useLicenseStore((s) => s.isPro());
  const {
    tweaks,
    selectedIds,
    activeTier,
    activeCategory,
    isApplying,
    loading,
    fetchTweaks,
    toggleTweak,
    clearSelection,
    applySelected,
    setActiveTier,
    setActiveCategory,
    lastApplyResults,
    lastApplyTimestamp,
    clearLastResults,
  } = useDebloatStore();

  const [showConfirm, setShowConfirm] = useState(false);
  const [resultsExpanded, setResultsExpanded] = useState(false);
  const [dryRunResults, setDryRunResults] = useState<DebloatResult[] | null>(null);

  useEffect(() => {
    fetchTweaks();
  }, [fetchTweaks]);

  const filteredTweaks = useMemo(() => {
    return tweaks.filter((t) => {
      const matchesTier = activeTier === "all" || t.tier === activeTier;
      const matchesCat = activeCategory === "all" || t.category === activeCategory;
      return matchesTier && matchesCat;
    });
  }, [tweaks, activeTier, activeCategory]);

  const tierCounts = useMemo(() => {
    const counts = { safe: 0, moderate: 0, expert: 0 };
    for (const t of tweaks) {
      if (t.tier in counts) counts[t.tier as keyof typeof counts]++;
    }
    return counts;
  }, [tweaks]);

  const selectedTweaks = tweaks.filter((t) => selectedIds.has(t.id));

  const handlePreview = async () => {
    const results = await applySelected(true);
    setDryRunResults(results);
    setShowConfirm(true);
  };

  const handleApplyClick = () => {
    setDryRunResults(null);
    setShowConfirm(true);
  };

  return (
    <div className="space-y-6 pb-20">
      {/* Header */}
      <div className="flex items-center gap-3">
        <div className="flex items-center justify-center w-10 h-10 rounded-lg bg-accent-muted">
          <Sparkles className="w-5 h-5 text-accent" />
        </div>
        <div>
          <h1 className="text-2xl font-bold text-text-primary">Optimize Windows</h1>
          <p className="text-sm text-text-secondary mt-0.5">
            Remove bloatware, disable telemetry, and improve privacy
          </p>
        </div>
      </div>

      {/* Warning banner */}
      <div className="flex items-center gap-3 px-4 py-3 rounded-lg bg-warning/10 border border-warning/20">
        <AlertTriangle className="w-5 h-5 text-warning shrink-0" />
        <p className="text-sm text-warning">
          These tweaks modify Windows registry and settings. A restore point is created
          automatically before anything changes.
        </p>
      </div>

      {/* Last optimization results */}
      {lastApplyResults && lastApplyTimestamp && (() => {
        const successCount = lastApplyResults.filter((r) => r.success).length;
        const failCount = lastApplyResults.filter((r) => !r.success).length;
        const allSuccess = failCount === 0;
        const ExpandIcon = resultsExpanded ? ChevronUp : ChevronDown;
        return (
          <div
            className={`rounded-lg border animate-fade-in ${
              allSuccess ? "border-success/20 bg-success/5" : "border-warning/20 bg-warning/5"
            }`}
            style={{ borderLeftWidth: "3px", borderLeftColor: allSuccess ? "#22c55e" : "#f59e0b" }}
          >
            <div className="flex items-center justify-between px-4 py-3">
              <div className="flex items-center gap-2 text-sm">
                {allSuccess ? (
                  <Check className="w-4 h-4 text-success shrink-0" />
                ) : (
                  <AlertTriangle className="w-4 h-4 text-warning shrink-0" />
                )}
                <span className="text-text-secondary">
                  Last optimization: <span className="text-text-muted">{lastApplyTimestamp}</span>
                  {" — "}
                  <span className="text-success font-medium">{successCount} applied</span>
                  {failCount > 0 && (
                    <>, <span className="text-error font-medium">{failCount} failed</span></>
                  )}
                </span>
              </div>
              <div className="flex items-center gap-1">
                <button
                  onClick={() => setResultsExpanded((v) => !v)}
                  className="p-1 rounded hover:bg-bg-tertiary text-text-muted hover:text-text-primary transition-colors"
                >
                  <ExpandIcon className="w-4 h-4" />
                </button>
                <button
                  onClick={clearLastResults}
                  className="p-1 rounded hover:bg-bg-tertiary text-text-muted hover:text-text-primary transition-colors"
                >
                  <X className="w-4 h-4" />
                </button>
              </div>
            </div>
            {resultsExpanded && (
              <div className="px-4 pb-3 space-y-1 border-t border-border/50 pt-2">
                {lastApplyResults.map((r) => (
                  <div key={r.tweakId} className="flex items-center gap-2 text-xs">
                    {r.success ? (
                      <Check className="w-3 h-3 text-success shrink-0" />
                    ) : (
                      <X className="w-3 h-3 text-error shrink-0" />
                    )}
                    <span className={r.success ? "text-text-secondary" : "text-error"}>
                      {r.message}
                    </span>
                  </div>
                ))}
              </div>
            )}
          </div>
        );
      })()}

      {/* Tier filter tabs */}
      <div className="flex items-center gap-1 bg-bg-card border border-border rounded-lg p-1">
        {TIER_FILTERS.map((filter) => {
          const isActive = activeTier === filter.value;
          const count =
            filter.value === "all"
              ? tweaks.length
              : tierCounts[filter.value as keyof typeof tierCounts];
          return (
            <button
              key={filter.value}
              onClick={() => setActiveTier(filter.value)}
              title={filter.tooltip || undefined}
              className={`flex items-center gap-1.5 px-3 py-1.5 rounded-md text-sm font-medium transition-all ${
                isActive
                  ? "bg-accent-muted text-accent"
                  : "text-text-secondary hover:text-text-primary hover:bg-bg-tertiary"
              }`}
            >
              <span className={!isActive ? filter.color : ""}>{filter.label}</span>
              {!isPro && (filter.value === "moderate" || filter.value === "expert") && (
                <Crown className="w-3 h-3 text-amber-500" />
              )}
              <span className="text-[10px] px-1.5 py-0.5 rounded-full bg-bg-tertiary text-text-muted">
                {count}
              </span>
            </button>
          );
        })}
      </div>

      {/* Category pills */}
      <div className="flex items-center gap-2 overflow-x-auto scrollbar-none">
        {CATEGORY_FILTERS.map((cat) => {
          const isActive = activeCategory === cat.value;
          return (
            <button
              key={cat.value}
              onClick={() => setActiveCategory(cat.value)}
              className={`shrink-0 px-3 py-1.5 rounded-full text-xs font-medium transition-all ${
                isActive
                  ? "bg-accent text-bg-primary"
                  : "bg-bg-tertiary text-text-secondary hover:text-text-primary"
              }`}
            >
              {cat.label}
            </button>
          );
        })}
      </div>

      {/* Loading */}
      {loading && (
        <div className="bg-bg-card border border-border rounded-lg divide-y divide-border" aria-busy="true" aria-label="Loading optimizations">
          {Array.from({ length: 8 }).map((_, i) => (
            <SkeletonRow key={i} />
          ))}
        </div>
      )}

      {/* Tweak list */}
      {!loading && (
        <div className="grid grid-cols-1 gap-2">
          {filteredTweaks.map((tweak) => (
            <TweakCard
              key={tweak.id}
              tweak={tweak}
              selected={selectedIds.has(tweak.id)}
              onToggle={() => toggleTweak(tweak.id)}
            />
          ))}
        </div>
      )}

      {/* Empty */}
      {!loading && filteredTweaks.length === 0 && (
        <div className="flex flex-col items-center justify-center py-16 animate-fade-in">
          <Sparkles className="w-12 h-12 text-text-muted mb-4" />
          <h3 className="text-lg font-semibold text-text-primary mb-1">No tweaks found</h3>
          <p className="text-sm text-text-secondary">Nothing matches those filters.</p>
        </div>
      )}

      {/* Bottom action bar */}
      {selectedIds.size > 0 && (
        <div className="fixed bottom-0 left-[280px] right-0 bg-bg-elevated border-t border-border px-8 py-4 flex items-center justify-between animate-fade-in z-40">
          <div className="flex items-center gap-3">
            <span className="text-sm text-text-secondary">
              {selectedIds.size} tweak{selectedIds.size !== 1 ? "s" : ""} selected
            </span>
            <button
              onClick={clearSelection}
              className="text-xs text-text-muted hover:text-text-primary transition-colors"
            >
              Clear
            </button>
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={handlePreview}
              disabled={isApplying}
              className="flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium border border-border text-text-secondary hover:text-text-primary hover:bg-bg-tertiary transition-colors"
            >
              <Eye className="w-4 h-4" />
              Preview Changes
            </button>
            <button
              onClick={handleApplyClick}
              disabled={isApplying}
              className="px-4 py-2 rounded-lg text-sm font-semibold bg-accent text-bg-primary hover:bg-accent-hover shadow-[0_0_20px_var(--accent-ring)] transition-all"
            >
              Apply Changes
            </button>
          </div>
        </div>
      )}

      {/* Confirm dialog */}
      {showConfirm && (
        <ApplyConfirmDialog
          selectedTweaks={selectedTweaks}
          dryRunResults={dryRunResults}
          onClose={() => {
            setShowConfirm(false);
            setDryRunResults(null);
          }}
        />
      )}
    </div>
  );
}
