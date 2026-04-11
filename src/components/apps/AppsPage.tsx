import { useEffect, useMemo, useState } from "react";
import {
  Package,
  Search,
  AlertTriangle,
  Download,
  BookMarked,
  Eye,
  EyeOff,
  RefreshCw,
} from "lucide-react";
import { useAppStore } from "../../stores/appStore";
import { AppCard } from "./AppCard";
import { CategoryFilter } from "./CategoryFilter";
import { InstallProgressPanel } from "./InstallProgressPanel";
import { SaveProfileDialog } from "../profiles/SaveProfileDialog";
import { PresetSelector } from "./PresetSelector";
import { WingetSearchResults } from "./WingetSearchResults";

export function AppsPage() {
  const [showSaveProfile, setShowSaveProfile] = useState(false);

  const {
    catalog,
    selectedIds,
    installProgress,
    isInstalling,
    wingetAvailable,
    searchQuery,
    activeCategory,
    loading,
    fetchCatalog,
    checkWinget,
    toggleApp,
    selectAll,
    clearSelection,
    installSelected,
    setSearchQuery,
    setActiveCategory,
    wingetResults,
    isSearchingWinget,
    installedAppIds,
    isCheckingInstalled,
    hideInstalled,
    setHideInstalled,
    checkInstalledApps,
  } = useAppStore();

  useEffect(() => {
    fetchCatalog();
    checkWinget();
  }, [fetchCatalog, checkWinget]);

  const filteredApps = useMemo(() => {
    return catalog.filter((app) => {
      const matchesCategory = activeCategory === "all" || app.category === activeCategory;
      const matchesSearch =
        !searchQuery ||
        app.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        app.description.toLowerCase().includes(searchQuery.toLowerCase());
      const matchesInstalled = !hideInstalled || !installedAppIds.has(app.id);
      return matchesCategory && matchesSearch && matchesInstalled;
    });
  }, [catalog, activeCategory, searchQuery, hideInstalled, installedAppIds]);

  const installedCount = installedAppIds.size;
  const showWingetResults = searchQuery.trim().length >= 2;

  return (
    <div className="space-y-6 pb-20">
      {/* Header */}
      <div className="flex items-start justify-between gap-4">
        <div className="flex items-center gap-3">
          <div className="flex items-center justify-center w-10 h-10 rounded-lg bg-accent-muted">
            <Package className="w-5 h-5 text-accent" />
          </div>
          <div>
            <h1 className="text-2xl font-bold text-text-primary">App Catalog</h1>
            <p className="text-sm text-text-secondary mt-0.5">
              Select apps to install with one click
            </p>
          </div>
        </div>
      </div>

      {/* Preset Selector */}
      <PresetSelector />

      {/* Winget warning */}
      {wingetAvailable === false && (
        <div className="flex items-center gap-3 px-4 py-3 rounded-lg bg-warning/10 border border-warning/20 animate-fade-in">
          <AlertTriangle className="w-5 h-5 text-warning shrink-0" />
          <p className="text-sm text-warning">
            Winget is not detected. Please install{" "}
            <span className="font-semibold">App Installer</span> from the Microsoft Store to enable
            app installation.
          </p>
        </div>
      )}

      {/* Toolbar */}
      <div className="space-y-3">
        <div className="flex items-center gap-3">
          {/* Search */}
          <div className="relative flex-1 max-w-sm">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-text-muted" />
            <input
              type="text"
              placeholder="Search apps or winget repository..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full pl-9 pr-3 py-2 rounded-lg bg-bg-tertiary border border-border text-sm text-text-primary placeholder:text-text-muted focus:outline-none focus:border-accent/50 transition-colors"
            />
          </div>

          {/* Installed filter */}
          {installedCount > 0 && (
            <button
              onClick={() => setHideInstalled(!hideInstalled)}
              className={`flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-xs transition-colors ${
                hideInstalled
                  ? "bg-accent-muted text-accent"
                  : "text-text-muted hover:text-text-primary hover:bg-bg-tertiary"
              }`}
              title={hideInstalled ? "Show installed apps" : "Hide installed apps"}
            >
              {hideInstalled ? (
                <EyeOff className="w-3.5 h-3.5" />
              ) : (
                <Eye className="w-3.5 h-3.5" />
              )}
              {installedCount} installed
            </button>
          )}

          {/* Refresh installed */}
          {installedCount > 0 && (
            <button
              onClick={checkInstalledApps}
              disabled={isCheckingInstalled}
              className="p-1.5 rounded-md text-text-muted hover:text-text-primary hover:bg-bg-tertiary transition-colors"
              title="Re-scan installed apps"
            >
              <RefreshCw
                className={`w-3.5 h-3.5 ${isCheckingInstalled ? "animate-spin" : ""}`}
              />
            </button>
          )}

          {/* Selection info + actions */}
          <div className="flex items-center gap-2 ml-auto">
            {selectedIds.size > 0 && (
              <>
                <span className="text-xs text-text-secondary">
                  {selectedIds.size} app{selectedIds.size !== 1 ? "s" : ""} selected
                </span>
                <button
                  onClick={clearSelection}
                  className="px-2.5 py-1.5 rounded-md text-xs text-text-muted hover:text-text-primary hover:bg-bg-tertiary transition-colors"
                >
                  Clear
                </button>
                <button
                  onClick={() => selectAll(activeCategory)}
                  className="px-2.5 py-1.5 rounded-md text-xs text-text-muted hover:text-text-primary hover:bg-bg-tertiary transition-colors"
                >
                  Select All
                </button>
              </>
            )}
            {selectedIds.size === 0 && (
              <button
                onClick={() => selectAll(activeCategory)}
                className="px-2.5 py-1.5 rounded-md text-xs text-text-muted hover:text-text-primary hover:bg-bg-tertiary transition-colors"
              >
                Select All
              </button>
            )}

            <button
              onClick={() => setShowSaveProfile(true)}
              disabled={selectedIds.size === 0}
              className={`flex items-center gap-2 px-3 py-2 rounded-lg text-sm font-medium transition-all duration-200 ${
                selectedIds.size > 0
                  ? "border border-border text-text-secondary hover:text-text-primary hover:bg-bg-tertiary"
                  : "bg-bg-tertiary text-text-muted cursor-not-allowed"
              }`}
            >
              <BookMarked className="w-4 h-4" />
              Save as Profile
            </button>

            <button
              onClick={installSelected}
              disabled={selectedIds.size === 0 || isInstalling || wingetAvailable === false}
              className={`flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-semibold transition-all duration-200 ${
                selectedIds.size > 0 && !isInstalling && wingetAvailable !== false
                  ? "bg-accent text-bg-primary hover:bg-accent-hover shadow-[0_0_20px_rgba(0,212,170,0.3)] hover:shadow-[0_0_28px_rgba(0,212,170,0.45)]"
                  : "bg-bg-tertiary text-text-muted cursor-not-allowed"
              }`}
            >
              <Download className="w-4 h-4" />
              Install Selected
            </button>
          </div>
        </div>

        {/* Category pills */}
        <CategoryFilter activeCategory={activeCategory} onSelect={setActiveCategory} />
      </div>

      {/* Loading */}
      {loading && (
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-3">
          {Array.from({ length: 12 }).map((_, i) => (
            <div key={i} className="h-20 rounded-lg bg-bg-card border border-border animate-pulse" />
          ))}
        </div>
      )}

      {/* App grid */}
      {!loading && (
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-3">
          {filteredApps.map((app) => (
            <AppCard
              key={app.id}
              app={app}
              selected={selectedIds.has(app.id)}
              progress={installProgress.get(app.id)}
              onToggle={() => toggleApp(app.id)}
              isInstalled={installedAppIds.has(app.id)}
            />
          ))}
        </div>
      )}

      {/* Empty state */}
      {!loading && filteredApps.length === 0 && !showWingetResults && (
        <div className="flex flex-col items-center justify-center py-16 animate-fade-in">
          <Package className="w-12 h-12 text-text-muted mb-4" />
          <h3 className="text-lg font-semibold text-text-primary mb-1">No apps found</h3>
          <p className="text-sm text-text-secondary">Try a different search or category.</p>
        </div>
      )}

      {/* Winget search results */}
      {showWingetResults && (
        <WingetSearchResults results={wingetResults} isSearching={isSearchingWinget} />
      )}

      {/* Install progress panel */}
      {installProgress.size > 0 && (
        <InstallProgressPanel
          progress={installProgress}
          onDone={() => useAppStore.setState({ installProgress: new Map() })}
        />
      )}

      {/* Save as Profile dialog */}
      {showSaveProfile && (
        <SaveProfileDialog
          onClose={() => setShowSaveProfile(false)}
          onSaved={() => setShowSaveProfile(false)}
        />
      )}
    </div>
  );
}
