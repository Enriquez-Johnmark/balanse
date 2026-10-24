import { InboxIcon } from "lucide-react"

import { PageHeader } from "@/components/page-header"
import { QueryError } from "@/components/query-error"
import { Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle } from "@/components/ui/empty"
import { Skeleton } from "@/components/ui/skeleton"
import { DraftCard } from "@/features/drafts/draft-card"
import { useDrafts } from "@/features/drafts/queries"

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
      {drafts.data?.length === 0 ? (
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
      ) : null}
      {drafts.data?.map((draft) => (
        <DraftCard key={draft.id} draft={draft} />
      ))}
    </>
  )
}
