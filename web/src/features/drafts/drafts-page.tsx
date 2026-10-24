import { InboxIcon } from "lucide-react"
import { useState } from "react"

import { PageHeader } from "@/components/page-header"
import type { DraftRow } from "@/api/generated/DraftRow"
import { QueryError } from "@/components/query-error"
import { Checkbox } from "@/components/ui/checkbox"
import { Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle } from "@/components/ui/empty"
import { Label } from "@/components/ui/label"
import { Skeleton } from "@/components/ui/skeleton"
import { DraftCard } from "@/features/drafts/draft-card"
import { useDrafts } from "@/features/drafts/queries"
import { ReviewBatchDialog } from "@/features/drafts/review-batch-dialog"

/** Every reminder waiting for the Owner's approval (D1–D2). */
export function DraftsPage() {
  const drafts = useDrafts()
  return (
    <>
      <PageHeader
        title="Drafts"
        description="Nothing is sent until you approve it. Drafts expire after three days."
      />
      {drafts.isPending ? <Skeleton className="h-64" /> : null}
      {drafts.isError ? <QueryError error={drafts.error} /> : null}
      {drafts.data ? <WaitingDrafts drafts={drafts.data} /> : null}
    </>
  )
}

function WaitingDrafts({ drafts }: { drafts: DraftRow[] }) {
  const [picked, setPicked] = useState<ReadonlySet<string>>(new Set())
  // Only Drafts still waiting count, e.g. after one was approved alone.
  const selected = drafts.filter((draft) => picked.has(draft.id))
  const allSelected = drafts.length > 0 && selected.length === drafts.length

  if (drafts.length === 0) {
    return (
      <Empty className="border">
        <EmptyHeader>
          <EmptyMedia variant="icon">
            <InboxIcon />
          </EmptyMedia>
          <EmptyTitle>No drafts are waiting</EmptyTitle>
          <EmptyDescription>
            New reminders appear here at 07:00 on the days they're due.
          </EmptyDescription>
        </EmptyHeader>
      </Empty>
    )
  }

  function handleSelect(id: string, isSelected: boolean) {
    setPicked((current) => {
      const next = new Set(current)
      if (isSelected) next.add(id)
      else next.delete(id)
      return next
    })
  }

  return (
    <section className="flex flex-col gap-3">
      <div className="sticky top-0 z-10 flex flex-wrap items-center justify-between gap-3 bg-background/95 py-2 backdrop-blur">
        <div className="flex items-center gap-2">
          <Checkbox
            id="select-all"
            checked={allSelected}
            indeterminate={selected.length > 0 && !allSelected}
            onCheckedChange={(checked) =>
              setPicked(checked === true ? new Set(drafts.map((d) => d.id)) : new Set())
            }
          />
          <Label htmlFor="select-all">
            Select all {drafts.length} {drafts.length === 1 ? "draft" : "drafts"}
          </Label>
        </div>
        <ReviewBatchDialog drafts={selected} onApproved={() => setPicked(new Set())} />
      </div>
      {drafts.map((draft) => (
        <DraftCard
          key={draft.id}
          draft={draft}
          isSelected={picked.has(draft.id)}
          onSelectedChange={(isSelected) => handleSelect(draft.id, isSelected)}
        />
      ))}
    </section>
  )
}
