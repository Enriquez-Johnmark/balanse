// Dates come from the API; the app only formats them, always in Manila
// time (docs/frontend.md: dates are shown, not computed).

const dayFormat = new Intl.DateTimeFormat("en-US", {
  weekday: "short",
  day: "numeric",
  month: "short",
  year: "numeric",
  timeZone: "UTC",
})

const timeFormat = new Intl.DateTimeFormat("en-GB", {
  hour: "2-digit",
  minute: "2-digit",
  hour12: false,
  timeZone: "Asia/Manila",
})

const manilaDay = new Intl.DateTimeFormat("en-CA", { timeZone: "Asia/Manila" })

function parts(date: Date) {
  const values = Object.fromEntries(dayFormat.formatToParts(date).map((p) => [p.type, p.value]))
  return `${values.weekday} ${values.day} ${values.month} ${values.year}`
}

/** "2026-11-05" → "Thu 5 Nov 2026". */
export function formatDate(isoDate: string) {
  const [year, month, day] = isoDate.split("-").map(Number)
  return parts(new Date(Date.UTC(year, month - 1, day)))
}

/** An RFC 3339 moment → its Manila date, "Thu 5 Nov 2026". */
export function formatDay(moment: string) {
  return formatDate(manilaDay.format(new Date(moment)))
}

/** An RFC 3339 moment → "Thu 5 Nov 2026, 07:30" in Manila. */
export function formatDateTime(moment: string) {
  return `${formatDay(moment)}, ${timeFormat.format(new Date(moment))}`
}
