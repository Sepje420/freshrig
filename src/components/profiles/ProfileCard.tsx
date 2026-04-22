import { useState } from "react";
import { Package, Trash2, Share2, Download } from "lucide-react";
import { ShareMenu } from "./ShareMenu";
import { Card } from "../ui/Card";
import type { ProfileSummary } from "../../types/profiles";

interface ProfileCardProps {
  profile: ProfileSummary;
  index: number;
  onLoad: () => void;
  onDelete: () => void;
  onShareProfile: (filePath: string) => void;
}

function relativeDate(iso: string): string {
  const now = Date.now();
  const then = new Date(iso).getTime();
  if (isNaN(then)) return iso;
  const diff = now - then;
  const secs = Math.floor(diff / 1000);
  if (secs < 60) return "just now";
  const mins = Math.floor(secs / 60);
  if (mins < 60) return `${mins} min ago`;
  const hours = Math.floor(mins / 60);
  if (hours < 24) return `${hours} hour${hours !== 1 ? "s" : ""} ago`;
  const days = Math.floor(hours / 24);
  if (days === 1) return "Yesterday";
  if (days < 30) return `${days} days ago`;
  return new Date(iso).toLocaleDateString();
}

export function ProfileCard({ profile, index, onLoad, onDelete, onShareProfile }: ProfileCardProps) {
  const [confirmDelete, setConfirmDelete] = useState(false);
  const [showShare, setShowShare] = useState(false);

  return (
    <Card
      className="group relative animate-fade-in hover:border-[var(--accent-ring)]"
      style={{ animationDelay: `${index * 50}ms` }}
    >
      <div className="p-5 space-y-3">
        {/* Header */}
        <div className="flex items-start justify-between">
          <div className="flex-1 min-w-0">
            <h3 className="text-sm font-semibold text-text-primary truncate">{profile.name}</h3>
            {profile.description && (
              <p className="text-xs text-text-muted mt-1 line-clamp-2">{profile.description}</p>
            )}
          </div>
          <div className="flex items-center gap-1 shrink-0 ml-3">
            <Package className="w-3.5 h-3.5 text-accent" />
            <span className="text-xs font-semibold text-accent">{profile.appCount}</span>
          </div>
        </div>

        {/* Meta */}
        <div className="flex items-center gap-3 text-[11px] text-text-muted">
          <span>{relativeDate(profile.updatedAt)}</span>
        </div>

        {/* Actions */}
        <div className="flex items-center gap-2 pt-1">
          <button
            onClick={onLoad}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-accent text-bg-primary text-xs font-medium hover:bg-accent-hover transition-colors"
          >
            <Download className="w-3.5 h-3.5" />
            Load
          </button>

          <div className="relative">
            <button
              onClick={() => setShowShare(!showShare)}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-md border border-border text-text-secondary text-xs font-medium hover:bg-bg-tertiary hover:text-text-primary transition-colors"
            >
              <Share2 className="w-3.5 h-3.5" />
              Share
            </button>
            {showShare && (
              <ShareMenu
                filePath={profile.filePath}
                onAction={() => {
                  setShowShare(false);
                  onShareProfile(profile.filePath);
                }}
                onClose={() => setShowShare(false)}
              />
            )}
          </div>

          {!confirmDelete ? (
            <button
              onClick={() => setConfirmDelete(true)}
              className="ml-auto p-1.5 rounded-md text-text-muted hover:text-error hover:bg-error/10 transition-colors opacity-0 group-hover:opacity-100"
            >
              <Trash2 className="w-3.5 h-3.5" />
            </button>
          ) : (
            <div className="ml-auto flex items-center gap-1">
              <button
                onClick={() => { onDelete(); setConfirmDelete(false); }}
                className="px-2 py-1 rounded text-[11px] font-medium bg-error/20 text-error hover:bg-error/30 transition-colors"
              >
                Delete
              </button>
              <button
                onClick={() => setConfirmDelete(false)}
                className="px-2 py-1 rounded text-[11px] text-text-muted hover:text-text-primary transition-colors"
              >
                Cancel
              </button>
            </div>
          )}
        </div>
      </div>
    </Card>
  );
}
