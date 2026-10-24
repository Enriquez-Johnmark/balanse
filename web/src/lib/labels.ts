// The API sends states as codes; these are the words the Owner sees
// (GLOSSARY.md). An unknown code is shown as it is rather than hidden.

const debtorStates: Record<string, string> = {
  active: "Reminders on",
  paused: "Paused: they replied",
  promised: "Promised to pay",
  needs_call: "Needs your call",
  payment_claimed: "Says they paid",
  disputed: "Disputed",
  opted_out: "Asked to stop",
  bounced: "Email bounced",
}

const debtStatuses: Record<string, string> = {
  open: "Open",
  payment_claimed: "Says paid",
  paid: "Paid",
  disputed: "Disputed",
  written_off: "Written off",
  cancelled: "Cancelled",
}

const steps: Record<string, string> = {
  heads_up: "Heads-up",
  first: "First reminder",
  second: "Second reminder",
  third: "Third reminder",
}

/** "needs_call" → "Needs your call". */
export function debtorStateLabel(state: string) {
  return debtorStates[state] ?? state
}

/** "payment_claimed" → "Says paid". */
export function debtStatusLabel(status: string) {
  return debtStatuses[status] ?? status
}

/** "heads_up" → "Heads-up". */
export function stepLabel(step: string) {
  return steps[step] ?? step
}
