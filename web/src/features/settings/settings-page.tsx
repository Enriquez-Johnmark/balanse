import { useMutation } from "@tanstack/react-query"
import { useState } from "react"

import { api, ApiError } from "@/api/client"
import type { Settings } from "@/api/generated/Settings"
import { PageHeader } from "@/components/page-header"
import { QueryError } from "@/components/query-error"
import { SaveError } from "@/components/save-error"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Field, FieldDescription, FieldLabel } from "@/components/ui/field"
import { Skeleton } from "@/components/ui/skeleton"
import { Spinner } from "@/components/ui/spinner"
import { Textarea } from "@/components/ui/textarea"
import { toast } from "@/components/ui/toast"
import { useSaveInstructions, useSettings } from "@/features/settings/queries"

export function SettingsPage() {
  const settings = useSettings()
  return (
    <>
      <PageHeader title="Settings" />
      {settings.isPending ? <Skeleton className="h-64" /> : null}
      {settings.isError ? <QueryError error={settings.error} /> : null}
      {settings.data ? <InstructionsCard settings={settings.data} /> : null}
    </>
  )
}

/** G6: how Debtors pay, in the Owner's own words, shown on every Debtor's
 *  page. Balanse never adds payment details of its own (R12). */
function InstructionsCard({ settings }: { settings: Settings }) {
  const [text, setText] = useState(settings.payment_instructions)
  const save = useSaveInstructions()
  const needsLogin = save.error instanceof ApiError && save.error.status === 403
  const logOutToLogIn = useMutation({
    mutationFn: () => api("POST", "/api/logout"),
    onSettled: () => location.assign("/login"),
  })
  return (
    <Card>
      <CardHeader>
        <CardTitle>Payment instructions</CardTitle>
        <CardDescription>
          Shown to your Debtors on their private page. Changing them needs a login from the last 10
          minutes, so nobody with a stolen session can redirect your payments.
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <Field>
          <FieldLabel htmlFor="payment-instructions">How your Debtors pay you</FieldLabel>
          <Textarea
            id="payment-instructions"
            rows={5}
            maxLength={1000}
            placeholder="e.g. GCash 0917 123 4567 (Ana Reyes), or BDO account 0000 1111 2222"
            value={text}
            onChange={(event) => setText(event.target.value)}
          />
          <FieldDescription>
            Your own account details only. Up to 1,000 characters.
          </FieldDescription>
        </Field>
        {needsLogin ? (
          <Alert variant="warning">
            <AlertTitle>Log in again to save this</AlertTitle>
            <AlertDescription className="flex flex-col items-start gap-2">
              <span>{save.error?.message}</span>
              <Button
                variant="outline"
                size="sm"
                onClick={() => logOutToLogIn.mutate()}
                disabled={logOutToLogIn.isPending}
              >
                Log in again
              </Button>
            </AlertDescription>
          </Alert>
        ) : save.isError ? (
          <SaveError error={save.error} />
        ) : null}
        <Button
          className="self-start"
          onClick={() =>
            save.mutate(
              { payment_instructions: text },
              {
                onSuccess: () =>
                  toast.add({ title: "Payment instructions saved", type: "success" }),
              }
            )
          }
          disabled={save.isPending || text === settings.payment_instructions}
        >
          {save.isPending ? <Spinner data-icon="inline-start" /> : null}
          Save
        </Button>
      </CardContent>
    </Card>
  )
}
