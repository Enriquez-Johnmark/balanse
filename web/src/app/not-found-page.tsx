import { Link } from "react-router"

import { Button } from "@/components/ui/button"
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from "@/components/ui/empty"

export function NotFoundPage() {
  return (
    <Empty>
      <EmptyHeader>
        <EmptyTitle>Page not found</EmptyTitle>
        <EmptyDescription>
          This page doesn't exist, or it belongs to another business.
        </EmptyDescription>
      </EmptyHeader>
      <EmptyContent>
        <Button render={<Link to="/" />} nativeButton={false}>
          Go to your Debtors
        </Button>
      </EmptyContent>
    </Empty>
  )
}
