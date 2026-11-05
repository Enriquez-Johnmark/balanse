import { describe, expect, it } from "vitest"

import {
  debtorStateLabel,
  debtStatusLabel,
  messageStatusLabel,
  nextStep,
  stepLabel,
} from "@/lib/labels"

describe("labels", () => {
  it("shows the Owner's words for each code", () => {
    expect(debtorStateLabel("needs_call")).toBe("Needs your call")
    expect(debtStatusLabel("payment_claimed")).toBe("Says paid")
    expect(stepLabel("heads_up")).toBe("Heads-up")
    expect(messageStatusLabel("expired")).toBe("Expired unapproved")
  })

  it("shows an unknown code as it is", () => {
    expect(debtorStateLabel("something_new")).toBe("something_new")
  })

  it("puts waiting drafts before the Debtor's state as the next step", () => {
    expect(nextStep({ state: "paused", open_debts: 1, drafts_waiting: 2 })).toBe("Approve 2 drafts")
    expect(nextStep({ state: "paused", open_debts: 1, drafts_waiting: 0 })).toBe("Read their reply")
    expect(nextStep({ state: "active", open_debts: 0, drafts_waiting: 0 })).toBe("Nothing to do")
  })
})
