// Every server-data key in one place, so a change in one feature can
// refresh what another shows.

export const debtorKeys = {
  all: () => ["debtors"] as const,
  list: () => [...debtorKeys.all(), "list"] as const,
  detail: (id: string) => [...debtorKeys.all(), "detail", id] as const,
}

export const draftKeys = {
  list: () => ["drafts"] as const,
}
