import {
  LayoutDashboard,
  Cpu,
  Package,
  BookMarked,
  Settings,
  Monitor,
} from "lucide-react";
import { APP_NAME, APP_VERSION } from "../../config/app";

interface SidebarProps {
  currentView: string;
  onNavigate: (view: string) => void;
}

const navItems = [
  { id: "dashboard", label: "Dashboard", icon: LayoutDashboard, enabled: true },
  { id: "drivers", label: "Drivers", icon: Cpu, enabled: true },
  { id: "apps", label: "Apps", icon: Package, enabled: true },
  { id: "profiles", label: "Profiles", icon: BookMarked, enabled: true },
  { id: "settings", label: "Settings", icon: Settings, enabled: true },
];

export function Sidebar({ currentView, onNavigate }: SidebarProps) {
  return (
    <aside className="flex flex-col w-[280px] shrink-0 h-full bg-bg-secondary border-r border-border overflow-y-auto">
      {/* Logo / App Name */}
      <div className="flex items-center gap-3 px-6 py-6">
        <div className="flex items-center justify-center w-10 h-10 rounded-lg bg-accent-muted">
          <Monitor className="w-5 h-5 text-accent" />
        </div>
        <div>
          <h1 className="text-lg font-semibold text-text-primary">{APP_NAME}</h1>
          <p className="text-xs text-text-muted">System Setup Tool</p>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 px-3 mt-2">
        <ul className="space-y-1">
          {navItems.map((item) => {
            const isActive = currentView === item.id;
            const Icon = item.icon;

            return (
              <li key={item.id} className="relative group">
                <button
                  onClick={() => item.enabled && onNavigate(item.id)}
                  className={`flex items-center gap-3 w-full px-4 py-2.5 rounded-md text-sm font-medium transition-all duration-200 ${
                    isActive
                      ? "bg-accent-muted text-accent border-l-2 border-accent"
                      : item.enabled
                        ? "text-text-secondary hover:text-text-primary hover:bg-bg-tertiary"
                        : "text-text-muted cursor-not-allowed"
                  }`}
                >
                  <Icon className="w-4.5 h-4.5" />
                  <span>{item.label}</span>
                  {!item.enabled && (
                    <span className="ml-auto text-[10px] px-1.5 py-0.5 rounded bg-bg-tertiary text-text-muted">
                      Soon
                    </span>
                  )}
                </button>

                {/* Coming Soon tooltip */}
                {!item.enabled && (
                  <div className="absolute left-full ml-2 top-1/2 -translate-y-1/2 px-2 py-1 bg-bg-elevated rounded text-xs text-text-secondary whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-50 shadow-elevated">
                    Coming Soon
                  </div>
                )}
              </li>
            );
          })}
        </ul>
      </nav>

      {/* Version */}
      <div className="px-6 py-4 border-t border-border">
        <p className="text-xs text-text-muted">v{APP_VERSION}</p>
      </div>
    </aside>
  );
}
