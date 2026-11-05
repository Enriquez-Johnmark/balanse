import { useMutation, useQueryClient } from "@tanstack/react-query"

import { api } from "@/api/client"
import type { ImportResult } from "@/api/generated/ImportResult"
import { debtorKeys } from "@/app/query-keys"

/** B2: the API reads the file, adds what's valid and says why the rest wasn't. */
export function useImport() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (file: File) => {
      const form = new FormData()
      form.append("file", file)
      return api<ImportResult>("POST", "/api/import", form)
    },
    onSuccess: () => queryClient.invalidateQueries({ queryKey: debtorKeys.all() }),
  })
}
