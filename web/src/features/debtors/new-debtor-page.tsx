import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
import { useNavigate } from "react-router"
import { z } from "zod"

import type { NewDebtor } from "@/api/generated/NewDebtor"
import { PageHeader } from "@/components/page-header"
import { SaveError } from "@/components/save-error"
import { Button } from "@/components/ui/button"
import { Field, FieldDescription, FieldError, FieldGroup, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { NativeSelect, NativeSelectOption } from "@/components/ui/native-select"
import { Spinner } from "@/components/ui/spinner"
import { toast } from "@/components/ui/toast"
import { useCreateDebtor } from "@/features/debtors/queries"

const schema = z.object({
  business_name: z.string().trim().min(1, "Enter the client's business name."),
  contact_name: z.string().trim().min(1, "Enter the name of the person who pays."),
  contact_email: z.email("Enter an email address, e.g. nena@example.com."),
  contact_mobile: z.string(),
  language: z.string(),
})

/** B1: a Debtor and the one Contact who receives reminders. The browser
 *  checks for convenience; the API's checks are final. */
export function NewDebtorPage() {
  const navigate = useNavigate()
  const create = useCreateDebtor()
  const form = useForm<NewDebtor>({
    resolver: zodResolver(schema),
    defaultValues: {
      business_name: "",
      contact_name: "",
      contact_email: "",
      contact_mobile: "",
      language: "taglish",
    },
  })
  const errors = form.formState.errors
  const save = (values: NewDebtor) =>
    create.mutate(values, {
      onSuccess: ({ id }) => {
        toast.add({ title: "Debtor added", type: "success" })
        navigate(`/debtors/${id}`)
      },
    })

  return (
    <>
      <PageHeader
        title="Add a Debtor"
        description="Reminders go only to this Contact, never to anyone else."
      />
      <form onSubmit={form.handleSubmit(save)} noValidate className="max-w-2xl">
        <FieldGroup>
          <Field data-invalid={errors.business_name ? true : undefined}>
            <FieldLabel htmlFor="business_name">Client's business name</FieldLabel>
            <Input
              id="business_name"
              aria-invalid={errors.business_name ? true : undefined}
              {...form.register("business_name")}
            />
            <FieldError errors={[errors.business_name]} />
          </Field>
          <div className="grid gap-6 sm:grid-cols-2">
            <Field data-invalid={errors.contact_name ? true : undefined}>
              <FieldLabel htmlFor="contact_name">Contact's name</FieldLabel>
              <Input
                id="contact_name"
                aria-invalid={errors.contact_name ? true : undefined}
                {...form.register("contact_name")}
              />
              <FieldError errors={[errors.contact_name]} />
            </Field>
            <Field data-invalid={errors.contact_email ? true : undefined}>
              <FieldLabel htmlFor="contact_email">Contact's email</FieldLabel>
              <Input
                id="contact_email"
                type="email"
                aria-invalid={errors.contact_email ? true : undefined}
                {...form.register("contact_email")}
              />
              <FieldError errors={[errors.contact_email]} />
            </Field>
          </div>
          <div className="grid gap-6 sm:grid-cols-2">
            <Field>
              <FieldLabel htmlFor="contact_mobile">Contact's mobile (optional)</FieldLabel>
              <Input
                id="contact_mobile"
                type="tel"
                autoComplete="off"
                {...form.register("contact_mobile")}
              />
              <FieldDescription>For copying a reminder into Viber or SMS.</FieldDescription>
            </Field>
            <Field>
              <FieldLabel htmlFor="language">Reminder language</FieldLabel>
              <NativeSelect id="language" {...form.register("language")}>
                <NativeSelectOption value="taglish">Taglish</NativeSelectOption>
                <NativeSelectOption value="english">English</NativeSelectOption>
              </NativeSelect>
            </Field>
          </div>
          {create.isError ? <SaveError error={create.error} /> : null}
          <Button type="submit" className="self-start" disabled={create.isPending}>
            {create.isPending ? <Spinner data-icon="inline-start" /> : null}
            Add Debtor
          </Button>
        </FieldGroup>
      </form>
    </>
  )
}
