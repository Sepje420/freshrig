import { Check, AlertTriangle } from "lucide-react";
import type { DebloatTweak, TweakTier, TweakCategory } from "../../types/debloat";

interface TweakCardProps {
  tweak: DebloatTweak;
  selected: boolean;
  onToggle: () => void;
}

const tierColors: Record<TweakTier, string> = {
  safe: "bg-success/20 text-success",
  moderate: "bg-warning/20 text-warning",
  risky: "bg-error/20 text-error",
};

const tierDotColors: Record<TweakTier, string> = {
  safe: "bg-success",
  moderate: "bg-warning",
  risky: "bg-error",
};

const categoryColors: Record<TweakCategory, string> = {
  privacy: "bg-blue-500/20 text-blue-400",
  bloatware: "bg-purple-500/20 text-purple-400",
  performance: "bg-orange-500/20 text-orange-400",
  appearance: "bg-cyan-500/20 text-cyan-400",
};

const categoryLabels: Record<TweakCategory, string> = {
  privacy: "Privacy",
  bloatware: "Bloatware",
  performance: "Performance",
  appearance: "Appearance",
};

const tierLabels: Record<TweakTier, string> = {
  safe: "Safe",
  moderate: "Moderate",
  risky: "Risky",
};

export function TweakCard({ tweak, selected, onToggle }: TweakCardProps) {
  const disabled = tweak.isApplied;

  return (
    <button
      onClick={disabled ? undefined : onToggle}
      disabled={disabled}
      className={`relative w-full text-left rounded-lg border transition-all duration-200 ${
        selected && !disabled
          ? "bg-accent-muted border-accent/50 shadow-elevated"
          : "bg-bg-card border-border hover:bg-bg-card-hover hover:border-border-hover shadow-card"
      } ${disabled ? "opacity-60 cursor-default" : "cursor-pointer"}`}
    >
      <div className="p-4 flex items-start gap-3">
        {/* Checkbox */}
        <div className="shrink-0 mt-0.5">
          {disabled ? (
            <div className="w-5 h-5 rounded bg-success/20 flex items-center justify-center">
              <Check className="w-3.5 h-3.5 text-success" />
            </div>
          ) : (
            <div
              className={`w-5 h-5 rounded border-2 transition-colors ${
                selected ? "bg-accent border-accent" : "border-border-hover bg-transparent"
              }`}
            >
              {selected && <Check className="w-3.5 h-3.5 text-bg-primary" />}
            </div>
          )}
        </div>

        {/* Tier dot */}
        <div className="shrink-0 mt-1.5">
          <div className={`w-2.5 h-2.5 rounded-full ${tierDotColors[tweak.tier]}`} />
        </div>

        {/* Info */}
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 flex-wrap">
            <h3 className="text-sm font-semibold text-text-primary">{tweak.name}</h3>
            <span
              className={`text-[9px] font-semibold px-1.5 py-0.5 rounded shrink-0 ${tierColors[tweak.tier]}`}
            >
              {tierLabels[tweak.tier]}
            </span>
            <span
              className={`text-[9px] font-semibold px-1.5 py-0.5 rounded shrink-0 ${categoryColors[tweak.category]}`}
            >
              {categoryLabels[tweak.category]}
            </span>
            {tweak.isApplied && (
              <span className="text-[9px] font-semibold px-1.5 py-0.5 rounded bg-success/15 text-success shrink-0">
                Applied
              </span>
            )}
          </div>
          <p className="text-xs text-text-muted mt-0.5">{tweak.description}</p>
          {tweak.warning && (
            <div className="flex items-center gap-1.5 mt-1.5">
              <AlertTriangle className="w-3 h-3 text-warning shrink-0" />
              <p className="text-[11px] text-warning">{tweak.warning}</p>
            </div>
          )}
          <p className="text-[10px] text-text-muted mt-1">
            {tweak.isReversible ? "Reversible" : "⚠ Not easily reversible"}
          </p>
        </div>
      </div>
    </button>
  );
}
