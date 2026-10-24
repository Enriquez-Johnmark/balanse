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
