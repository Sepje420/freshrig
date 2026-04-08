import { useState, useEffect, useRef } from "react";
import { Check, X, Loader2, Clock, SkipForward, BookMarked, List, PartyPopper } from "lucide-react";
import confetti from "canvas-confetti";
import type { InstallProgress } from "../../types/apps";
import { SaveProfileDialog } from "../profiles/SaveProfileDialog";

interface InstallProgressPanelProps {
  progress: Map<string, InstallProgress>;
  onDone?: () => void;
}

const statusIcons: Record<string, React.ElementType> = {
  Pending: Clock,
  Installing: Loader2,
  Completed: Check,
  Failed: X,
  Skipped: SkipForward,
};

const statusStyles: Record<string, string> = {
  Pending: "text-text-muted",
  Installing: "text-accent animate-spin",
  Completed: "text-success",
  Failed: "text-error",
  Skipped: "text-warning",
};

export function InstallProgressPanel({ progress, onDone }: InstallProgressPanelProps) {
  const entries = [...progress.values()];
  const completed = entries.filter((p) => p.status === "Completed").length;
  const failed = entries.filter((p) => p.status === "Failed").length;
  const total = entries.length;
  const done = entries.filter(
    (p) => p.status === "Completed" || p.status === "Failed" || p.status === "Skipped"
  ).length;
  const allDone = done === total;
  const progressPct = total > 0 ? (done / total) * 100 : 0;

  const [startTime] = useState(() => Date.now());
  const [elapsed, setElapsed] = useState(0);
  const [showSummary, setShowSummary] = useState(false);
  const [showSaveProfile, setShowSaveProfile] = useState(false);
  const [animatedSaved, setAnimatedSaved] = useState(0);
  const confettiFired = useRef(false);

  // Track elapsed time
  useEffect(() => {
    if (allDone) return;
    const interval = setInterval(() => setElapsed(Date.now() - startTime), 1000);
    return () => clearInterval(interval);
  }, [allDone, startTime]);

  // On completion: show summary + confetti
  useEffect(() => {
    if (allDone && !showSummary && total > 0) {
      setElapsed(Date.now() - startTime);
      setShowSummary(true);

      if (completed > 0 && failed === 0 && !confettiFired.current) {
        confettiFired.current = true;
        confetti({
          particleCount: 100,
          spread: 70,
          origin: { y: 0.6 },
          colors: ["#00d4aa", "#22c55e", "#3b82f6"],
          disableForReducedMotion: true,
        });
      }
    }
  }, [allDone, total, completed, failed, showSummary, startTime]);

  // Animate "time saved" counter
  useEffect(() => {
    if (!showSummary) return;
    const timeSavedMin = Math.max(0, completed * 3 - Math.floor(elapsed / 60000));
    const duration = 1500;
    const startTs = Date.now();
    const animate = () => {
      const progress = Math.min(1, (Date.now() - startTs) / duration);
      const eased = 1 - Math.pow(1 - progress, 3);
      setAnimatedSaved(Math.round(eased * timeSavedMin));
      if (progress < 1) requestAnimationFrame(animate);
    };
    requestAnimationFrame(animate);
  }, [showSummary, completed, elapsed]);

  const formatTime = (ms: number) => {
    const secs = Math.floor(ms / 1000);
    const m = Math.floor(secs / 60);
    const s = secs % 60;
    return `${m}:${s.toString().padStart(2, "0")}`;
  };

  // Summary view
  if (showSummary) {
    return (
      <div className="fixed bottom-0 left-[280px] right-0 bg-bg-elevated border-t border-border shadow-elevated z-40 animate-fade-in">
        <div className="px-6 py-6 space-y-4">
          <div className="flex items-center gap-4">
            <div className="flex items-center justify-center w-12 h-12 rounded-full bg-success/20 animate-check-pop">
              <Check className="w-6 h-6 text-success" />
            </div>
            <div>
              <h3 className="text-lg font-bold text-text-primary">Setup Complete!</h3>
              <p className="text-xs text-text-muted">Your rig is ready.</p>
            </div>
          </div>

          {/* Stats */}
          <div className="flex items-center gap-6 text-sm">
            <span className="text-text-secondary">
              <span className="font-semibold text-success">{completed}</span> apps installed
            </span>
            {failed > 0 && (
              <span className="text-text-secondary">
                <span className="font-semibold text-error">{failed}</span> failed
              </span>
            )}
            <span className="text-text-muted">Time: {formatTime(elapsed)}</span>
            <span className="text-accent font-semibold">
              ~{animatedSaved} min saved vs manual
            </span>
          </div>

          {/* Buttons */}
          <div className="flex items-center gap-2">
            <button
              onClick={() => setShowSummary(false)}
              className="flex items-center gap-1.5 px-3 py-2 rounded-lg text-xs text-text-secondary hover:text-text-primary hover:bg-bg-tertiary border border-border transition-colors"
            >
              <List className="w-3.5 h-3.5" />
              View Details
            </button>
            <button
              onClick={() => setShowSaveProfile(true)}
              className="flex items-center gap-1.5 px-3 py-2 rounded-lg text-xs text-text-secondary hover:text-text-primary hover:bg-bg-tertiary border border-border transition-colors"
            >
              <BookMarked className="w-3.5 h-3.5" />
              Save as Profile
            </button>
            <button
              onClick={onDone}
              className="flex items-center gap-1.5 px-4 py-2 rounded-lg text-xs font-semibold bg-accent text-bg-primary hover:bg-accent-hover transition-colors ml-auto"
            >
              <PartyPopper className="w-3.5 h-3.5" />
              Done
            </button>
          </div>
        </div>

        {showSaveProfile && (
          <SaveProfileDialog
            onClose={() => setShowSaveProfile(false)}
            onSaved={() => setShowSaveProfile(false)}
          />
        )}
      </div>
    );
  }

  // Progress view
  return (
    <div className="fixed bottom-0 left-[280px] right-0 bg-bg-elevated border-t border-border shadow-elevated z-40 animate-fade-in">
      <div className="px-6 py-4 space-y-3 max-h-[320px] overflow-y-auto">
        <div className="flex items-center justify-between">
          <h3 className="text-sm font-semibold text-text-primary">
            {allDone
              ? `Installation Complete — ${completed} installed, ${failed} failed`
              : `Installing... (${done}/${total})`}
          </h3>
          <span className="text-xs text-text-muted font-mono">{formatTime(elapsed)}</span>
        </div>

        <div className="w-full h-1.5 bg-bg-tertiary rounded-full overflow-hidden">
          <div
            className="h-full bg-accent rounded-full transition-all duration-500 ease-out"
            style={{ width: `${progressPct}%` }}
          />
        </div>

        <div className="space-y-1.5">
          {entries.map((p) => {
            const Icon = statusIcons[p.status] ?? Clock;
            return (
              <div
                key={p.appId}
                className="flex items-center gap-3 px-3 py-1.5 rounded-md bg-bg-card/50"
              >
                <Icon className={`w-4 h-4 shrink-0 ${statusStyles[p.status]}`} />
                <span className="text-sm text-text-primary flex-1 truncate">{p.appName}</span>
                <span className="text-xs text-text-muted truncate max-w-[300px]">{p.message}</span>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}
