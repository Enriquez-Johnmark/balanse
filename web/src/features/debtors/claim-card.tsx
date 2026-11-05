import { FileTextIcon, ImageIcon } from "lucide-react"
import { useState } from "react"

import type { ClaimRow } from "@/api/generated/ClaimRow"
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
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Spinner } from "@/components/ui/spinner"
import { useAnswerClaim } from "@/features/debtors/queries"
import { formatDateTime } from "@/lib/dates"

/** G2: the Contact says they've paid. Reminders wait until the Owner
 *  confirms or rejects the claim; proof files only download (G5), so
 *  nothing a Contact sent is ever shown in the page. */
export function ClaimCard({ claim, debtorName }: { claim: ClaimRow; debtorName: string }) {
  const answer = useAnswerClaim(claim.id)
  const [isConfirming, setIsConfirming] = useState(false)
  return (
    <Card>
      <CardHeader>
        <CardTitle>{debtorName} says they've paid</CardTitle>
        <CardDescription>
          {formatDateTime(claim.claimed_at)}. Reminders are paused until you confirm or reject it.
          Fake receipts exist, so check your account before confirming.
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        {claim.files.length === 0 ? (
          <p className="text-sm text-muted-foreground">No proof attached.</p>
        ) : (
          <ul className="flex flex-col gap-2 text-sm">
            {claim.files.map((file, index) => (
              <li key={file.id}>
                <a
                  href={`/api/claims/${claim.id}/files/${file.id}`}
                  className="inline-flex items-center gap-2 underline underline-offset-4"
                >
                  {file.media_type === "application/pdf" ? (
                    <FileTextIcon className="size-4" />
                  ) : (
                    <ImageIcon className="size-4" />
                  )}
                  Download proof {index + 1} ({Math.ceil(file.size_bytes / 1024)} KB)
                </a>
              </li>
            ))}
          </ul>
        )}
        <div className="flex flex-wrap gap-2">
          <AlertDialog open={isConfirming} onOpenChange={setIsConfirming}>
            <AlertDialogTrigger render={<Button disabled={answer.isPending} />}>
              Confirm payment
            </AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Confirm {debtorName}'s payment?</AlertDialogTitle>
                <AlertDialogDescription>
                  Every open invoice is marked paid and no more reminders are drafted for them.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction
                  onClick={() =>
                    answer.mutate("confirm", { onSettled: () => setIsConfirming(false) })
                  }
                  disabled={answer.isPending}
                >
                  {answer.isPending ? <Spinner data-icon="inline-start" /> : null}
                  Confirm payment
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
          <Button
            variant="outline"
            onClick={() => answer.mutate("reject")}
            disabled={answer.isPending}
          >
            It hasn't arrived
          </Button>
        </div>
        {answer.isError ? (
          <Alert variant="destructive">
            <AlertDescription>{answer.error.message}</AlertDescription>
          </Alert>
        ) : null}
      </CardContent>
    </Card>
  )
}
