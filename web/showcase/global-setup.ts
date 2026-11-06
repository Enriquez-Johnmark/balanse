import { existsSync, mkdirSync, writeFileSync } from "node:fs"

import { chromium, type FullConfig } from "@playwright/test"

import { balanse, loginLink, nextMonday } from "../e2e/support.ts"
import { BUSINESS_NAME, OWNER_EMAIL, PROOF, SESSION, WORLD, type World } from "./support.ts"

/** Debtors and their unpaid invoices, imported the way an Owner would.
 *  Every name and address is made up. */
const DEBTS = [
  "Debtor,Contact name,Contact email,Contact mobile,Language,Invoice number,Invoice date,Amount,Due date,Note",
  'Santos Trading Corp,Lito Santos,lito@santostrading.example.test,0917 555 0101,taglish,RCS-2026-031,2026-09-01,"₱45,000.00",2026-09-30,Brand refresh, phase 1',
  'Santos Trading Corp,Lito Santos,lito@santostrading.example.test,0917 555 0101,taglish,RCS-2026-038,2026-09-20,"₱18,500.00",2026-10-20,Social media kit',
  'Cruz & Co. Events,Mara Cruz,mara@cruzevents.example.test,0918 555 0102,taglish,RCS-2026-034,2026-09-10,"₱32,000.00",2026-10-10,Event posters',
  'Bautista Media,Jun Bautista,jun@bautistamedia.example.test,,english,RCS-2026-035,2026-09-12,"₱27,500.00",2026-10-12,Video editing',
  'Lim Logistics,Grace Lim,grace@limlogistics.example.test,,english,RCS-2026-036,2026-09-15,"₱60,000.00",2026-10-15,Website redesign',
  'Garcia Dental Clinic,Rina Garcia,rina@garciadental.example.test,0917 555 0105,taglish,RCS-2026-037,2026-09-18,"₱12,000.00",2026-10-18,Clinic signage',
  'Tan Prints,Ben Tan,ben@tanprints.example.test,0919 555 0106,english,RCS-2026-039,2026-09-25,"₱9,800.00",2026-10-25,Packaging labels',
  'Ocampo Bakeshop,Joy Ocampo,joy@ocampobakeshop.example.test,0917 555 0107,taglish,RCS-2026-040,2026-10-01,"₱15,000.00",2026-10-31,Menu design',
].join("\n")

const INSTRUCTIONS =
  "GCash 0917 555 0100 (Ana Reyes)\nBDO savings 0012 3456 7890, Reyes Creative Studio\nPlease send a screenshot after paying. Salamat po!"

/** A new demo Business, set up through Balanse itself: requested, approved
 *  by the operator, Debtors imported, reminders drafted, and the Contacts'
 *  own answers on their pages. Runs once per address. */
export default async function globalSetup(config: FullConfig) {
  if (existsSync(SESSION) && existsSync(WORLD)) return
  const baseURL = config.projects[0].use.baseURL ?? "http://127.0.0.1:3000"
  const requested = await fetch(`${baseURL}/api/signup`, {
    method: "POST",
    headers: { "Content-Type": "application/json", Origin: baseURL },
    body: JSON.stringify({
      business_name: BUSINESS_NAME,
      email: OWNER_EMAIL,
      accepted_terms: true,
      lends_money: false,
    }),
  })
  if (requested.status !== 202) throw new Error(`sign-up answered ${requested.status}`)
  const id = balanse("signups")
    .split("\n")
    .find((line) => line.includes(OWNER_EMAIL))
    ?.split(/\s+/)[0]
  if (!id) throw new Error(`no sign-up request for ${OWNER_EMAIL}; use a new SHOWCASE_EMAIL`)
  balanse("approve-signup", id)
  balanse("outbox-drain")

  mkdirSync("showcase/.auth", { recursive: true })
  const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_PATH })
  const page = await browser.newPage({ baseURL })
  await page.goto(await loginLink(OWNER_EMAIL))
  await page.getByRole("button", { name: "Log in" }).click()
  await page.getByRole("heading", { name: "Debtors" }).waitFor()
  await page.context().storageState({ path: SESSION })
  const api = page.request
  const headers = { Origin: baseURL }

  await ok(
    api.put("/api/settings/payment-instructions", {
      headers,
      data: { payment_instructions: INSTRUCTIONS },
    })
  )
  await ok(
    api.post("/api/import", {
      headers,
      multipart: {
        file: { name: "unpaid-invoices.csv", mimeType: "text/csv", buffer: Buffer.from(DEBTS) },
      },
    })
  )
  balanse("dev-schedule", "--date", nextMonday())

  const world: World = { debtors: {}, pages: {} }
  const rows = (await (await ok(api.get("/api/debtors"))).json()) as {
    id: string
    business_name: string
  }[]
  for (const row of rows) world.debtors[row.business_name] = row.id
  const drafts = (await (await ok(api.get("/api/drafts"))).json()) as {
    id: string
    hash: string
    debtor_name: string
    body: string
  }[]
  for (const draft of drafts) {
    const link = draft.body.match(/https?:\/\/\S+?(\/d\/[0-9a-f]+)/)?.[1]
    if (link) world.pages[draft.debtor_name] = link
  }

  // The Contacts answer on their own pages.
  await ok(
    api.post(`${world.pages["Bautista Media"]}/paid`, {
      headers,
      multipart: { file: PROOF },
    })
  )
  await ok(api.post(`${world.pages["Lim Logistics"]}/dispute`, { headers, form: {} }))
  // The Owner's own decisions and records.
  const inTwoWeeks = new Date(Date.now() + 14 * 86_400_000).toISOString().slice(0, 10)
  await ok(
    api.post(`/api/debtors/${world.debtors["Garcia Dental Clinic"]}/outcome`, {
      headers,
      data: { outcome: "promised", until: inTwoWeeks },
    })
  )
  const today = new Intl.DateTimeFormat("en-CA", { timeZone: "Asia/Manila" }).format(new Date())
  await ok(
    api.post(`/api/debtors/${world.debtors["Ocampo Bakeshop"]}/sent-by-hand`, {
      headers,
      data: { sent_on: today },
    })
  )
  const santos = drafts.find((draft) => draft.debtor_name === "Santos Trading Corp")
  if (santos) {
    await ok(
      api.post("/api/drafts/approve", {
        headers,
        data: { drafts: [{ id: santos.id, hash: santos.hash }] },
      })
    )
  }
  writeFileSync(WORLD, JSON.stringify(world, null, 2))
  await browser.close()
}

async function ok<T extends { ok(): boolean; status(): number; url(): string }>(
  answer: Promise<T>
): Promise<T> {
  const response = await answer
  if (!response.ok()) throw new Error(`${response.url()} answered ${response.status()}`)
  return response
}
