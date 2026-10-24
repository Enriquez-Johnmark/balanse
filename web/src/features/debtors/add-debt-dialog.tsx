import { zodResolver } from "@hookform/resolvers/zod"
import { PlusIcon } from "lucide-react"
import { useState } from "react"
import { useForm } from "react-hook-form"
import { z } from "zod"

import type { NewDebt } from "@/api/generated/NewDebt"
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
import { Field, FieldDescription, FieldError, FieldGroup, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Spinner } from "@/components/ui/spinner"
import { Textarea } from "@/components/ui/textarea"
import { useAddDebt } from "@/features/debtors/queries"

const schema = z.object({
  invoice_number: z.string().trim().min(1, "Enter your invoice number."),
  invoice_date: z.string().min(1, "Enter the invoice date."),
  amount: z.string().trim().min(1, "Enter the amount owed."),
  due_date: z.string().min(1, "Enter the due date."),
  note: z.string(),
})

const empty: NewDebt = { invoice_number: "", invoice_date: "", amount: "", due_date: "", note: "" }

/** B1: an unpaid invoice. The amount is kept exactly as typed (B3). */
export function AddDebtDialog({ debtorId }: { debtorId: string }) {
  const [isOpen, setIsOpen] = useState(false)
  const add = useAddDebt(debtorId)
  const form = useForm<NewDebt>({ resolver: zodResolver(schema), defaultValues: empty })
  const errors = form.formState.errors
  const save = (values: NewDebt) => add.mutate(values, { onSuccess: () => setIsOpen(false) })

  return (
    <Dialog
      open={isOpen}
      onOpenChange={(open) => {
        setIsOpen(open)
        if (!open) {
          add.reset()
          form.reset(empty)
        }
      }}
    >
      <DialogTrigger render={<Button variant="outline" size="sm" />}>
        <PlusIcon data-icon="inline-start" />
        Add an invoice
      </DialogTrigger>
      <DialogContent>
        <form onSubmit={form.handleSubmit(save)} noValidate className="flex flex-col gap-6">
          <DialogHeader>
            <DialogTitle>Add an invoice</DialogTitle>
            <DialogDescription>An invoice you issued that hasn't been paid.</DialogDescription>
          </DialogHeader>
          <FieldGroup>
            <div className="grid gap-6 sm:grid-cols-2">
              <Field data-invalid={errors.invoice_number ? true : undefined}>
                <FieldLabel htmlFor="invoice_number">Invoice number</FieldLabel>
                <Input
                  id="invoice_number"
                  aria-invalid={errors.invoice_number ? true : undefined}
                  {...form.register("invoice_number")}
                />
                <FieldError errors={[errors.invoice_number]} />
              </Field>
              <Field data-invalid={errors.amount ? true : undefined}>
                <FieldLabel htmlFor="amount">Amount owed</FieldLabel>
                <Input
                  id="amount"
                  inputMode="decimal"
                  placeholder="₱12,500.00"
                  aria-invalid={errors.amount ? true : undefined}
                  {...form.register("amount")}
                />
                <FieldError errors={[errors.amount]} />
              </Field>
              <Field data-invalid={errors.invoice_date ? true : undefined}>
                <FieldLabel htmlFor="invoice_date">Invoice date</FieldLabel>
                <Input
                  id="invoice_date"
                  type="date"
                  aria-invalid={errors.invoice_date ? true : undefined}
                  {...form.register("invoice_date")}
                />
                <FieldError errors={[errors.invoice_date]} />
              </Field>
              <Field data-invalid={errors.due_date ? true : undefined}>
                <FieldLabel htmlFor="due_date">Due date</FieldLabel>
                <Input
                  id="due_date"
                  type="date"
                  aria-invalid={errors.due_date ? true : undefined}
                  {...form.register("due_date")}
                />
                <FieldError errors={[errors.due_date]} />
              </Field>
            </div>
            <Field>
              <FieldLabel htmlFor="note">Private note (optional)</FieldLabel>
              <Textarea id="note" rows={2} {...form.register("note")} />
              <FieldDescription>Only you see this; it's never in a reminder.</FieldDescription>
            </Field>
            {add.isError ? <SaveError error={add.error} /> : null}
          </FieldGroup>
          <DialogFooter>
            <DialogClose render={<Button variant="outline" />}>Cancel</DialogClose>
            <Button type="submit" disabled={add.isPending}>
              {add.isPending ? <Spinner data-icon="inline-start" /> : null}
              Add invoice
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
