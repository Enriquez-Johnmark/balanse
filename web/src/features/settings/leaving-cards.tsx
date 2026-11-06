import { DownloadIcon } from "lucide-react"
import { useState } from "react"

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
import { ActionError } from "@/features/settings/action-error"
import { useCloseAccount, useDownloadData } from "@/features/settings/queries"
import { formatDay } from "@/lib/dates"

/** H5: everything the Business has in Balanse, to keep. */
export function DownloadDataCard() {
  const download = useDownloadData()
  return (
    <Card>
      <CardHeader>
        <CardTitle>Your data</CardTitle>
        <CardDescription>
          Your Debtors, their invoices and every message, as one file. It needs a login from the
          last 10 minutes.
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col items-start gap-4">
        <ActionError error={download.error} />
        <Button variant="outline" onClick={() => download.mutate()} disabled={download.isPending}>
          {download.isPending ? (
            <Spinner data-icon="inline-start" />
          ) : (
            <DownloadIcon data-icon="inline-start" />
          )}
          Download all your data
        </Button>
      </CardContent>
    </Card>
  )
}

/** H5: closing stops everything now and erases it after a week. */
export function CloseAccountCard({ businessName }: { businessName: string }) {
  const [typed, setTyped] = useState("")
  const close = useCloseAccount()
  return (
    <Card>
      <CardHeader>
        <CardTitle>Close your account</CardTitle>
        <CardDescription>
          Reminders stop at once and you're logged out. Everything is deleted after 7 days.
        </CardDescription>
      </CardHeader>
      <CardContent>
        {close.data ? (
          <p className="text-sm">
            Your account is closed. Everything is deleted on {formatDay(close.data.erase_after)}.{" "}
            <a href="/login" className="underline underline-offset-4">
              Go to the login page
            </a>
          </p>
        ) : (
          <Dialog
            onOpenChange={(open) => {
              if (!open) {
                setTyped("")
                close.reset()
              }
            }}
          >
            <DialogTrigger render={<Button variant="destructive" />}>Close account</DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Close your account?</DialogTitle>
                <DialogDescription>
                  Download your data first if you want to keep it. To confirm, type{" "}
                  <strong>{businessName}</strong>.
                </DialogDescription>
              </DialogHeader>
              <Field>
                <FieldLabel htmlFor="typed-name">Your business name</FieldLabel>
                <Input
                  id="typed-name"
                  autoComplete="off"
                  value={typed}
                  onChange={(event) => setTyped(event.target.value)}
                />
              </Field>
              <ActionError error={close.error} />
              <DialogFooter>
                <DialogClose render={<Button variant="outline" />}>Keep my account</DialogClose>
                <Button
                  variant="destructive"
                  onClick={() => close.mutate({ typed_name: typed })}
                  disabled={close.isPending || typed.trim() === ""}
                >
                  {close.isPending ? <Spinner data-icon="inline-start" /> : null}
                  Close account
                </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        )}
      </CardContent>
    </Card>
  )
}
