import { readFileSync } from "node:fs"

import { test, type Browser, type Page } from "@playwright/test"

import { OUT, SESSION, WORLD, type World } from "./support.ts"

const world = (): World => JSON.parse(readFileSync(WORLD, "utf8")) as World

const desktop = { viewport: { width: 1440, height: 900 }, deviceScaleFactor: 2 }
const phone = { viewport: { width: 390, height: 844 }, deviceScaleFactor: 3, isMobile: true }

async function open(browser: Browser, baseURL: string | undefined, size: typeof desktop) {
  const context = await browser.newContext({ baseURL, storageState: SESSION, ...size })
  return { context, page: await context.newPage() }
}

/** Waits for the page's data, then a moment for the layout to settle. */
async function shot(page: Page, name: string, fullPage = false) {
  await page.waitForLoadState("networkidle")
  await page.waitForTimeout(400)
  await page.screenshot({ path: `${OUT}/${name}.png`, fullPage })
}

test("desktop", async ({ browser }, testInfo) => {
  const { context, page } = await open(browser, testInfo.project.use.baseURL, desktop)
  const { debtors, pages } = world()

  await page.goto("/")
  await page.getByRole("link", { name: "Bautista Media" }).waitFor()
  await shot(page, "desktop-1-dashboard")

  await page.goto("/drafts")
  await page.getByText(/^Select all/).waitFor()
  await shot(page, "desktop-2-drafts")

  await page.getByText(/^Select all/).click()
  await page.getByRole("button", { name: /^Review \d+ selected$/ }).click()
  await page
    .getByRole("dialog")
    .getByText(/^Approve these \d+$/)
    .waitFor()
  await shot(page, "desktop-3-review-before-approving")
  await page.keyboard.press("Escape")
  // Untick them again, and bring the cards into view.
  await page.reload()
  await page.getByText(/^Select all/).waitFor()

  const card = page.locator("[data-slot=card]", { hasText: "Tan Prints" }).first()
  await card.getByRole("button", { name: "Copy and open" }).click()
  await page.getByRole("menuitem", { name: "Copy for SMS" }).waitFor()
  await shot(page, "desktop-4-copy-and-open")
  await page.keyboard.press("Escape")

  await page.goto(`/debtors/${debtors["Bautista Media"]}`)
  await page.getByText(/says they've paid/).waitFor()
  await shot(page, "desktop-5-payment-claim")

  await page.goto(`/debtors/${debtors["Santos Trading Corp"]}`)
  await page.getByRole("button", { name: "Export evidence log" }).waitFor()
  await shot(page, "desktop-6-debtor")

  await page.goto(`/debtors/${debtors["Villanueva Florals"]}`)
  await page.getByText("Delivery problems").waitFor()
  await shot(page, "desktop-10-bounced-address")

  await page.goto("/debtors/import")
  await page.getByLabel("Spreadsheet").setInputFiles({
    name: "new-clients.csv",
    mimeType: "text/csv",
    buffer: Buffer.from(
      [
        "Debtor,Contact name,Contact email,Invoice number,Invoice date,Amount,Due date",
        'Mendoza Farms,Carla Mendoza,carla@mendozafarms.example.test,RCS-2026-041,2026-10-05,"₱22,000.00",2026-11-04',
        "Ramos Auto Parts,Nico Ramos,nico@ramosauto.example.test,RCS-2026-042,2026-10-07,soon,2026-11-06",
      ].join("\n")
    ),
  })
  await page.getByRole("button", { name: "Import", exact: true }).click()
  await page.getByRole("region", { name: "Import result" }).waitFor()
  await shot(page, "desktop-7-import")

  await page.goto("/settings")
  await page.getByRole("button", { name: "Close account" }).waitFor()
  await shot(page, "desktop-8-settings", true)

  // The Contact's own page: no login, no app.
  const contact = await browser.newContext({ baseURL: testInfo.project.use.baseURL, ...desktop })
  const debtorPage = await contact.newPage()
  await debtorPage.goto(pages["Cruz & Co. Events"])
  await shot(debtorPage, "desktop-9-debtors-page")
  await contact.close()
  await context.close()
})

test("phone", async ({ browser }, testInfo) => {
  const { context, page } = await open(browser, testInfo.project.use.baseURL, phone)
  const { pages } = world()

  await page.goto("/")
  await page.getByText("Bautista Media").first().waitFor()
  await shot(page, "phone-1-dashboard")
  await page.goto("/drafts")
  await page.getByText(/^Select all/).waitFor()
  await shot(page, "phone-2-drafts")
  await context.close()

  const contact = await browser.newContext({ baseURL: testInfo.project.use.baseURL, ...phone })
  const debtorPage = await contact.newPage()
  await debtorPage.goto(pages["Cruz & Co. Events"])
  await shot(debtorPage, "phone-3-debtors-page", true)
  await debtorPage.goto(pages["Tan Prints"])
  await shot(debtorPage, "phone-4-debtors-page-english", true)
  await contact.close()
})
