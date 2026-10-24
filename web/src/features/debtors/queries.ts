import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

import { api } from "@/api/client"
import type { Created } from "@/api/generated/Created"
import type { DebtorRow } from "@/api/generated/DebtorRow"
import type { NewDebtor } from "@/api/generated/NewDebtor"

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
