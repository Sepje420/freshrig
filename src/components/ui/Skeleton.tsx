export function SkeletonLine({
  width = "100%",
  height = "1rem",
}: {
  width?: string;
  height?: string;
}) {
  return <div className="bg-bg-card-hover rounded animate-pulse" style={{ width, height }} />;
}

export function SkeletonCard() {
  return (
    <div className="bg-bg-card rounded-lg border border-border p-4 space-y-3">
      <div className="flex items-center justify-between">
        <SkeletonLine width="40%" height="1.25rem" />
        <SkeletonLine width="0.75rem" height="0.75rem" />
      </div>
      <SkeletonLine width="70%" />
      <SkeletonLine width="50%" />
    </div>
  );
}

export function SkeletonRow() {
  return (
    <div className="flex items-center gap-3 p-3">
      <SkeletonLine width="1.5rem" height="1.5rem" />
      <div className="flex-1 space-y-2">
        <SkeletonLine width="60%" />
        <SkeletonLine width="40%" height="0.75rem" />
      </div>
    </div>
  );
}
