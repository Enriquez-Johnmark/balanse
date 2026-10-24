import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

import { api } from "@/api/client"
import { debtorKeys, draftKeys } from "@/app/query-keys"
import type { Approval } from "@/api/generated/Approval"
import type { Approved } from "@/api/generated/Approved"
import type { DraftRow } from "@/api/generated/DraftRow"
import type { Edit } from "@/api/generated/Edit"

export function useDrafts() {
  return useQuery({
    queryKey: draftKeys.list(),
    queryFn: () => api<DraftRow[]>("GET", "/api/drafts"),
  })
}

/** Refreshes everything that shows Drafts after one changes. */
function useRefresh() {
  const queryClient = useQueryClient()
  return () =>
    Promise.all([
      queryClient.invalidateQueries({ queryKey: draftKeys.list() }),
      queryClient.invalidateQueries({ queryKey: debtorKeys.all() }),
    ])
}

/** D1–D2: approves the text the Owner saw, each Draft named by its hash.
 *  The API answers for each one; a refused Draft says why. */
export function useApprove() {
  const refresh = useRefresh()
  return useMutation({
    mutationFn: (drafts: DraftRow[]) =>
      api<Approved[]>("POST", "/api/drafts/approve", {
        drafts: drafts.map(({ id, hash }) => ({ id, hash })),
      } satisfies Approval),
    onSettled: refresh,
  })
}

export function useDiscard() {
  const refresh = useRefresh()
  return useMutation({
    mutationFn: (draft: DraftRow) =>
      api<void>("POST", `/api/drafts/${encodeURIComponent(draft.id)}/discard`),
    onSettled: refresh,
  })
}

export function useEditDraft() {
  const refresh = useRefresh()
  return useMutation({
    mutationFn: ({ id, edit }: { id: string; edit: Edit }) =>
      api<void>("PUT", `/api/drafts/${encodeURIComponent(id)}`, edit),
    onSuccess: refresh,
  })
}
