import { DownloadIcon } from "lucide-react"
import { useState } from "react"
import { Link } from "react-router"

import type { ImportResult } from "@/api/generated/ImportResult"
import { PageHeader } from "@/components/page-header"
import { SaveError } from "@/components/save-error"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Field, FieldDescription, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Spinner } from "@/components/ui/spinner"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { useImport } from "@/features/import/queries"

/** B2: add many Debtors and their unpaid invoices from one CSV or Excel file. */
export function ImportPage() {
  const [file, setFile] = useState<File | null>(null)
  const upload = useImport()
  return (
    <>
      <PageHeader
        title="Import a spreadsheet"
        description="Add your Debtors and their unpaid invoices from one CSV or Excel file."
      />
      <Card>
        <CardHeader>
          <CardTitle>Your file</CardTitle>
          <CardDescription>
            One row per unpaid invoice. Rows already in Balanse are skipped, so importing the same
            file twice adds nothing new.
          </CardDescription>
        </CardHeader>
        <CardContent className="flex flex-col gap-4">
          <Button
            variant="outline"
            className="self-start"
            render={<a href="/api/import/template" download />}
            nativeButton={false}
          >
            <DownloadIcon data-icon="inline-start" />
            Download the template
          </Button>
          <form
            className="flex flex-col gap-4"
            onSubmit={(event) => {
              event.preventDefault()
              if (file) upload.mutate(file)
            }}
          >
            <Field>
              <FieldLabel htmlFor="import-file">Spreadsheet</FieldLabel>
              <Input
                id="import-file"
                type="file"
                accept=".csv,.xlsx,text/csv,application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                onChange={(event) => {
                  setFile(event.target.files?.[0] ?? null)
                  upload.reset()
                }}
              />
              <FieldDescription>CSV or Excel (.xlsx), up to 2 MB.</FieldDescription>
            </Field>
            {upload.isError ? <SaveError error={upload.error} /> : null}
            <Button type="submit" className="self-start" disabled={!file || upload.isPending}>
              {upload.isPending ? <Spinner data-icon="inline-start" /> : null}
              Import
            </Button>
          </form>
        </CardContent>
      </Card>
      {upload.data ? <ImportSummary result={upload.data} /> : null}
    </>
  )
}

function ImportSummary({ result }: { result: ImportResult }) {
  return (
    <section aria-label="Import result" className="flex flex-col gap-4">
      <Alert variant={result.invalid.length === 0 ? "default" : "warning"}>
        <AlertTitle>
          Added {plural(result.added_debts, "invoice")} for{" "}
          {plural(result.added_debtors, "new Debtor")}
        </AlertTitle>
        <AlertDescription className="flex flex-col items-start gap-2">
          {result.already_there > 0 ? (
            <span>{plural(result.already_there, "row")} already in Balanse, skipped.</span>
          ) : null}
          {result.invalid.length > 0 ? (
            <span>
              {plural(result.invalid.length, "row")} not added. Fix them in your file and import it
              again.
            </span>
          ) : null}
          <Link to="/" className="underline underline-offset-4">
            See your Debtors
          </Link>
        </AlertDescription>
      </Alert>
      {result.invalid.length > 0 ? (
        <Card>
          <CardContent>
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead className="w-20">Row</TableHead>
                  <TableHead>Why it wasn't added</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {result.invalid.map((row) => (
                  <TableRow key={row.row}>
                    <TableCell>{row.row}</TableCell>
                    <TableCell className="whitespace-normal">{row.problems.join(" ")}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      ) : null}
    </section>
  )
}

function plural(count: number, word: string) {
  return `${count} ${word}${count === 1 ? "" : "s"}`
}
