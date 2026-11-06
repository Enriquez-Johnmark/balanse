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

const messageStatuses: Record<string, string> = {
  draft: "Waiting for your approval",
  approved: "Approved, waiting for sending hours",
  sending: "Sending",
  sent: "Sent",
  failed: "Not sent yet; trying again",
  blocked: "Blocked by the pre-send check",
  unconfirmed: "Sending wasn't confirmed; not retried",
  expired: "Expired unapproved",
  cancelled: "Discarded",
  by_hand: "Sent by you",
}

/** What the Owner should do next for a Debtor on the dashboard (H1). */
export function nextStep(debtor: { state: string; open_debts: number; drafts_waiting: number }) {
  if (debtor.drafts_waiting > 0) {
    return debtor.drafts_waiting === 1
      ? "Approve the draft"
      : `Approve ${debtor.drafts_waiting} drafts`
  }
  switch (debtor.state) {
    case "paused":
      return "Read their reply"
    case "payment_claimed":
      return "Confirm or reject their payment"
    case "needs_call":
      return "Decide what happens next"
    case "disputed":
      return "Sort out the dispute"
    case "bounced":
      return "Fix their email address"
    case "promised":
      return "Wait for the promised date"
    case "opted_out":
      return "Contact them yourself"
  }
  return debtor.open_debts > 0 ? "Next reminder on schedule" : "Nothing to do"
}

const bounceKinds: Record<string, string> = {
  bounce: "Bounced: the address doesn't take mail",
  soft_bounce: "Bounced for now; it may arrive later",
  complaint: "Marked as spam",
}

/** "soft_bounce" → "Bounced for now; it may arrive later". */
export function bounceKindLabel(kind: string) {
  return bounceKinds[kind] ?? kind
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

/** An outbox status, "expired" → "Expired unapproved". */
export function messageStatusLabel(status: string) {
  return messageStatuses[status] ?? status
}
