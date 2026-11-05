import { useState } from "react"

import { Alert, AlertDescription } from "@/components/ui/alert"
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
import { Field, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Spinner } from "@/components/ui/spinner"
import { toast } from "@/components/ui/toast"
import { useSentByHand } from "@/features/debtors/queries"
import { todayInManila } from "@/lib/dates"

/** H4: "I sent it on …", so the frequency cap counts a message the Owner
 *  sent by Viber, WhatsApp or SMS. */
export function SentByHandDialog({ debtorId }: { debtorId: string }) {
  const [isOpen, setIsOpen] = useState(false)
  const [sentOn, setSentOn] = useState(todayInManila)
  const log = useSentByHand(debtorId)
  return (
    <Dialog
      open={isOpen}
      onOpenChange={(open) => {
        setIsOpen(open)
        if (!open) log.reset()
      }}
    >
      <DialogTrigger render={<Button variant="outline" size="sm" />}>
        I sent one myself
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>I sent a reminder myself</DialogTitle>
          <DialogDescription>
            By Viber, Messenger, WhatsApp or SMS. Balanse counts it, so the next email waits long
            enough not to crowd them.
          </DialogDescription>
        </DialogHeader>
        <Field>
          <FieldLabel htmlFor="sent-on">I sent it on</FieldLabel>
          <Input
            id="sent-on"
            type="date"
            value={sentOn}
            onChange={(event) => setSentOn(event.target.value)}
          />
        </Field>
        {log.isError ? (
          <Alert variant="destructive">
            <AlertDescription>{log.error.message}</AlertDescription>
          </Alert>
        ) : null}
        <DialogFooter>
          <DialogClose render={<Button variant="outline" />}>Cancel</DialogClose>
          <Button
            onClick={() =>
              log.mutate(
                { sent_on: sentOn },
                {
                  onSuccess: () => {
                    setIsOpen(false)
                    toast.add({ title: "Noted", type: "success" })
                  },
                }
              )
            }
            disabled={log.isPending || !sentOn}
          >
            {log.isPending ? <Spinner data-icon="inline-start" /> : null}
            Save
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
