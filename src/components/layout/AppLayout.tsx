import { Sidebar } from "./Sidebar";
import { TitleBar } from "./TitleBar";

interface AppLayoutProps {
  currentView: string;
  onNavigate: (view: string) => void;
  children: React.ReactNode;
}

export function AppLayout({ currentView, onNavigate, children }: AppLayoutProps) {
  return (
    <div className="flex flex-col h-screen bg-bg-primary">
      <TitleBar />
      <div className="flex flex-1 min-h-0">
        <Sidebar currentView={currentView} onNavigate={onNavigate} />
        <main className="flex-1 min-h-0 overflow-y-auto">
          <div className="p-8">{children}</div>
        </main>
      </div>
    </div>
  );
}
