// Copyright (c) 2026 Seppe Willemsens (ZIPREX420). MIT License.
import { Crown, Clock } from "lucide-react";
import { toast } from "sonner";
import { useLicenseStore } from "../../stores/licenseStore";
import { PRO_PURCHASE_URL, PRO_PRICE_LABEL, TRIAL_DAYS } from "../../config/app";

interface ProFeatureGateProps {
  feature: string;
  children: React.ReactNode;
  mode?: "blur" | "overlay" | "badge" | "hide";
  fallback?: React.ReactNode;
}

function openPurchasePage() {
  window.open(PRO_PURCHASE_URL, "_blank", "noopener,noreferrer");
}

function UpsellCard({ feature, compact = false }: { feature: string; compact?: boolean }) {
  const canStartTrial = useLicenseStore((s) => s.canStartTrial());
  const startTrial = useLicenseStore((s) => s.startTrial);
  const isTrial = useLicenseStore((s) => s.isTrial());
  const trialDays = useLicenseStore((s) => s.trialDaysRemaining());

  const onStartTrial = () => {
    const r = startTrial();
    if (r.ok) {
      toast.success(`${TRIAL_DAYS}-day Pro trial started — enjoy!`);
    } else {
      toast.error(r.error ?? "Could not start trial");
    }
  };

  return (
    <div
      role="region"
      aria-label="Pro feature required"
      className={`flex flex-col items-center gap-2 ${compact ? "" : "max-w-xs"}`}
    >
      <Crown className={compact ? "w-6 h-6 text-amber-400" : "w-8 h-8 text-amber-400"} />
      <p className="text-text-primary font-semibold">Unlock {feature}</p>
      <button
        onClick={openPurchasePage}
        className="inline-flex items-center gap-2 bg-amber-400 hover:bg-amber-300 text-black px-4 py-2 rounded-lg font-semibold transition-colors"
      >
        Upgrade to Pro
      </button>
      <p className="text-xs text-text-secondary">{PRO_PRICE_LABEL}</p>
      {isTrial ? (
        <p className="text-xs text-amber-400 flex items-center gap-1">
          <Clock className="w-3 h-3" />
          Trial: {trialDays} day{trialDays === 1 ? "" : "s"} left
        </p>
      ) : canStartTrial ? (
        <button
          onClick={onStartTrial}
          className="text-xs text-accent hover:text-accent-hover underline underline-offset-2"
        >
          Or start a {TRIAL_DAYS}-day free trial
        </button>
      ) : (
        <p className="text-xs text-text-muted">Trial already activated</p>
      )}
    </div>
  );
}

export function ProFeatureGate({
  feature,
  children,
  mode = "overlay",
  fallback,
}: ProFeatureGateProps) {
  const isPro = useLicenseStore((s) => s.isPro());

  if (isPro) {
    return <>{children}</>;
  }

  if (mode === "hide") {
    return fallback ? <>{fallback}</> : null;
  }

  if (mode === "badge") {
    return (
      <div className="relative">
        {children}
        <div className="absolute top-2 right-2 flex items-center gap-1 bg-amber-500/90 text-black text-xs font-semibold px-2 py-0.5 rounded-full">
          <Crown className="w-3 h-3" />
          PRO
        </div>
      </div>
    );
  }

  if (mode === "blur") {
    return (
      <div className="relative">
        <div aria-hidden="true" className="blur-sm pointer-events-none select-none">{children}</div>
        <div className="absolute inset-0 flex items-center justify-center bg-bg-primary/70 rounded-lg">
          <UpsellCard feature={feature} />
        </div>
      </div>
    );
  }

  // overlay mode (default)
  return (
    <div className="relative group">
      <div aria-hidden="true" className="opacity-40 pointer-events-none select-none">{children}</div>
      <div className="absolute inset-0 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity bg-bg-primary/60 rounded-lg">
        <UpsellCard feature={feature} />
      </div>
    </div>
  );
}
