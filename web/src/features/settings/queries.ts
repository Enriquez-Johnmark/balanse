import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

import { api } from "@/api/client"
import type { NewInstructions } from "@/api/generated/NewInstructions"
import type { Settings } from "@/api/generated/Settings"
import { settingsKeys } from "@/app/query-keys"

export function useSettings() {
  return useQuery({
    queryKey: settingsKeys.all(),
    queryFn: () => api<Settings>("GET", "/api/settings"),
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
