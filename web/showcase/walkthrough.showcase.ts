import { readFileSync } from "node:fs"

import { test } from "@playwright/test"

import {
  addOverlay,
  OUT,
  pointAt,
  press,
  PROOF,
  record,
  say,
  SESSION,
  WORLD,
  type World,
} from "./support.ts"

const size = { width: 1280, height: 720 }

/** About two minutes through an Owner's week, with captions; saved as
 *  docs/showcase/walkthrough.mp4. It approves drafts and confirms a
 *  payment, so it runs after the screenshots. */
test("walkthrough", async ({ browser }, testInfo) => {
  const { debtors, pages } = JSON.parse(readFileSync(WORLD, "utf8")) as World
  const baseURL = testInfo.project.use.baseURL
  const context = await browser.newContext({ baseURL, storageState: SESSION, viewport: size })
  const page = await context.newPage()
  await addOverlay(page)
  const stop = await record(page, testInfo.outputPath("frames"))

  // The dashboard.
  await page.goto("/")
  await page.getByRole("link", { name: "Bautista Media" }).waitFor()
  await say(page, "Balanse follows up on unpaid invoices for you, politely.")
  await say(page, "The Debtors who need you come first, then the oldest overdue.")
  await pointAt(page, page.getByText("Says they paid").first())
  await say(page, "Who says they paid, who disputes, who promised: all at a glance.")

  // Drafts.
  await press(page, page.getByRole("link", { name: "Drafts" }))
  await page.getByText(/^Select all/).waitFor()
  await say(page, "Each weekday morning, Balanse drafts the reminders that are due.")
  const card = page.locator("[data-slot=card]", { hasText: "Cruz & Co. Events" }).first()
  await pointAt(page, card.getByText(/Paalala po/))
  await say(page, "In Taglish or English, always polite, never a threat.")
  await pointAt(page, card.getByText(/\/d\/[0-9a-f]+/))
  await say(page, "Each one links to the Debtor's own private page.")
  await press(page, card.getByRole("button", { name: "Copy and open" }))
  await page.getByRole("menuitem", { name: "Copy for SMS" }).waitFor()
  await say(page, "Prefer Viber, WhatsApp or SMS? Copy it into your own chat.", 4500)
  await page.keyboard.press("Escape")

  await press(page, page.getByText(/^Select all/))
  await press(page, page.getByRole("button", { name: /^Review \d+ selected$/ }))
  const review = page.getByRole("dialog")
  await review.waitFor()
  await say(page, "Nothing is sent until you approve it, exactly as it will go out.")
  await press(page, review.getByRole("button", { name: /^Approve / }))
  await page
    .getByText(/approved/)
    .first()
    .waitFor()
  await say(page, "Approved. Each email goes out once, on a weekday between 8 a.m. and 8 p.m.")

  // The Debtor's side.
  await page.goto(pages["Cruz & Co. Events"])
  await page.getByRole("button", { name: "Nagbayad na po ako" }).waitFor()
  await say(page, "Your client opens their page on their phone. No app, no password.")
  await pointAt(page, page.getByText(/Paano magbayad/))
  await say(page, "They see what's owed and how to pay you. Balanse never touches the money.")
  await page.getByLabel(/Proof ng bayad/).setInputFiles(PROOF)
  await say(page, "They tap 'I've paid' with a screenshot, or 'I dispute this', or 'Stop'.", 4500)
  await press(page, page.getByRole("button", { name: "Nagbayad na po ako" }))
  await page.getByText(/kukumpirmahin nila ito/).waitFor()
  await say(page, "Reminders pause at once. Every file is virus-scanned and encrypted.")

  // Back with the Owner.
  await page.goto(`/debtors/${debtors["Cruz & Co. Events"]}`)
  await page.getByText(/says they've paid/).waitFor()
  await say(page, "You check your account, then confirm. The invoice is marked paid.")
  await press(page, page.getByRole("button", { name: "Confirm payment" }))
  await press(page, page.getByRole("button", { name: "Confirm payment" }).last())
  await page.getByRole("cell", { name: "Paid" }).waitFor()
  await pointAt(page, page.getByRole("button", { name: "Export evidence log" }))
  await say(page, "Every message and answer is kept in an evidence log you can export.")

  await page.goto("/debtors/import")
  await page.getByRole("heading", { name: "Import a spreadsheet" }).waitFor()
  await say(page, "Start with a spreadsheet of who owes you. Bad rows are listed with why.")

  await press(page, page.getByRole("link", { name: "Settings" }))
  await page.getByRole("heading", { name: "Settings" }).waitFor()
  await say(page, "Your payment instructions, your data to download, and closing your account.")
  await page.goto("/")
  await page.getByRole("link", { name: "Bautista Media" }).waitFor()
  await say(page, "No totals, no ledgers, no invoices: Balanse only follows up.")
  await say(page, "Balanse: polite reminders, approved by you.", 4500)

  await stop(`${OUT}/walkthrough.mp4`)
  await context.close()
})
