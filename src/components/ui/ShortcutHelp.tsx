import { useEffect } from "react";
import { X, Keyboard } from "lucide-react";

interface ShortcutHelpProps {
  onClose: () => void;
}

const shortcuts = {
  Navigation: [
    { keys: "Ctrl+1", description: "Dashboard" },
    { keys: "Ctrl+2", description: "Drivers" },
    { keys: "Ctrl+3", description: "Apps" },
    { keys: "Ctrl+4", description: "Profiles" },
    { keys: "Ctrl+5", description: "Optimize Windows" },
    { keys: "Ctrl+,", description: "Settings" },
  ],
  Actions: [
    { keys: "Ctrl+K", description: "Command Palette" },
    { keys: "Ctrl+Shift+/", description: "Keyboard Shortcuts" },
  ],
};

export function ShortcutHelp({ onClose }: ShortcutHelpProps) {
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [onClose]);

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/60"
      onClick={onClose}
    >
      <div
        className="bg-bg-elevated border border-border rounded-xl shadow-elevated w-full max-w-md mx-4 animate-fade-in"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-border">
          <div className="flex items-center gap-2">
            <Keyboard className="w-5 h-5 text-accent" />
            <h2 className="text-lg font-semibold text-text-primary">Keyboard Shortcuts</h2>
          </div>
          <button
            onClick={onClose}
            className="p-1.5 rounded-md hover:bg-bg-tertiary text-text-muted hover:text-text-primary transition-colors"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Body */}
        <div className="px-6 py-4 space-y-6">
          {Object.entries(shortcuts).map(([category, items]) => (
            <div key={category}>
              <h3 className="text-xs font-semibold text-text-muted uppercase tracking-wider mb-3">
                {category}
              </h3>
              <div className="space-y-2">
                {items.map((item) => (
                  <div key={item.keys} className="flex items-center justify-between">
                    <span className="text-sm text-text-secondary">{item.description}</span>
                    <kbd className="text-xs px-2 py-1 rounded bg-bg-tertiary text-text-primary border border-border font-mono">
                      {item.keys}
                    </kbd>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>

        {/* Footer */}
        <div className="px-6 py-3 border-t border-border">
          <p className="text-xs text-text-muted text-center">Press Esc to close</p>
        </div>
      </div>
    </div>
  );
}
