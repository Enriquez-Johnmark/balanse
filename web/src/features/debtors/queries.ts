import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

import { api } from "@/api/client"
import type { Created } from "@/api/generated/Created"
import type { DebtorPage } from "@/api/generated/DebtorPage"
import type { DebtorRow } from "@/api/generated/DebtorRow"
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
