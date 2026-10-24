import { UserPlusIcon } from "lucide-react"
import { Link } from "react-router"

import type { DebtorRow } from "@/api/generated/DebtorRow"
import { PageHeader } from "@/components/page-header"
import { QueryError } from "@/components/query-error"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from "@/components/ui/empty"
import { Skeleton } from "@/components/ui/skeleton"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { useDebtors } from "@/features/debtors/queries"
import { formatDate } from "@/lib/dates"
import { debtorStateLabel } from "@/lib/labels"

export function DebtorsPage() {
  const debtors = useDebtors()
  return (
    <>
      <PageHeader
        title="Debtors"
        description="Clients who owe you, and where their reminders stand."
      >
        <Button render={<Link to="/debtors/new" />} nativeButton={false}>
          <UserPlusIcon data-icon="inline-start" />
          Add a Debtor
        </Button>
      </PageHeader>
      {debtors.isPending ? <Skeleton className="h-64" /> : null}
      {debtors.isError ? <QueryError error={debtors.error} /> : null}
      {debtors.data ? <DebtorList debtors={debtors.data} /> : null}
    </>
  )
}

/** What each Debtor owes is counted, never summed (H2). */
function DebtorList({ debtors }: { debtors: DebtorRow[] }) {
  if (debtors.length === 0) {
    return (
      <Empty className="border">
        <EmptyHeader>
          <EmptyTitle>No Debtors yet</EmptyTitle>
          <EmptyDescription>
            Add a client who owes you, then the invoices they haven't paid.
          </EmptyDescription>
        </EmptyHeader>
        <EmptyContent>
          <Button render={<Link to="/debtors/new" />} nativeButton={false}>
            Add a Debtor
          </Button>
        </EmptyContent>
      </Empty>
    )
  }
  return (
    <Card>
      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Debtor</TableHead>
              <TableHead className="hidden sm:table-cell">Open invoices</TableHead>
              <TableHead className="hidden md:table-cell">Oldest due</TableHead>
              <TableHead>Reminders</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {debtors.map((debtor) => (
              <TableRow key={debtor.id}>
                <TableCell className="whitespace-normal">
                  <Link to={`/debtors/${debtor.id}`} className="font-medium hover:underline">
                    {debtor.business_name}
                  </Link>
                  <div className="text-xs text-muted-foreground">{debtor.contact_name}</div>
                </TableCell>
                <TableCell className="hidden sm:table-cell">{debtor.open_debts}</TableCell>
                <TableCell className="hidden md:table-cell">
                  {debtor.oldest_due ? formatDate(debtor.oldest_due) : "—"}
                </TableCell>
                <TableCell className="whitespace-normal">
                  <Badge variant={debtor.state === "active" ? "secondary" : "outline"}>
                    {debtorStateLabel(debtor.state)}
                  </Badge>
                  {debtor.promised_until ? (
                    <div className="text-xs text-muted-foreground">
                      until {formatDate(debtor.promised_until)}
                    </div>
                  ) : null}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  )
}
