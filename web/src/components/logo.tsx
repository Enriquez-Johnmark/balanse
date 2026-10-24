import { cn } from "@/lib/utils"

/** Balanse's name beside two level lines: a balance settled. */
export function Logo({ className }: { className?: string }) {
  return (
    <span className={cn("flex items-center gap-2 font-semibold", className)}>
      <svg viewBox="0 0 32 32" aria-hidden="true" className="size-6">
        <rect width="32" height="32" rx="8" className="fill-primary" />
        <path
          d="M9 12h14M9 20h14"
          fill="none"
          strokeWidth="3.2"
          strokeLinecap="round"
          className="stroke-primary-foreground"
        />
      </svg>
      Balanse
    </span>
  )
}
