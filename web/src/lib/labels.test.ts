import { describe, expect, it } from "vitest"

import { debtorStateLabel, debtStatusLabel, messageStatusLabel, stepLabel } from "@/lib/labels"

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
})
