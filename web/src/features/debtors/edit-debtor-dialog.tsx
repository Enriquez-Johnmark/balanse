import { zodResolver } from "@hookform/resolvers/zod"
import { useState } from "react"
import { useForm } from "react-hook-form"

import type { DebtorPage } from "@/api/generated/DebtorPage"
import type { NewDebtor } from "@/api/generated/NewDebtor"
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
import { FieldGroup } from "@/components/ui/field"
import { Spinner } from "@/components/ui/spinner"
import { toast } from "@/components/ui/toast"
import { DebtorFields, debtorSchema } from "@/features/debtors/debtor-fields"
import { useUpdateDebtor } from "@/features/debtors/queries"

/** Changes a Debtor and its Contact. After a bounce, a new address is what
 *  starts reminders again (F4); the API decides that. */
export function EditDebtorDialog({ debtor }: { debtor: DebtorPage }) {
  const [isOpen, setIsOpen] = useState(false)
  const update = useUpdateDebtor(debtor.id)
  const form = useForm<NewDebtor>({
    resolver: zodResolver(debtorSchema),
    values: {
      business_name: debtor.business_name,
      contact_name: debtor.contact_name,
      contact_email: debtor.contact_email,
      contact_mobile: debtor.contact_mobile ?? "",
      language: debtor.language,
    },
  })
  const save = (values: NewDebtor) =>
    update.mutate(values, {
      onSuccess: () => {
        setIsOpen(false)
        toast.add({ title: "Details saved", type: "success" })
      },
    })
  return (
    <Dialog
      open={isOpen}
      onOpenChange={(open) => {
        setIsOpen(open)
        if (!open) {
          update.reset()
          form.reset()
        }
      }}
    >
      <DialogTrigger render={<Button variant="outline" />}>Edit details</DialogTrigger>
      <DialogContent className="sm:max-w-2xl">
        <DialogHeader>
          <DialogTitle>Edit {debtor.business_name}</DialogTitle>
          <DialogDescription>
            {debtor.state === "bounced"
              ? "Their email bounced. Reminders start again once you enter a new address."
              : "Reminders go only to this Contact."}
          </DialogDescription>
        </DialogHeader>
        <form onSubmit={form.handleSubmit(save)} noValidate>
          <FieldGroup>
            <DebtorFields form={form} />
            {update.isError ? <SaveError error={update.error} /> : null}
            <DialogFooter>
              <DialogClose render={<Button variant="outline" type="button" />}>Cancel</DialogClose>
              <Button type="submit" disabled={update.isPending}>
                {update.isPending ? <Spinner data-icon="inline-start" /> : null}
                Save
              </Button>
            </DialogFooter>
          </FieldGroup>
        </form>
      </DialogContent>
    </Dialog>
  )
}
