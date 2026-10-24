import { CheckCheckIcon } from "lucide-react"
import { useState } from "react"

import type { Approved } from "@/api/generated/Approved"
import type { DraftRow } from "@/api/generated/DraftRow"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { Spinner } from "@/components/ui/spinner"
import { toast } from "@/components/ui/toast"
import { approvedToast, DraftHeading } from "@/features/drafts/draft-card"
import { MessageText } from "@/features/drafts/message-text"
import { useApprove } from "@/features/drafts/queries"

type ReviewBatchDialogProps = {
  drafts: DraftRow[]
  /** Called once the batch is approved, to clear the selection. */
  onApproved: () => void
}

/** D2: every selected Draft in full, before any of them is approved. Each
 *  approval is bound to the text shown here. */
export function ReviewBatchDialog({ drafts, onApproved }: ReviewBatchDialogProps) {
  const [isOpen, setIsOpen] = useState(false)
  const [refused, setRefused] = useState<Approved[]>([])
  // The drafts as they were when the review opened. A refresh of the list
  // while reading never changes the text being reviewed, or what's approved:
  // a draft edited meanwhile is refused by its hash instead.
  const [reviewed, setReviewed] = useState<DraftRow[]>([])
  const approve = useApprove()
  const count = isOpen ? reviewed.length : drafts.length

  function handleOpenChange(open: boolean) {
    setIsOpen(open)
    if (open) setReviewed(drafts)
    else {
      setRefused([])
      approve.reset()
    }
  }

  function handleApprove() {
    approve.mutate(reviewed, {
      onSuccess: (answers) => {
        onApproved()
        const approved = answers.filter((answer) => answer.approved).length
        if (approved > 0) toast.add({ ...approvedToast, title: `${approved} approved` })
        const notApproved = answers.filter((answer) => !answer.approved)
        if (notApproved.length === 0) setIsOpen(false)
        else setRefused(notApproved)
      },
    })
  }

  return (
    <Dialog open={isOpen} onOpenChange={handleOpenChange}>
      <DialogTrigger render={<Button disabled={count === 0} />}>
        <CheckCheckIcon data-icon="inline-start" />
        Review {count} selected
      </DialogTrigger>
      <DialogContent className="flex max-h-[90svh] flex-col sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle>Review {count === 1 ? "1 draft" : `${count} drafts`}</DialogTitle>
          <DialogDescription>
            Read each message as it will be sent. Nothing is approved until you press the button
            below.
          </DialogDescription>
        </DialogHeader>
        {refused.length > 0 ? (
          <Alert variant="warning">
            <AlertTitle>
              {refused.length === 1
                ? "1 draft wasn't approved"
                : `${refused.length} drafts weren't approved`}
            </AlertTitle>
            <AlertDescription>
              <ul className="flex flex-col gap-1">
                {refused.map((refusal) => (
                  <li key={refusal.id}>
                    {reviewed.find((draft) => draft.id === refusal.id)?.debtor_name}:{" "}
                    {refusal.reason}
                  </li>
                ))}
              </ul>
            </AlertDescription>
          </Alert>
        ) : (
          <ol className="-mx-1 flex flex-col gap-4 overflow-y-auto px-1">
            {reviewed.map((draft) => (
              <li key={draft.id} className="flex flex-col gap-2">
                <DraftHeading draft={draft} />
                <MessageText subject={draft.subject} body={draft.body} />
              </li>
            ))}
          </ol>
        )}
        {approve.isError ? (
          <Alert variant="destructive">
            <AlertDescription>{approve.error.message}</AlertDescription>
          </Alert>
        ) : null}
        <DialogFooter>
          <DialogClose render={<Button variant="outline" />}>
            {refused.length > 0 ? "Close" : "Cancel"}
          </DialogClose>
          {refused.length > 0 ? null : (
            <Button onClick={handleApprove} disabled={approve.isPending}>
              {approve.isPending ? <Spinner data-icon="inline-start" /> : null}
              Approve {count === 1 ? "this draft" : `these ${count}`}
            </Button>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
