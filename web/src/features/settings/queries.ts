import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

import { api } from "@/api/client"
import type { CloseAccount } from "@/api/generated/CloseAccount"
import type { Closed } from "@/api/generated/Closed"
import type { NewInstructions } from "@/api/generated/NewInstructions"
import type { Settings } from "@/api/generated/Settings"
import { settingsKeys } from "@/app/query-keys"
import { todayInManila } from "@/lib/dates"

export function useSettings() {
  return useQuery({
    queryKey: settingsKeys.all(),
    queryFn: () => api<Settings>("GET", "/api/settings"),
  })
}

/** H5: everything, as one JSON file; the API asks for a recent login. */
export function useDownloadData() {
  return useMutation({
    mutationFn: async () => {
      const data = await api<unknown>("GET", "/api/export")
      const url = URL.createObjectURL(
        new Blob([JSON.stringify(data, null, 2)], { type: "application/json" })
      )
      const link = document.createElement("a")
      link.href = url
      link.download = `balanse-data-${todayInManila()}.json`
      link.click()
      // Some browsers read the file after click() returns.
      setTimeout(() => URL.revokeObjectURL(url), 60_000)
    },
  })
}

/** H5: closes the account; the API erases it after its grace period. */
export function useCloseAccount() {
  return useMutation({
    mutationFn: (input: CloseAccount) => api<Closed>("POST", "/api/account/close", input),
  })
}

/** G6: the API refuses unless the Owner logged in within 10 minutes. */
export function useSaveInstructions() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (input: NewInstructions) =>
      api<void>("PUT", "/api/settings/payment-instructions", input),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: settingsKeys.all() }),
  })
}
