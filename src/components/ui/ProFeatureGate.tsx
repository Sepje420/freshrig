import { Crown } from "lucide-react";
import { useLicenseStore } from "../../stores/licenseStore";

interface ProFeatureGateProps {
  feature: string;
  children: React.ReactNode;
  mode?: "blur" | "overlay" | "badge" | "hide";
  fallback?: React.ReactNode;
}

export function ProFeatureGate({
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
        <div className="blur-sm pointer-events-none select-none">{children}</div>
        <div className="absolute inset-0 flex items-center justify-center bg-bg-primary/60 rounded-lg">
          <div className="text-center space-y-2">
            <Crown className="w-8 h-8 text-amber-500 mx-auto" />
            <p className="text-text-primary font-medium">Pro Feature</p>
            <p className="text-text-secondary text-sm">Upgrade to unlock</p>
          </div>
        </div>
      </div>
    );
  }

  // overlay mode (default)
  return (
    <div className="relative group cursor-pointer">
      <div className="opacity-50 pointer-events-none">{children}</div>
      <div className="absolute inset-0 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity bg-bg-primary/40 rounded-lg">
        <div className="flex items-center gap-2 bg-amber-500 text-black px-4 py-2 rounded-lg font-medium">
          <Crown className="w-4 h-4" />
          Upgrade to Pro
        </div>
      </div>
    </div>
  );
}
