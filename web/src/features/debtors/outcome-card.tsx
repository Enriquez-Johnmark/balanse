import { useState } from "react"

import type { DebtorPage } from "@/api/generated/DebtorPage"
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
import { Field, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Spinner } from "@/components/ui/spinner"
import { useRecordOutcome } from "@/features/debtors/queries"
import { formatDate } from "@/lib/dates"
import { debtorStateLabel } from "@/lib/labels"

/** F3: where reminders stand, and the Owner's decision about them. The API
 *  decides what each choice does. */
export function OutcomeCard({ debtor }: { debtor: DebtorPage }) {
  const record = useRecordOutcome(debtor.id)
  const busy = record.isPending
  const [isConfirming, setIsConfirming] = useState(false)
  // Only the Debtor (or a new address) can lift these; the API refuses too.
  const stopped = debtor.state === "opted_out" || debtor.state === "bounced"
  return (
    <Card>
      <CardHeader>
        <CardTitle>{debtorStateLabel(debtor.state)}</CardTitle>
        <CardDescription>
          {debtor.promised_until
            ? `Reminders start again after ${formatDate(debtor.promised_until)}.`
            : debtor.state === "active"
              ? "Reminders are drafted for you to approve."
              : stopped
                ? "No reminders go to this Debtor. Only they can turn them back on."
                : "No reminders are drafted until you resume them."}
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <div className="flex flex-wrap gap-2">
          <AlertDialog open={isConfirming} onOpenChange={setIsConfirming}>
            <AlertDialogTrigger render={<Button disabled={busy} />}>They paid</AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Mark {debtor.business_name} as paid?</AlertDialogTitle>
                <AlertDialogDescription>
                  Every open invoice is settled and no more reminders are drafted for them.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction
                  onClick={() =>
                    record.mutate(
                      { outcome: "paid", until: null },
                      { onSettled: () => setIsConfirming(false) }
                    )
                  }
                  disabled={busy}
                >
                  {busy ? <Spinner data-icon="inline-start" /> : null}
                  Mark as paid
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
          {stopped ? null : <PromiseDialog debtorId={debtor.id} />}
          <Button
            variant="outline"
            onClick={() => record.mutate({ outcome: "disputed", until: null })}
            disabled={busy}
          >
            They dispute it
          </Button>
          {debtor.state === "active" || stopped ? null : (
            <Button
              variant="outline"
              onClick={() => record.mutate({ outcome: "resume", until: null })}
              disabled={busy}
            >
              Resume reminders
            </Button>
          )}
        </div>
        {record.isError ? (
          <Alert variant="destructive">
            <AlertDescription>{record.error.message}</AlertDescription>
          </Alert>
        ) : null}
      </CardContent>
    </Card>
  )
}

function PromiseDialog({ debtorId }: { debtorId: string }) {
  const [isOpen, setIsOpen] = useState(false)
  const [until, setUntil] = useState("")
  const record = useRecordOutcome(debtorId)
  return (
    <Dialog
      open={isOpen}
      onOpenChange={(open) => {
        setIsOpen(open)
        if (!open) record.reset()
      }}
    >
      <DialogTrigger render={<Button variant="outline" />}>They promised to pay</DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Promised to pay</DialogTitle>
          <DialogDescription>
            No reminders until this date; they start again the day after if it's still unpaid.
          </DialogDescription>
        </DialogHeader>
        <Field>
          <FieldLabel htmlFor="promised-until">Pay by</FieldLabel>
          <Input
            id="promised-until"
            type="date"
            value={until}
            onChange={(event) => setUntil(event.target.value)}
          />
        </Field>
        {record.isError ? (
          <Alert variant="destructive">
            <AlertDescription>{record.error.message}</AlertDescription>
          </Alert>
        ) : null}
        <DialogFooter>
          <DialogClose render={<Button variant="outline" />}>Cancel</DialogClose>
          <Button
            onClick={() =>
              record.mutate({ outcome: "promised", until }, { onSuccess: () => setIsOpen(false) })
            }
            disabled={record.isPending || !until}
          >
            {record.isPending ? <Spinner data-icon="inline-start" /> : null}
            Save promise
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
