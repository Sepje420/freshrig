import type { AppCategory } from "../../types/apps";

interface CategoryFilterProps {
  activeCategory: AppCategory | "all";
  onSelect: (cat: AppCategory | "all") => void;
}

const categories: { id: AppCategory | "all"; label: string }[] = [
  { id: "all", label: "All" },
  { id: "Browser", label: "Browsers" },
  { id: "Gaming", label: "Gaming" },
  { id: "Communication", label: "Communication" },
  { id: "Development", label: "Development" },
  { id: "Media", label: "Media" },
  { id: "Productivity", label: "Productivity" },
  { id: "Utilities", label: "Utilities" },
  { id: "Security", label: "Security" },
  { id: "Runtime", label: "Runtimes" },
];

export function CategoryFilter({ activeCategory, onSelect }: CategoryFilterProps) {
  return (
    <div className="flex gap-1.5 overflow-x-auto pb-1 scrollbar-none">
      {categories.map((cat) => (
        <button
          key={cat.id}
          onClick={() => onSelect(cat.id)}
          className={`px-3 py-1.5 rounded-full text-xs font-medium whitespace-nowrap transition-all duration-200 ${
            activeCategory === cat.id
              ? "bg-accent text-bg-primary"
              : "bg-bg-tertiary text-text-secondary hover:bg-bg-card-hover hover:text-text-primary"
          }`}
        >
          {cat.label}
        </button>
      ))}
    </div>
  );
}
