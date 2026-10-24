import { useState } from "react"

import type { DebtRow } from "@/api/generated/DebtRow"
import { SaveError } from "@/components/save-error"
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
import { useChangeAmount } from "@/features/debtors/queries"

/** B3: the amount still owed after a part payment or a correction. The
 *  change is recorded in the Evidence log. */
export function ChangeAmountDialog({ debt }: { debt: DebtRow }) {
  const [isOpen, setIsOpen] = useState(false)
  const [amount, setAmount] = useState(debt.amount)
  const change = useChangeAmount(debt.id)
  return (
    <Dialog
      open={isOpen}
      onOpenChange={(open) => {
        setIsOpen(open)
        if (open) setAmount(debt.amount)
        else change.reset()
      }}
    >
      <DialogTrigger render={<Button variant="ghost" size="sm" />}>Change</DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Amount owed on {debt.invoice_number}</DialogTitle>
          <DialogDescription>
            After a part payment or a correction. A reminder still waiting is drafted again with the
            new amount.
          </DialogDescription>
        </DialogHeader>
        <Field>
          <FieldLabel htmlFor={`amount-${debt.id}`}>Amount still owed</FieldLabel>
          <Input
            id={`amount-${debt.id}`}
            inputMode="decimal"
            value={amount}
            onChange={(event) => setAmount(event.target.value)}
          />
        </Field>
        {change.isError ? <SaveError error={change.error} /> : null}
        <DialogFooter>
          <DialogClose render={<Button variant="outline" />}>Cancel</DialogClose>
          <Button
            onClick={() => change.mutate({ amount }, { onSuccess: () => setIsOpen(false) })}
            disabled={change.isPending || !amount.trim()}
          >
            {change.isPending ? <Spinner data-icon="inline-start" /> : null}
            Save amount
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
