import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
import { useNavigate } from "react-router"

import type { NewDebtor } from "@/api/generated/NewDebtor"
import { PageHeader } from "@/components/page-header"
import { SaveError } from "@/components/save-error"
import { Button } from "@/components/ui/button"
import { FieldGroup } from "@/components/ui/field"
import { Spinner } from "@/components/ui/spinner"
import { toast } from "@/components/ui/toast"
import { DebtorFields, debtorSchema } from "@/features/debtors/debtor-fields"
import { useCreateDebtor } from "@/features/debtors/queries"

/** B1: a Debtor and the one Contact who receives reminders. The browser
 *  checks for convenience; the API's checks are final. */
export function NewDebtorPage() {
  const navigate = useNavigate()
  const create = useCreateDebtor()
  const form = useForm<NewDebtor>({
    resolver: zodResolver(debtorSchema),
    defaultValues: {
      business_name: "",
      contact_name: "",
      contact_email: "",
      contact_mobile: "",
      language: "taglish",
    },
  })
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
          <DebtorFields form={form} />
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
