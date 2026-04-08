import { useState, useEffect } from "react";
import { X } from "lucide-react";
import { toast } from "sonner";
import { useProfileStore } from "../../stores/profileStore";
import { useAppStore } from "../../stores/appStore";
import { APP_VERSION } from "../../config/app";
import type { RigProfile } from "../../types/profiles";
import type { AppCategory } from "../../types/apps";

interface SaveProfileDialogProps {
  onClose: () => void;
  onSaved: () => void;
}

export function SaveProfileDialog({ onClose, onSaved }: SaveProfileDialogProps) {
  const { saveProfile, getHardwareSnapshot } = useProfileStore();
  const { selectedIds, catalog } = useAppStore();

  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [author, setAuthor] = useState("");
  const [includeHardware, setIncludeHardware] = useState(true);
  const [saving, setSaving] = useState(false);

  // Close on Escape
  useEffect(() => {
    const handler = (e: KeyboardEvent) => { if (e.key === "Escape") onClose(); };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [onClose]);

  const handleSave = async () => {
    if (!name.trim()) return;
    setSaving(true);

    try {
      const now = new Date().toISOString();

      // Deduce categories from selected apps
      const categories = [
        ...new Set(
          catalog
            .filter((a) => selectedIds.has(a.id))
            .map((a) => a.category)
        ),
      ] as AppCategory[];

      const profile: RigProfile = {
        configVersion: 1,
        metadata: {
          name: name.trim(),
          description: description.trim() || undefined,
          author: author.trim() || undefined,
          createdAt: now,
          updatedAt: now,
          appVersion: APP_VERSION,
          sourceHardware: includeHardware ? await getHardwareSnapshot() : undefined,
        },
        apps: [...selectedIds],
        categories,
      };

      await saveProfile(profile);
      toast.success("Profile saved successfully");
      onSaved();
      onClose();
    } catch (err) {
      toast.error(`Failed to save profile: ${err}`);
    } finally {
      setSaving(false);
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
          <h2 className="text-lg font-semibold text-text-primary">Save Profile</h2>
          <button onClick={onClose} className="p-1 rounded hover:bg-bg-tertiary text-text-muted hover:text-text-primary transition-colors">
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Body */}
        <div className="px-6 py-5 space-y-4">
          {selectedIds.size === 0 && (
            <div className="px-3 py-2 rounded-md bg-warning/10 border border-warning/20 text-xs text-warning">
              No apps selected. Go to the App Catalog and select some apps first.
            </div>
          )}

          <div className="space-y-1.5">
            <label className="text-xs font-medium text-text-secondary">Name *</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value.slice(0, 50))}
              placeholder="My Gaming Setup"
              className="w-full px-3 py-2 rounded-lg bg-bg-tertiary border border-border text-sm text-text-primary placeholder:text-text-muted focus:outline-none focus:border-accent/50 transition-colors"
              autoFocus
            />
            <p className="text-[11px] text-text-muted">{name.length}/50</p>
          </div>

          <div className="space-y-1.5">
            <label className="text-xs font-medium text-text-secondary">Description</label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value.slice(0, 200))}
              placeholder="Essential apps for my gaming PC..."
              rows={2}
              className="w-full px-3 py-2 rounded-lg bg-bg-tertiary border border-border text-sm text-text-primary placeholder:text-text-muted focus:outline-none focus:border-accent/50 transition-colors resize-none"
            />
            <p className="text-[11px] text-text-muted">{description.length}/200</p>
          </div>

          <div className="space-y-1.5">
            <label className="text-xs font-medium text-text-secondary">Author</label>
            <input
              type="text"
              value={author}
              onChange={(e) => setAuthor(e.target.value)}
              placeholder="Your name (optional)"
              className="w-full px-3 py-2 rounded-lg bg-bg-tertiary border border-border text-sm text-text-primary placeholder:text-text-muted focus:outline-none focus:border-accent/50 transition-colors"
            />
          </div>

          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={includeHardware}
              onChange={(e) => setIncludeHardware(e.target.checked)}
              className="accent-accent"
            />
            <span className="text-xs text-text-secondary">Include my hardware info</span>
          </label>

          <div className="text-xs text-text-muted">
            {selectedIds.size} app{selectedIds.size !== 1 ? "s" : ""} will be saved in this profile.
          </div>
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
            onClick={handleSave}
            disabled={!name.trim() || selectedIds.size === 0 || saving}
            className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all duration-200 ${
              name.trim() && selectedIds.size > 0 && !saving
                ? "bg-accent text-bg-primary hover:bg-accent-hover"
                : "bg-bg-tertiary text-text-muted cursor-not-allowed"
            }`}
          >
            {saving ? "Saving..." : "Save Profile"}
          </button>
        </div>
      </div>
    </div>
  );
}
