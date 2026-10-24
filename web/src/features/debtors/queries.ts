import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

import { api } from "@/api/client"
import type { Created } from "@/api/generated/Created"
import type { DebtorPage } from "@/api/generated/DebtorPage"
import type { DebtorRow } from "@/api/generated/DebtorRow"
import type { NewAmount } from "@/api/generated/NewAmount"
import type { NewDebt } from "@/api/generated/NewDebt"
import type { NewDebtor } from "@/api/generated/NewDebtor"
import type { Outcome } from "@/api/generated/Outcome"

export const debtorKeys = {
  all: () => ["debtors"] as const,
  list: () => [...debtorKeys.all(), "list"] as const,
  detail: (id: string) => [...debtorKeys.all(), "detail", id] as const,
}

export function useDebtors() {
  return useQuery({
    queryKey: debtorKeys.list(),
    queryFn: () => api<DebtorRow[]>("GET", "/api/debtors"),
  })
}

export function useDebtor(id: string) {
  return useQuery({
    queryKey: debtorKeys.detail(id),
    queryFn: () => api<DebtorPage>("GET", `/api/debtors/${encodeURIComponent(id)}`),
  })
}

/** Refreshes every page that shows Debtors after one changes. */
export function useRefreshDebtors() {
  const queryClient = useQueryClient()
  return () => queryClient.invalidateQueries({ queryKey: debtorKeys.all() })
}

export function useCreateDebtor() {
  const refresh = useRefreshDebtors()
  return useMutation({
    mutationFn: (input: NewDebtor) => api<Created>("POST", "/api/debtors", input),
    onSuccess: refresh,
  })
}

/** F3: the Owner's decision: paid, promised, disputed or resume. */
export function useRecordOutcome(id: string) {
  const refresh = useRefreshDebtors()
  return useMutation({
    mutationFn: (outcome: Outcome) =>
      api<void>("POST", `/api/debtors/${encodeURIComponent(id)}/outcome`, outcome),
    onSuccess: refresh,
  })
}

/** B1: an unpaid invoice of this Debtor. */
export function useAddDebt(debtorId: string) {
  const refresh = useRefreshDebtors()
  return useMutation({
    mutationFn: (input: NewDebt) =>
      api<Created>("POST", `/api/debtors/${encodeURIComponent(debtorId)}/debts`, input),
    onSuccess: refresh,
  })
}

/** B3: what is still owed on a Debt, after a part payment or a correction. */
export function useChangeAmount(debtId: string) {
  const refresh = useRefreshDebtors()
  return useMutation({
    mutationFn: (input: NewAmount) =>
      api<void>("POST", `/api/debts/${encodeURIComponent(debtId)}/amount`, input),
    onSuccess: refresh,
  })
}
