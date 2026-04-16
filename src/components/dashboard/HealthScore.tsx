import type { HardwareSummary, DriverIssue } from "../../types/hardware";

interface HealthScoreProps {
  summary: HardwareSummary;
  driverIssues: DriverIssue[];
}

function calculateScore(summary: HardwareSummary, driverIssues: DriverIssue[]): number {
  let score = 100;

  // -10 per driver issue
  score -= driverIssues.length * 10;

  // -5 if any GPU driver is older than 6 months
  const sixMonthsAgo = new Date();
  sixMonthsAgo.setMonth(sixMonthsAgo.getMonth() - 6);

  for (const gpu of summary.gpus) {
    if (gpu.driverDate && gpu.driverDate !== "Unknown") {
      const driverDate = new Date(gpu.driverDate);
      if (!isNaN(driverDate.getTime()) && driverDate < sixMonthsAgo) {
        score -= 5;
        break;
      }
    }
  }

  // -5 if no GPU detected
  if (summary.gpus.length === 0) {
    score -= 5;
  }

  return Math.max(0, score);
}

function getScoreColor(score: number): string {
  if (score >= 80) return "text-success";
  if (score >= 50) return "text-warning";
  return "text-error";
}

function getStrokeColor(score: number): string {
  if (score >= 80) return "#22c55e";
  if (score >= 50) return "#f59e0b";
  return "#ef4444";
}

function getMessage(score: number): string {
  if (score >= 80) return "Your PC is ready";
  if (score >= 50) return "Some issues detected";
  return "Attention needed";
}

function getStatusLabel(score: number): string {
  if (score >= 90) return "Excellent";
  if (score >= 70) return "Good";
  return "Needs attention";
}

export function HealthScore({ summary, driverIssues }: HealthScoreProps) {
  const score = calculateScore(summary, driverIssues);
  const circumference = 2 * Math.PI * 54;
  const offset = circumference - (score / 100) * circumference;
  const strokeColor = getStrokeColor(score);
  const statusLabel = getStatusLabel(score);

  return (
    <div className="flex flex-col items-center justify-center py-4 animate-fade-in">
      <div className="relative w-32 h-32">
        <svg
          className="w-full h-full -rotate-90"
          viewBox="0 0 120 120"
          role="img"
          aria-label={`System health score: ${score} out of 100. ${statusLabel}`}
        >
          <title>System Health Score: {score}/100</title>
          {/* Background circle */}
          <circle
            cx="60"
            cy="60"
            r="54"
            fill="none"
            stroke="currentColor"
            strokeWidth="8"
            className="text-border"
          />
          {/* Progress circle */}
          <circle
            cx="60"
            cy="60"
            r="54"
            fill="none"
            stroke={strokeColor}
            strokeWidth="8"
            strokeLinecap="round"
            strokeDasharray={circumference}
            strokeDashoffset={offset}
            className="transition-all duration-1000 ease-out"
          />
        </svg>
        {/* Score text */}
        <div className="absolute inset-0 flex items-center justify-center">
          <span aria-hidden="true" className={`text-3xl font-bold ${getScoreColor(score)}`}>{score}</span>
        </div>
      </div>
      <p className={`mt-3 text-sm font-medium ${getScoreColor(score)}`}>{getMessage(score)}</p>
      <p className="text-xs text-text-muted mt-1">System Readiness</p>
    </div>
  );
}
