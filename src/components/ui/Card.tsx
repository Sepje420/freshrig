import { forwardRef } from "react";
import type { HTMLAttributes } from "react";

export interface CardProps extends HTMLAttributes<HTMLDivElement> {
  interactive?: boolean;
}

export const Card = forwardRef<HTMLDivElement, CardProps>(function Card(
  { className = "", interactive = false, children, ...props },
  ref,
) {
  const base =
    "bg-[var(--bg-card)] border border-[var(--border)] rounded-xl transition-colors duration-150 shadow-[inset_0_1px_0_rgba(255,255,255,0.04)]";
  const hover = interactive
    ? "hover:border-[var(--border-hover)] hover:bg-[var(--bg-card-hover)]"
    : "hover:border-[var(--border-hover)]";

  return (
    <div ref={ref} className={`${base} ${hover} ${className}`} {...props}>
      {children}
    </div>
  );
});
