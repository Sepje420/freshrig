import { useEffect } from "react";
import { X, Sparkles } from "lucide-react";
import Markdown from "react-markdown";
import { APP_VERSION } from "../../config/app";
import { CHANGELOG } from "../../data/changelog";

interface WhatsNewModalProps {
  onClose: () => void;
}

export function WhatsNewModal({ onClose }: WhatsNewModalProps) {
  const content = CHANGELOG[APP_VERSION] ?? "";

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [onClose]);

  if (!content) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/60"
      onClick={onClose}
    >
      <div
        className="bg-bg-elevated border border-border rounded-xl shadow-elevated w-full max-w-lg mx-4 max-h-[80vh] flex flex-col animate-fade-in"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-border shrink-0">
          <div className="flex items-center gap-2">
            <Sparkles className="w-5 h-5 text-accent" />
            <h2 className="text-lg font-semibold text-text-primary">
              What's New in FreshRig v{APP_VERSION}
            </h2>
          </div>
          <button
            onClick={onClose}
            className="p-1.5 rounded-md hover:bg-bg-tertiary text-text-muted hover:text-text-primary transition-colors"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Body */}
        <div className="flex-1 overflow-y-auto px-6 py-4">
          <div className="prose prose-invert prose-sm max-w-none text-text-secondary [&_h3]:text-text-primary [&_h3]:text-base [&_h3]:font-semibold [&_h3]:mb-3 [&_strong]:text-text-primary [&_li]:my-1 [&_ul]:space-y-1">
            <Markdown>{content}</Markdown>
          </div>
        </div>

        {/* Footer */}
        <div className="px-6 py-4 border-t border-border shrink-0">
          <button
            onClick={onClose}
            className="w-full py-2 rounded-lg text-sm font-semibold bg-accent text-bg-primary hover:bg-accent-hover transition-colors"
          >
            Got it
          </button>
        </div>
      </div>
    </div>
  );
}
