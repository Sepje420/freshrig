import { useState } from "react";
import { AlertTriangle, RefreshCw, Copy, ChevronDown, ChevronUp } from "lucide-react";
import type { FallbackProps } from "react-error-boundary";
import { APP_NAME } from "../config/app";

export function ErrorFallback({ error, resetErrorBoundary }: FallbackProps) {
  const [showDetails, setShowDetails] = useState(false);
  const err = error instanceof Error ? error : new Error(String(error));

  const handleCopy = () => {
    const text = `${err.message}\n\n${err.stack ?? "No stack trace"}`;
    navigator.clipboard.writeText(text);
  };

  return (
    <div className="flex items-center justify-center min-h-[400px] p-8 animate-fade-in">
      <div className="max-w-md w-full bg-bg-card border border-border rounded-xl p-6 space-y-4 shadow-elevated">
        <div className="flex items-center gap-3">
          <div className="flex items-center justify-center w-10 h-10 rounded-lg bg-error/10">
            <AlertTriangle className="w-5 h-5 text-error" />
          </div>
          <div>
            <h2 className="text-lg font-semibold text-text-primary">Something went wrong</h2>
            <p className="text-xs text-text-muted">{APP_NAME} ran into an unexpected error</p>
          </div>
        </div>

        <p className="text-sm text-text-secondary">{err.message}</p>

        {/* Collapsible details */}
        <button
          onClick={() => setShowDetails(!showDetails)}
          className="flex items-center gap-1.5 text-xs text-text-muted hover:text-text-primary transition-colors"
        >
          {showDetails ? <ChevronUp className="w-3.5 h-3.5" /> : <ChevronDown className="w-3.5 h-3.5" />}
          {showDetails ? "Hide" : "Show"} details
        </button>

        {showDetails && (
          <pre className="text-[11px] text-text-muted bg-bg-tertiary rounded-md px-3 py-2 overflow-x-auto max-h-48 font-mono">
            {err.stack ?? "No stack trace available"}
          </pre>
        )}

        {/* Actions */}
        <div className="flex items-center gap-2 pt-2">
          <button
            onClick={resetErrorBoundary}
            className="flex items-center gap-1.5 px-4 py-2 rounded-lg text-sm font-semibold bg-accent text-bg-primary hover:bg-accent-hover transition-colors"
          >
            <RefreshCw className="w-4 h-4" />
            Try Again
          </button>
          <button
            onClick={() => window.location.reload()}
            className="px-4 py-2 rounded-lg text-sm text-text-secondary hover:text-text-primary hover:bg-bg-tertiary border border-border transition-colors"
          >
            Restart App
          </button>
          <button
            onClick={handleCopy}
            className="flex items-center gap-1.5 px-3 py-2 rounded-lg text-xs text-text-muted hover:text-text-primary hover:bg-bg-tertiary transition-colors ml-auto"
          >
            <Copy className="w-3.5 h-3.5" />
            Copy Error
          </button>
        </div>
      </div>
    </div>
  );
}
