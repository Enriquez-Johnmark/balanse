import type { UseFormReturn } from "react-hook-form"
import { z } from "zod"

import type { NewDebtor } from "@/api/generated/NewDebtor"
import { Field, FieldDescription, FieldError, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { NativeSelect, NativeSelectOption } from "@/components/ui/native-select"

/** The browser's checks, for convenience; the API's are final. */
export const debtorSchema = z.object({
  business_name: z.string().trim().min(1, "Enter the client's business name."),
  contact_name: z.string().trim().min(1, "Enter the name of the person who pays."),
  contact_email: z.email("Enter an email address, e.g. nena@example.com."),
  contact_mobile: z.string(),
  language: z.string(),
})

/** A Debtor's fields and its one Contact's (B1), for adding or changing. */
export function DebtorFields({ form }: { form: UseFormReturn<NewDebtor> }) {
  const errors = form.formState.errors
  return (
    <>
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
    </>
  )
}
