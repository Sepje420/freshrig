import { useEffect, useRef } from "react";
import { Copy, FileDown, Link, MessageSquare } from "lucide-react";
import { toast } from "sonner";
import { useProfileStore } from "../../stores/profileStore";
import { useAppStore } from "../../stores/appStore";
import type { RigProfile } from "../../types/profiles";

interface ShareMenuProps {
  filePath: string;
  onAction: () => void;
  onClose: () => void;
}

export function ShareMenu({ filePath, onAction, onClose }: ShareMenuProps) {
  const { loadProfile, exportToFile, exportAsText, generateShareCode } = useProfileStore();
  const { catalog } = useAppStore();
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handler = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) onClose();
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, [onClose]);

  const getProfile = async (): Promise<RigProfile> => {
    return await loadProfile(filePath);
  };

  const handleCopyShareCode = async () => {
    try {
      const profile = await getProfile();
      const code = await generateShareCode(profile);
      await navigator.clipboard.writeText(code);
      toast.success("Share code copied to clipboard");
      onAction();
    } catch (err) {
      toast.error(`Failed: ${err}`);
    }
  };

  const handleCopyMarkdown = async () => {
    try {
      const profile = await getProfile();
      const text = await exportAsText(profile, catalog);
      await navigator.clipboard.writeText(text);
      toast.success("Copied for Reddit/Discord");
      onAction();
    } catch (err) {
      toast.error(`Failed: ${err}`);
    }
  };

  const handleExportFile = async () => {
    try {
      const profile = await getProfile();
      await exportToFile(profile);
      toast.success("Profile exported");
      onAction();
    } catch (err) {
      if (!String(err).includes("cancelled")) toast.error(`Failed: ${err}`);
      onClose();
    }
  };

  const handleCopyLink = async () => {
    try {
      const profile = await getProfile();
      const code = await generateShareCode(profile);
      const link = `https://freshrig.app/import#config=${code}`;
      await navigator.clipboard.writeText(link);
      toast.success("Share link copied to clipboard");
      onAction();
    } catch (err) {
      toast.error(`Failed: ${err}`);
    }
  };

  const items = [
    { icon: Copy, label: "Copy Share Code", action: handleCopyShareCode },
    { icon: MessageSquare, label: "Copy for Reddit/Discord", action: handleCopyMarkdown },
    { icon: FileDown, label: "Export as File", action: handleExportFile },
    { icon: Link, label: "Copy Share Link", action: handleCopyLink },
  ];

  return (
    <div
      ref={ref}
      className="absolute top-full left-0 mt-1 w-52 bg-bg-elevated border border-border rounded-lg shadow-elevated z-50 py-1 animate-fade-in"
    >
      {items.map((item) => (
        <button
          key={item.label}
          onClick={item.action}
          className="flex items-center gap-2.5 w-full px-3 py-2 text-xs text-text-secondary hover:text-text-primary hover:bg-bg-tertiary transition-colors"
        >
          <item.icon className="w-3.5 h-3.5" />
          {item.label}
        </button>
      ))}
    </div>
  );
}
