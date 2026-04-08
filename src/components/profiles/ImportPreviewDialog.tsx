import { useState } from "react";
import { X, AlertTriangle, Check } from "lucide-react";
import { toast } from "sonner";
import { useProfileStore } from "../../stores/profileStore";
import { useAppStore } from "../../stores/appStore";
import type { RigProfile } from "../../types/profiles";

interface ImportPreviewDialogProps {
  profile: RigProfile;
  onClose: () => void;
}

export function ImportPreviewDialog({ profile, onClose }: ImportPreviewDialogProps) {
  const { saveProfile, fetchProfiles, setActiveProfile } = useProfileStore();
  const { catalog } = useAppStore();
  const setSelectedIds = useAppStore((s) => s.clearSelection);
  const toggleApp = useAppStore((s) => s.toggleApp);

  const [checkedApps, setCheckedApps] = useState<Set<string>>(new Set(profile.apps));
  const [importing, setImporting] = useState(false);

  const catalogIds = new Set(catalog.map((a) => a.id));
  const appName = (id: string) => catalog.find((a) => a.id === id)?.name ?? id;

  const toggleCheck = (id: string) => {
    setCheckedApps((prev) => {
      const n = new Set(prev);
      if (n.has(id)) n.delete(id); else n.add(id);
      return n;
    });
  };

  const handleApply = async () => {
    setImporting(true);
    try {
      // Save profile locally
      await saveProfile(profile);
      await fetchProfiles();

      // Apply selections to app store
      setSelectedIds();
      for (const id of checkedApps) {
        toggleApp(id);
      }

      setActiveProfile(profile);
      toast.success(`Profile "${profile.metadata.name}" imported with ${checkedApps.size} apps`);
      onClose();
    } catch (err) {
      toast.error(`Import failed: ${err}`);
    } finally {
      setImporting(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60" onClick={onClose}>
      <div
        className="bg-bg-elevated border border-border rounded-xl shadow-elevated w-full max-w-lg mx-4 animate-fade-in max-h-[80vh] flex flex-col"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-border shrink-0">
          <h2 className="text-lg font-semibold text-text-primary">Import Profile</h2>
          <button onClick={onClose} className="p-1 rounded hover:bg-bg-tertiary text-text-muted hover:text-text-primary transition-colors">
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Body */}
        <div className="px-6 py-5 space-y-4 overflow-y-auto flex-1">
          {/* Profile info */}
          <div className="space-y-2">
            <h3 className="text-sm font-semibold text-text-primary">{profile.metadata.name}</h3>
            {profile.metadata.description && (
              <p className="text-xs text-text-muted">{profile.metadata.description}</p>
            )}
            <div className="flex flex-wrap gap-3 text-[11px] text-text-muted">
              {profile.metadata.author && <span>By {profile.metadata.author}</span>}
              <span>{new Date(profile.metadata.createdAt).toLocaleDateString()}</span>
              <span>v{profile.metadata.appVersion}</span>
            </div>
          </div>

          {/* Source hardware */}
          {profile.metadata.sourceHardware && (
            <div className="px-3 py-2 rounded-md bg-bg-tertiary text-xs text-text-secondary space-y-1">
              <p className="font-medium text-text-primary text-[11px] uppercase tracking-wider">Source Hardware</p>
              {profile.metadata.sourceHardware.cpu && <p>CPU: {profile.metadata.sourceHardware.cpu}</p>}
              {profile.metadata.sourceHardware.gpu && <p>GPU: {profile.metadata.sourceHardware.gpu}</p>}
              {profile.metadata.sourceHardware.ramGb && <p>RAM: {profile.metadata.sourceHardware.ramGb.toFixed(0)} GB</p>}
              {profile.metadata.sourceHardware.os && <p>OS: {profile.metadata.sourceHardware.os}</p>}
            </div>
          )}

          {/* App list */}
          <div className="space-y-1">
            <p className="text-xs font-medium text-text-secondary mb-2">
              Apps ({checkedApps.size}/{profile.apps.length} selected)
            </p>
            {profile.apps.map((id) => {
              const inCatalog = catalogIds.has(id);
              return (
                <label
                  key={id}
                  className={`flex items-center gap-2.5 px-3 py-2 rounded-md cursor-pointer transition-colors ${
                    checkedApps.has(id) ? "bg-accent-muted/30" : "hover:bg-bg-tertiary"
                  }`}
                >
                  <input
                    type="checkbox"
                    checked={checkedApps.has(id)}
                    onChange={() => toggleCheck(id)}
                    className="accent-accent"
                  />
                  <span className="text-sm text-text-primary flex-1 truncate">{appName(id)}</span>
                  {!inCatalog && (
                    <span className="flex items-center gap-1 text-[10px] text-warning shrink-0">
                      <AlertTriangle className="w-3 h-3" />
                      Not in catalog
                    </span>
                  )}
                </label>
              );
            })}
          </div>
        </div>

        {/* Footer */}
        <div className="flex justify-end gap-2 px-6 py-4 border-t border-border shrink-0">
          <button
            onClick={onClose}
            className="px-4 py-2 rounded-lg text-sm text-text-secondary hover:text-text-primary hover:bg-bg-tertiary transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleApply}
            disabled={importing}
            className="flex items-center gap-1.5 px-4 py-2 rounded-lg text-sm font-semibold bg-accent text-bg-primary hover:bg-accent-hover transition-colors"
          >
            <Check className="w-4 h-4" />
            {importing ? "Importing..." : "Import & Apply"}
          </button>
        </div>
      </div>
    </div>
  );
}
