import { zodResolver } from "@hookform/resolvers/zod"
import { useState, type ReactNode } from "react"
import { useForm } from "react-hook-form"
import { z } from "zod"

import type { DraftRow } from "@/api/generated/DraftRow"
import type { Edit } from "@/api/generated/Edit"
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
import { Field, FieldDescription, FieldError, FieldGroup, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Spinner } from "@/components/ui/spinner"
import { Textarea } from "@/components/ui/textarea"
import { useEditDraft } from "@/features/drafts/queries"

const schema = z.object({
  subject: z.string().trim().min(1, "Write a subject."),
  body: z.string().trim().min(1, "Write the message."),
})

type EditDraftDialogProps = { draft: DraftRow; children: ReactNode }

/** D2: new wording for a Draft. Saving cancels any earlier approval; the
 *  recipient can't be changed here. */
export function EditDraftDialog({ draft, children }: EditDraftDialogProps) {
  const [isOpen, setIsOpen] = useState(false)
  const edit = useEditDraft()
  const form = useForm<Edit>({
    resolver: zodResolver(schema),
    values: { subject: draft.subject, body: draft.body },
  })

  function handleOpenChange(open: boolean) {
    setIsOpen(open)
    if (!open) edit.reset()
  }

  return (
    <Dialog open={isOpen} onOpenChange={handleOpenChange}>
      <DialogTrigger render={<Button variant="outline" />}>{children}</DialogTrigger>
      <DialogContent className="sm:max-w-2xl">
        <DialogHeader>
          <DialogTitle>Edit the reminder to {draft.debtor_name}</DialogTitle>
          <DialogDescription>
            To {draft.to_address}. You approve the new text afterwards.
          </DialogDescription>
        </DialogHeader>
        <form
          id={`edit-${draft.id}`}
          onSubmit={form.handleSubmit((values) =>
            edit.mutate({ id: draft.id, edit: values }, { onSuccess: () => setIsOpen(false) })
          )}
          noValidate
        >
          <FieldGroup>
            <Field data-invalid={form.formState.errors.subject ? true : undefined}>
              <FieldLabel htmlFor={`subject-${draft.id}`}>Subject</FieldLabel>
              <Input
                id={`subject-${draft.id}`}
                aria-invalid={form.formState.errors.subject ? true : undefined}
                {...form.register("subject")}
              />
              <FieldError errors={[form.formState.errors.subject]} />
            </Field>
            <Field data-invalid={form.formState.errors.body ? true : undefined}>
              <FieldLabel htmlFor={`body-${draft.id}`}>Message</FieldLabel>
              <Textarea
                id={`body-${draft.id}`}
                rows={12}
                aria-invalid={form.formState.errors.body ? true : undefined}
                {...form.register("body")}
              />
              <FieldDescription>
                Keep the list of invoices and the closing line as they are, and add no links: a
                reminder without them, or with a link, can't be sent.
              </FieldDescription>
              <FieldError errors={[form.formState.errors.body]} />
            </Field>
            {edit.isError ? (
              <Alert variant="destructive">
                <AlertDescription>{edit.error.message}</AlertDescription>
              </Alert>
            ) : null}
          </FieldGroup>
        </form>
        <DialogFooter>
          <DialogClose render={<Button variant="outline" />}>Cancel</DialogClose>
          <Button type="submit" form={`edit-${draft.id}`} disabled={edit.isPending}>
            {edit.isPending ? <Spinner data-icon="inline-start" /> : null}
            Save
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
