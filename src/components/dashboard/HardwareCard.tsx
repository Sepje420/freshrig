import type { LucideIcon } from "lucide-react";
import { Card } from "../ui/Card";

interface HardwareCardProps {
  title: string;
  icon: LucideIcon;
  status: "good" | "warning" | "error";
  children: React.ReactNode;
}

const statusColors = {
  good: "bg-success",
  warning: "bg-warning",
  error: "bg-error",
};

export function HardwareCard({ title, icon: Icon, status, children }: HardwareCardProps) {
  return (
    <Card interactive className="relative animate-fade-in">
      {/* Status dot */}
      <div className={`absolute top-4 right-4 w-2.5 h-2.5 rounded-full ${statusColors[status]}`} />

      {/* Header */}
      <div className="flex items-center gap-2.5 px-5 pt-5 pb-3">
        <Icon className="w-4.5 h-4.5 text-[var(--accent)]" />
        <h3 className="text-sm font-semibold text-[var(--text-primary)]">{title}</h3>
      </div>

      {/* Content */}
      <div className="px-5 pb-5">{children}</div>
    </Card>
  );
}
