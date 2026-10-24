import { CheckIcon, PencilIcon, Trash2Icon } from "lucide-react"

import type { DraftRow } from "@/api/generated/DraftRow"
import { Alert, AlertDescription } from "@/components/ui/alert"
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardFooter, CardHeader } from "@/components/ui/card"
import { Spinner } from "@/components/ui/spinner"
import { toast } from "@/components/ui/toast"
import { EditDraftDialog } from "@/features/drafts/edit-draft-dialog"
import { MessageText } from "@/features/drafts/message-text"
import { useApprove, useDiscard } from "@/features/drafts/queries"
import { formatDateTime } from "@/lib/dates"
import { stepLabel } from "@/lib/labels"

/** Approved emails wait for sending hours (C5). */
export const approvedToast = {
  title: "Approved",
  description: "It's sent on a weekday between 08:00 and 20:00, Manila time.",
  type: "success",
}

/** One Draft waiting for approval, shown in full (D1). */
export function DraftCard({ draft }: { draft: DraftRow }) {
  const approve = useApprove()
  const discard = useDiscard()
  const refusal = approve.data?.find((answer) => !answer.approved)?.reason
  const error = approve.error?.message ?? refusal ?? discard.error?.message
  return (
    <Card>
      <CardHeader>
        <DraftHeading draft={draft} />
      </CardHeader>
      <CardContent className="flex flex-col gap-3">
        <MessageText subject={draft.subject} body={draft.body} />
        {error ? (
          <Alert variant="destructive">
            <AlertDescription>{error}</AlertDescription>
          </Alert>
        ) : null}
      </CardContent>
      <CardFooter className="flex flex-wrap gap-2">
        <Button
          onClick={() =>
            approve.mutate([draft], {
              onSuccess: (answers) => {
                if (answers.every((answer) => answer.approved)) toast.add(approvedToast)
              },
            })
          }
          disabled={approve.isPending}
        >
          {approve.isPending ? (
            <Spinner data-icon="inline-start" />
          ) : (
            <CheckIcon data-icon="inline-start" />
          )}
          Approve
        </Button>
        <EditDraftDialog draft={draft}>
          <PencilIcon data-icon="inline-start" />
          Edit
        </EditDraftDialog>
        <AlertDialog>
          <AlertDialogTrigger render={<Button variant="ghost" />}>
            <Trash2Icon data-icon="inline-start" />
            Discard
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>Discard this draft?</AlertDialogTitle>
              <AlertDialogDescription>
                The reminder to {draft.debtor_name} won't be sent. Later reminders still come on
                schedule.
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel>Keep it</AlertDialogCancel>
              <AlertDialogAction variant="destructive" onClick={() => discard.mutate(draft)}>
                Discard
              </AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      </CardFooter>
    </Card>
  )
}

/** Who a reminder goes to, which step it is and when it expires. */
export function DraftHeading({ draft }: { draft: DraftRow }) {
  return (
    <div className="flex min-w-0 flex-1 flex-col gap-1">
      <div className="flex flex-wrap items-center gap-2">
        <span className="font-medium">{draft.debtor_name}</span>
        <Badge variant="outline">{stepLabel(draft.step)}</Badge>
      </div>
      <p className="text-sm text-muted-foreground">
        To {draft.to_address}
        {draft.expires_at ? ` · Expires ${formatDateTime(draft.expires_at)}` : null}
      </p>
    </div>
  )
}
