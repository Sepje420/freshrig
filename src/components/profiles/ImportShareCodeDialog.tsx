import { useState, useEffect } from "react";
import { X, KeyRound } from "lucide-react";
import { toast } from "sonner";
import { useProfileStore } from "../../stores/profileStore";
import type { RigProfile } from "../../types/profiles";

interface ImportShareCodeDialogProps {
  onClose: () => void;
  onImported: (profile: RigProfile) => void;
}

export function ImportShareCodeDialog({ onClose, onImported }: ImportShareCodeDialogProps) {
  const { importFromShareCode } = useProfileStore();
  const [code, setCode] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const handler = (e: KeyboardEvent) => { if (e.key === "Escape") onClose(); };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [onClose]);

  const handleImport = async () => {
    const trimmed = code.trim();
    if (!trimmed) return;

    setLoading(true);
    setError(null);

    try {
      // Strip URL prefix if pasted as link
      let shareCode = trimmed;
      const configPrefix = "config=";
      const idx = shareCode.indexOf(configPrefix);
      if (idx !== -1) {
        shareCode = shareCode.substring(idx + configPrefix.length);
      }

      const profile = await importFromShareCode(shareCode);
      toast.success(`Decoded profile: "${profile.metadata.name}"`);
      onImported(profile);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60" onClick={onClose}>
      <div
        className="bg-bg-elevated border border-border rounded-xl shadow-elevated w-full max-w-md mx-4 animate-fade-in"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-border">
          <div className="flex items-center gap-2">
            <KeyRound className="w-5 h-5 text-accent" />
            <h2 className="text-lg font-semibold text-text-primary">Import Share Code</h2>
          </div>
          <button onClick={onClose} className="p-1 rounded hover:bg-bg-tertiary text-text-muted hover:text-text-primary transition-colors">
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Body */}
        <div className="px-6 py-5 space-y-4">
          <p className="text-xs text-text-secondary">
            Paste a share code or a freshrig.app link to import a profile.
          </p>

          <textarea
            value={code}
            onChange={(e) => { setCode(e.target.value); setError(null); }}
            placeholder="Paste share code here..."
            rows={4}
            className="w-full px-3 py-2 rounded-lg bg-bg-tertiary border border-border text-sm text-text-primary font-mono placeholder:text-text-muted focus:outline-none focus:border-accent/50 transition-colors resize-none"
            autoFocus
          />

          {error && (
            <div className="px-3 py-2 rounded-md bg-error/10 border border-error/20 text-xs text-error">
              {error}
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="flex justify-end gap-2 px-6 py-4 border-t border-border">
          <button
            onClick={onClose}
            className="px-4 py-2 rounded-lg text-sm text-text-secondary hover:text-text-primary hover:bg-bg-tertiary transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleImport}
            disabled={!code.trim() || loading}
            className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all duration-200 ${
              code.trim() && !loading
                ? "bg-accent text-bg-primary hover:bg-accent-hover"
                : "bg-bg-tertiary text-text-muted cursor-not-allowed"
            }`}
          >
            {loading ? "Decoding..." : "Import"}
          </button>
        </div>
      </div>
    </div>
  );
}
