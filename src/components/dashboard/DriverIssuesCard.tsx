import { AlertTriangle } from "lucide-react";
import type { DriverIssue } from "../../types/hardware";

interface DriverIssuesCardProps {
  issues: DriverIssue[];
}

export function DriverIssuesCard({ issues }: DriverIssuesCardProps) {
  if (issues.length === 0) return null;

  return (
    <div className="relative bg-bg-card rounded-lg border border-error/30 shadow-card animate-fade-in">
      {/* Header */}
      <div className="flex items-center gap-2.5 px-5 pt-5 pb-3">
        <AlertTriangle className="w-4.5 h-4.5 text-error" />
        <h3 className="text-sm font-semibold text-error">
          Driver Issues ({issues.length})
        </h3>
      </div>

      {/* Issues list */}
      <div className="px-5 pb-5 space-y-3">
        {issues.map((issue, i) => (
          <div
            key={i}
            className="px-3 py-2.5 rounded-md bg-error/5 border border-error/10 space-y-1"
          >
            <div className="flex items-start justify-between gap-2">
              <p className="text-sm text-text-primary font-medium">{issue.deviceName}</p>
              <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-error/20 text-error shrink-0">
                Code {issue.errorCode}
              </span>
            </div>
            <p className="text-xs text-warning">{issue.errorDescription}</p>
            {issue.hardwareId.length > 0 && (
              <p className="text-[11px] text-text-muted font-mono truncate" title={issue.hardwareId[0]}>
                {issue.hardwareId[0]}
              </p>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
