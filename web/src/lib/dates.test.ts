import { describe, expect, it } from "vitest"

import { formatDate, formatDateTime, formatDay } from "@/lib/dates"

describe("dates", () => {
  it("shows a calendar date with its weekday, as Balanse's emails do", () => {
    expect(formatDate("2026-11-05")).toBe("Thu 5 Nov 2026")
    expect(formatDate("2027-09-10")).toBe("Fri 10 Sep 2027")
  })

  it("shows a moment as its date and time in Manila", () => {
    // 23:30 UTC on 4 Nov is 07:30 on 5 Nov in Manila.
    expect(formatDay("2026-11-04T23:30:00Z")).toBe("Thu 5 Nov 2026")
    expect(formatDateTime("2026-11-04T23:30:00Z")).toBe("Thu 5 Nov 2026, 07:30")
  })
})
