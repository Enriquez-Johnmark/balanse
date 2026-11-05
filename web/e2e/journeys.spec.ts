import { readFileSync } from "node:fs"

import { expect, test, type Page } from "@playwright/test"

import { balanse, nextMonday } from "./support.ts"

// Names unique to this run, so the tests can share one Business.
const run = Date.now().toString(36)

/** Adds a Debtor through the form and an unpaid invoice on its page. */
async function addDebtor(page: Page, name: string, email: string) {
  await page.goto("/debtors/new")
  await page.getByLabel("Client's business name").fill(name)
  await page.getByLabel("Contact's name").fill("Lito")
  await page.getByLabel("Contact's email").fill(email)
  await page.getByRole("button", { name: "Add Debtor" }).click()
  await expect(page.getByRole("heading", { name })).toBeVisible()

  await page.getByRole("button", { name: "Add an invoice" }).click()
  const dialog = page.getByRole("dialog", { name: "Add an invoice" })
  await dialog.getByLabel("Invoice number").fill("INV-7")
  await dialog.getByLabel("Amount owed").fill("₱12,500.00")
  await dialog.getByLabel("Invoice date").fill("2026-01-02")
  await dialog.getByLabel("Due date").fill("2026-01-05")
  await dialog.getByRole("button", { name: "Add invoice" }).click()
  await expect(dialog).toHaveCount(0)
}

test("a1 a logged-in Owner sees their Business and its Debtors", async ({ page }) => {
  const { email } = JSON.parse(readFileSync("e2e/.auth/business.json", "utf8")) as {
    email: string
  }
  await page.goto("/")
  await expect(page.getByRole("heading", { name: "Debtors" })).toBeVisible()
  await expect(page.getByText("E2E Creative")).toBeVisible()
  await expect(page.getByText(email)).toBeVisible()
})

test("b1 a Debtor and its invoice are added, the amount kept as typed", async ({ page }) => {
  await addDebtor(page, `Santos Trading ${run}`, "lito@santos.test")

  await expect(page.getByRole("cell", { name: /₱12,500\.00/ })).toBeVisible()
  await page.goto("/")
  const row = page.getByRole("row", { name: new RegExp(`Santos Trading ${run}`) })
  await expect(row.getByText("Reminders on")).toBeVisible()
})

test("d2 a batch shows every draft in full before any is approved", async ({ page }) => {
  await addDebtor(page, `Cruz Studio ${run}`, "lito@cruz.test")
  balanse("dev-schedule", "--date", nextMonday())

  await page.goto("/drafts")
  const card = page.locator("[data-slot=card]", { hasText: `Cruz Studio ${run}` })
  await card.getByRole("checkbox").click()
  await page.getByRole("button", { name: /Review 1 selected/ }).click()
  const review = page.getByRole("dialog")
  await expect(review.getByText("lito@cruz.test")).toBeVisible()
  await expect(review.getByText(/INV-7/).first()).toBeVisible()
  await expect(review.getByText(/Sent via Balanse/).first()).toBeVisible()
  await review.getByRole("button", { name: "Approve this draft" }).click()

  await expect(page.getByText("1 approved")).toBeVisible()
  await expect(card).toHaveCount(0)
})

test("f3 a promise pauses reminders until its date, and paid settles the invoices", async ({
  page,
}) => {
  await addDebtor(page, `Reyes Foods ${run}`, "lito@reyesfoods.test")

  await page.getByRole("button", { name: "They promised to pay" }).click()
  const dialog = page.getByRole("dialog", { name: "Promised to pay" })
  await dialog.getByLabel("Pay by").fill(nextMonday())
  await dialog.getByRole("button", { name: "Save promise" }).click()
  await expect(page.getByText("Promised to pay").first()).toBeVisible()
  await expect(page.getByText(/Reminders start again after/)).toBeVisible()

  await page.getByRole("button", { name: "They paid" }).click()
  await page.getByRole("button", { name: "Mark as paid" }).click()
  await expect(page.getByRole("cell", { name: "Paid" })).toBeVisible()
})

test.describe("a visitor", () => {
  test.use({ storageState: { cookies: [], origins: [] } })

  test("a3 a business that lends money is refused with the reason, and nothing is kept", async ({
    page,
  }) => {
    const email = `lender-${run}@example.test`
    await page.goto("/login")
    await page.getByRole("link", { name: "Request an account" }).click()
    await page.getByLabel("Business name").fill("Quick Cash Lending")
    await page.getByLabel("Your email").fill(email)
    await page.getByLabel("Yes").check()
    await page.getByRole("checkbox", { name: /I accept the terms/ }).click()
    await page.getByRole("button", { name: "Request an account" }).click()

    await expect(page.getByText(/can't be used to collect loans/)).toBeVisible()
    expect(balanse("signups")).not.toContain(email)
  })
})

test("g2 a Contact says they've paid with proof, and the Owner confirms it", async ({
  page,
  browser,
}) => {
  await addDebtor(page, `Bautista Media ${run}`, "lito@bautista.test")
  balanse("dev-schedule", "--date", nextMonday())
  await page.goto("/drafts")
  const card = page.locator("[data-slot=card]", { hasText: `Bautista Media ${run}` })
  const body = (await card.textContent()) ?? ""
  const link = body.match(/https?:\/\/\S+?\/d\/[0-9a-f]+/)?.[0]
  expect(link, body).toBeTruthy()

  // The Contact, not logged in, on their phone.
  const contact = await browser.newPage({ storageState: { cookies: [], origins: [] } })
  await contact.goto(link!)
  await expect(contact.getByText("INV-7")).toBeVisible()
  // The Debtor's language is Taglish, the form's default.
  await contact.getByLabel(/Proof ng bayad/).setInputFiles({
    name: "receipt.jpg",
    mimeType: "image/jpeg",
    buffer: Buffer.from([0xff, 0xd8, 0xff, 0xe0, 0x10, 0x20]),
  })
  await contact.getByRole("button", { name: "Nagbayad na po ako" }).click()
  await expect(contact.getByText(/kukumpirmahin nila ito/)).toBeVisible()
  await contact.close()

  // The Owner sees the claim with its proof, and confirms it.
  await page.goto("/")
  await page.getByRole("link", { name: `Bautista Media ${run}` }).click()
  await expect(page.getByText(/says they've paid/)).toBeVisible()
  await expect(page.getByRole("link", { name: /Download proof 1/ })).toBeVisible()
  await page.getByRole("button", { name: "Confirm payment" }).click()
  await page.getByRole("button", { name: "Confirm payment" }).last().click()
  await expect(page.getByRole("cell", { name: "Paid" })).toBeVisible()
})

test("b2 a spreadsheet adds its valid rows and lists why the others weren't", async ({ page }) => {
  await page.goto("/")
  await page.getByRole("button", { name: "Import a spreadsheet" }).click()
  const csv = [
    "Debtor,Contact name,Contact email,Contact mobile,Language,Invoice number,Invoice date,Amount,Due date,Note",
    `Lazaro Studio ${run},Mila,mila@lazaro.test,,english,INV-21,2026-10-01,"₱8,000.00",2026-10-31,`,
    `Lazaro Studio ${run},Mila,mila@lazaro.test,,english,INV-22,2026-10-01,lots,2026-10-31,`,
  ].join("\n")
  await page.getByLabel("Spreadsheet").setInputFiles({
    name: "debts.csv",
    mimeType: "text/csv",
    buffer: Buffer.from(csv),
  })
  await page.getByRole("button", { name: "Import", exact: true }).click()
  const result = page.getByRole("region", { name: "Import result" })
  await expect(result.getByText("Added 1 invoice for 1 new Debtor")).toBeVisible()
  await expect(result.getByRole("cell", { name: "3", exact: true })).toBeVisible()
  await expect(result.getByRole("cell", { name: /^Amount/ })).toBeVisible()

  await result.getByRole("link", { name: "See your Debtors" }).click()
  await expect(page.getByRole("link", { name: `Lazaro Studio ${run}` })).toBeVisible()
})

test("h4 a draft is copied for SMS without its link, and a message sent by hand is noted", async ({
  page,
  context,
}) => {
  await context.grantPermissions(["clipboard-read", "clipboard-write"])
  await addDebtor(page, `Tan Prints ${run}`, "lito@tanprints.test")
  balanse("dev-schedule", "--date", nextMonday())

  await page.goto("/drafts")
  const card = page.locator("[data-slot=card]", { hasText: `Tan Prints ${run}` })
  await card.getByRole("button", { name: "Copy and open" }).click()
  await page.getByRole("menuitem", { name: "Copy for SMS" }).click()
  await expect(page.getByText("Copied for SMS")).toBeVisible()
  const sms = await page.evaluate<string>("navigator.clipboard.readText()")
  expect(sms).toContain("INV-7")
  expect(sms).not.toMatch(/https?:|\/d\//)
  // Copying approved nothing.
  await expect(card.getByRole("button", { name: "Approve" })).toBeVisible()

  await page.goto("/")
  await page.getByRole("link", { name: `Tan Prints ${run}` }).click()
  await page.getByRole("button", { name: "I sent one myself" }).click()
  await page.getByRole("dialog").getByRole("button", { name: "Save" }).click()
  await expect(page.getByText("Noted")).toBeVisible()
})

test("h2 the Owner's pages show no totals, aging or accounting documents", async ({ page }) => {
  await addDebtor(page, `Lim Events ${run}`, "lito@limevents.test")
  const never = /\btotal|\bag(e)?ing\b|statement of account|billing statement|ledger/i

  for (const path of [page.url(), "/", "/drafts", "/settings", "/debtors/import"]) {
    await page.goto(path)
    await expect(page.getByRole("heading").first()).toBeVisible()
    await page.waitForLoadState("networkidle")
    expect(await page.locator("body").innerText()).not.toMatch(never)
  }
})
