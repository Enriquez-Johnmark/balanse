import { DownloadIcon } from "lucide-react"
import { useParams } from "react-router"

import { ApiError } from "@/api/client"
import type { BounceRow } from "@/api/generated/BounceRow"
import type { DebtorPage as DebtorDetail } from "@/api/generated/DebtorPage"
import type { DebtRow } from "@/api/generated/DebtRow"
import type { ReminderRow } from "@/api/generated/ReminderRow"
import type { ReplyRow } from "@/api/generated/ReplyRow"
import { NotFoundPage } from "@/app/not-found-page"
import { PageHeader } from "@/components/page-header"
import { QueryError } from "@/components/query-error"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardAction, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { CopyMenu } from "@/features/copy/copy-menu"
import { SentByHandDialog } from "@/features/copy/sent-by-hand-dialog"
import { AddDebtDialog } from "@/features/debtors/add-debt-dialog"
import { ChangeAmountDialog } from "@/features/debtors/change-amount-dialog"
import { ClaimCard } from "@/features/debtors/claim-card"
import { EditDebtorDialog } from "@/features/debtors/edit-debtor-dialog"
import { OutcomeCard } from "@/features/debtors/outcome-card"
import { useDebtor } from "@/features/debtors/queries"
import { formatDate, formatDateTime } from "@/lib/dates"
import { bounceKindLabel, debtStatusLabel, messageStatusLabel, stepLabel } from "@/lib/labels"

/** One Debtor: its Contact, its Debts, its reminders and its replies. */
export function DebtorPage() {
  const { id = "" } = useParams()
  const debtor = useDebtor(id)
  if (debtor.error instanceof ApiError && debtor.error.status === 404) return <NotFoundPage />
  return (
    <>
      {debtor.isPending ? <Skeleton className="h-96" /> : null}
      {debtor.isError ? <QueryError error={debtor.error} /> : null}
      {debtor.data ? <DebtorContent debtor={debtor.data} /> : null}
    </>
  )
}

function DebtorContent({ debtor }: { debtor: DebtorDetail }) {
  const contact = [
    debtor.contact_name,
    debtor.contact_email,
    debtor.contact_mobile,
    debtor.language === "taglish" ? "Taglish" : "English",
  ].filter(Boolean)
  return (
    <>
      <PageHeader title={debtor.business_name} description={contact.join(" · ")}>
        <EditDebtorDialog debtor={debtor} />
        {/* H3: everything sent and seen, as a CSV file to keep. */}
        <Button
          variant="outline"
          render={
            <a href={`/api/debtors/${encodeURIComponent(debtor.id)}/evidence-log`} download />
          }
          nativeButton={false}
        >
          <DownloadIcon data-icon="inline-start" />
          Export evidence log
        </Button>
      </PageHeader>
      {debtor.bounces.length > 0 ? <BouncesCard bounces={debtor.bounces} /> : null}
      {debtor.claim ? <ClaimCard claim={debtor.claim} debtorName={debtor.business_name} /> : null}
      <OutcomeCard debtor={debtor} />
      <DebtsCard debtorId={debtor.id} debts={debtor.debts} />
      <div className="grid gap-6 lg:grid-cols-2">
        <RemindersCard debtorId={debtor.id} reminders={debtor.reminders} />
        <RepliesCard replies={debtor.replies} />
      </div>
    </>
  )
}

/** Each amount exactly as typed; never a total (H2). */
function DebtsCard({ debtorId, debts }: { debtorId: string; debts: DebtRow[] }) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Invoices owed</CardTitle>
        <CardAction>
          <AddDebtDialog debtorId={debtorId} />
        </CardAction>
      </CardHeader>
      <CardContent>
        {debts.length === 0 ? (
          <p className="text-sm text-muted-foreground">No invoices yet.</p>
        ) : (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Invoice</TableHead>
                <TableHead>Amount</TableHead>
                <TableHead className="hidden sm:table-cell">Due</TableHead>
                <TableHead className="hidden sm:table-cell">Status</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {debts.map((debt) => (
                <TableRow key={debt.id}>
                  <TableCell className="whitespace-normal">
                    <span className="font-medium whitespace-nowrap">{debt.invoice_number}</span>
                    <div className="text-xs text-muted-foreground">
                      {formatDate(debt.invoice_date)}
                    </div>
                    {debt.note ? (
                      <div className="text-xs text-muted-foreground">{debt.note}</div>
                    ) : null}
                    {/* On a phone the status sits under the invoice, so the row fits. */}
                    <div className="mt-1 sm:hidden">
                      <Badge variant={debt.status === "open" ? "secondary" : "outline"}>
                        {debtStatusLabel(debt.status)}
                      </Badge>
                    </div>
                  </TableCell>
                  <TableCell>
                    {debt.amount}
                    {debt.status === "open" ? <ChangeAmountDialog debt={debt} /> : null}
                  </TableCell>
                  <TableCell className="hidden sm:table-cell">
                    {formatDate(debt.due_date)}
                  </TableCell>
                  <TableCell className="hidden sm:table-cell">
                    <Badge variant={debt.status === "open" ? "secondary" : "outline"}>
                      {debtStatusLabel(debt.status)}
                    </Badge>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        )}
      </CardContent>
    </Card>
  )
}

/** Drafts and approved reminders can be copied (H4); the API checks them
 *  as it would before sending. */
const copyable = new Set(["draft", "approved"])

function RemindersCard({ debtorId, reminders }: { debtorId: string; reminders: ReminderRow[] }) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Reminders</CardTitle>
        <CardAction>
          <SentByHandDialog debtorId={debtorId} />
        </CardAction>
      </CardHeader>
      <CardContent>
        {reminders.length === 0 ? (
          <p className="text-sm text-muted-foreground">No reminders drafted yet.</p>
        ) : (
          <ul className="flex flex-col gap-3 text-sm">
            {reminders.map((reminder) => (
              <li key={reminder.id} className="flex flex-wrap items-center justify-between gap-2">
                <div className="flex flex-col">
                  <span className="font-medium">{stepLabel(reminder.step)}</span>
                  <span className="text-muted-foreground">
                    {reminder.sent_at
                      ? `Sent ${formatDateTime(reminder.sent_at)}`
                      : `${messageStatusLabel(reminder.status)} · drafted ${formatDateTime(reminder.drafted_at)}`}
                  </span>
                </div>
                {copyable.has(reminder.status) ? <CopyMenu messageId={reminder.id} /> : null}
              </li>
            ))}
          </ul>
        )}
      </CardContent>
    </Card>
  )
}

/** F4: bounces and complaints; the provider's reason is shown as text. */
function BouncesCard({ bounces }: { bounces: BounceRow[] }) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Delivery problems</CardTitle>
      </CardHeader>
      <CardContent>
        <ul className="flex flex-col gap-3 text-sm">
          {bounces.map((bounce) => (
            <li key={bounce.at + bounce.kind} className="flex flex-col">
              <span className="font-medium">{bounceKindLabel(bounce.kind)}</span>
              <span className="text-muted-foreground">
                {bounce.address} · {formatDateTime(bounce.at)}
                {bounce.reason ? ` · ${bounce.reason}` : ""}
              </span>
            </li>
          ))}
        </ul>
      </CardContent>
    </Card>
  )
}

/** Replies are untrusted text, shown only as text. */
function RepliesCard({ replies }: { replies: ReplyRow[] }) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Replies</CardTitle>
      </CardHeader>
      <CardContent>
        {replies.length === 0 ? (
          <p className="text-sm text-muted-foreground">No replies yet.</p>
        ) : (
          <ul className="flex flex-col gap-4 text-sm">
            {replies.map((reply) => (
              <li key={reply.received_at + reply.from_address} className="flex flex-col gap-1">
                <span className="font-medium">{reply.subject}</span>
                <span className="text-xs text-muted-foreground">
                  {reply.from_address} · {formatDateTime(reply.received_at)}
                  {reply.automatic ? " · automatic reply" : ""}
                </span>
                {reply.text ? <p className="whitespace-pre-wrap">{reply.text}</p> : null}
              </li>
            ))}
          </ul>
        )}
      </CardContent>
    </Card>
  )
}
