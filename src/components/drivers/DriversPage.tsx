import { useEffect } from "react";
import { Cpu, Info } from "lucide-react";
import { useDriverStore } from "../../stores/driverStore";
import { DriverCard } from "./DriverCard";

export function DriversPage() {
  const { recommendations, loading, error, fetchRecommendations } = useDriverStore();

  useEffect(() => {
    fetchRecommendations();
  }, [fetchRecommendations]);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <div className="flex items-center gap-3">
          <div className="flex items-center justify-center w-10 h-10 rounded-lg bg-accent-muted">
            <Cpu className="w-5 h-5 text-accent" />
          </div>
          <div>
            <h1 className="text-2xl font-bold text-text-primary">Driver Recommendations</h1>
            <p className="text-sm text-text-secondary mt-0.5">Based on your detected hardware</p>
          </div>
        </div>
      </div>

      {/* Info banner */}
      {!loading && !error && recommendations.length > 0 && (
        <div className="flex items-start gap-3 px-4 py-3 rounded-lg bg-info/10 border border-info/20">
          <Info className="w-5 h-5 text-info shrink-0 mt-0.5" />
          <p className="text-xs text-text-secondary leading-relaxed">
            Drivers marked <span className="font-semibold text-text-primary">Install</span> use winget to install the
            vendor's driver management tool (e.g., GeForce Experience, Intel DSA), which then handles driver
            downloads and updates automatically. Other drivers open the vendor's support page in your browser.
          </p>
        </div>
      )}

      {/* Loading */}
      {loading && (
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
          {Array.from({ length: 6 }).map((_, i) => (
            <div key={i} className="h-52 rounded-lg bg-bg-card border border-border animate-pulse" />
          ))}
        </div>
      )}

      {/* Error */}
      {error && (
        <div className="flex flex-col items-center justify-center py-16 animate-fade-in">
          <div className="w-14 h-14 rounded-full bg-error/10 flex items-center justify-center mb-4">
            <span className="text-error text-xl">!</span>
          </div>
          <h3 className="text-lg font-semibold text-text-primary mb-2">Failed to detect drivers</h3>
          <p className="text-sm text-text-secondary max-w-md text-center mb-4">{error}</p>
          <button
            onClick={fetchRecommendations}
            className="px-4 py-2 rounded-md bg-accent text-bg-primary text-sm font-medium hover:bg-accent-hover transition-colors"
          >
            Retry
          </button>
        </div>
      )}

      {/* Recommendations grid */}
      {!loading && !error && recommendations.length > 0 && (
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
          {recommendations.map((rec, i) => (
            <DriverCard key={`${rec.category}-${i}`} recommendation={rec} />
          ))}
        </div>
      )}

      {/* Empty state */}
      {!loading && !error && recommendations.length === 0 && (
        <div className="flex flex-col items-center justify-center py-16 animate-fade-in">
          <Cpu className="w-12 h-12 text-text-muted mb-4" />
          <h3 className="text-lg font-semibold text-text-primary mb-1">No recommendations</h3>
          <p className="text-sm text-text-secondary">Could not detect hardware that needs driver updates.</p>
        </div>
      )}
    </div>
  );
}
