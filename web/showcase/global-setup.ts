import { execFileSync } from "node:child_process"
import { createHmac } from "node:crypto"
import { existsSync, mkdirSync, writeFileSync } from "node:fs"
import { resolve } from "node:path"

import { chromium, type FullConfig } from "@playwright/test"

import { loginLink, nextMonday } from "../e2e/support.ts"
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
  'Villanueva Florals,Rosa Villanueva,rosa@villanuevaflorals.example.test,0917 555 0108,english,RCS-2026-033,2026-09-08,"₱21,000.00",2026-10-08,Shop window lettering',
].join("\n")

const BOUNCING = "Villanueva Florals"
const BOUNCING_ADDRESS = "rosa@villanuevaflorals.example.test"
/** The server's RESEND_WEBHOOK_SECRET; the showcase serves with this one. */
const WEBHOOK_SECRET = process.env.RESEND_WEBHOOK_SECRET ?? "whsec_c2hvd2Nhc2Utb25seQ=="

const INSTRUCTIONS =
  "GCash 0917 555 0100 (Ana Reyes)\nBDO savings 0012 3456 7890, Reyes Creative Studio\nPlease send a screenshot after paying. Salamat po!"

/** A new demo Business, set up through Balanse itself: requested, approved
 *  by the operator, Debtors imported, reminders drafted, and the Contacts'
 *  own answers on their pages. Runs once per address. */
export default async function globalSetup(config: FullConfig) {
  if (existsSync(SESSION) && existsSync(WORLD)) return
  const baseURL = config.projects[0].use.baseURL ?? "http://127.0.0.1:3000"
  const balanse = await operator(baseURL)
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
  // Two reminders approved and sent; one address turns out not to exist,
  // and the provider reports the bounce through the signed delivery hook.
  const approved = drafts.filter((draft) =>
    ["Santos Trading Corp", BOUNCING].includes(draft.debtor_name)
  )
  await ok(
    api.post("/api/drafts/approve", {
      headers,
      data: { drafts: approved.map((draft) => ({ id: draft.id, hash: draft.hash })) },
    })
  )
  await reportBounce(baseURL, await sendApproved(BOUNCING_ADDRESS))
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

/** Waits for the server's sender to send the approved reminders, and
 *  returns the provider's ID for the one to `to`. Balanse sends only on
 *  weekdays, 08:00 to 20:00 Manila time; outside them, serve under
 *  `faketime` (docs/showcase/README.md). */
async function sendApproved(to: string) {
  const id = await mailpitId(to, 150)
  if (id) return id
  throw new Error(
    `the reminder to ${to} wasn't sent: outside 08:00-20:00 Manila on a weekday, serve under faketime`
  )
}

/** Mailpit's ID for the newest email to `to`: the dev mailer's provider ID. */
async function mailpitId(to: string, attempts: number) {
  const mailpit = process.env.MAILPIT_URL ?? "http://localhost:8025"
  for (let attempt = 0; attempt < attempts; attempt++) {
    const search = await fetch(
      `${mailpit}/api/v1/search?query=${encodeURIComponent(`to:${to}`)}&limit=1`
    )
    const id = ((await search.json()) as { messages?: { ID: string }[] }).messages?.[0]?.ID
    if (id) return id
    await new Promise((resolve) => setTimeout(resolve, 200))
  }
  return undefined
}

/** Posts a permanent bounce for `emailId` to the delivery hook, signed the
 *  way Resend (Svix) signs it. */
async function reportBounce(baseURL: string, emailId: string) {
  const body = JSON.stringify({
    type: "email.bounced",
    data: {
      email_id: emailId,
      bounce: { type: "Permanent", message: "550 5.1.1 The email account does not exist" },
    },
  })
  const id = "msg_showcase_bounce"
  // Signed at the server's own time, which may be shifted (see sendApproved).
  const timestamp = Math.floor((await serverTime(baseURL)) / 1000).toString()
  const key = Buffer.from(WEBHOOK_SECRET.replace(/^whsec_/, ""), "base64")
  const signature = createHmac("sha256", key).update(`${id}.${timestamp}.${body}`).digest("base64")
  const answer = await fetch(`${baseURL}/hooks/resend`, {
    method: "POST",
    headers: {
      "content-type": "application/json",
      "svix-id": id,
      "svix-timestamp": timestamp,
      "svix-signature": `v1,${signature}`,
    },
    body,
  })
  if (answer.status !== 204) {
    throw new Error(`the delivery hook answered ${answer.status}; serve with RESEND_WEBHOOK_SECRET`)
  }
}

/** The `balanse` command, run at the server's time: when the server is on a
 *  shifted clock (`faketime`, see sendApproved), so is the command, or the
 *  login links it makes would already have expired. */
async function operator(baseURL: string) {
  const repository = resolve(import.meta.dirname, "../..")
  const binary = process.env.BALANSE_BIN ?? resolve(repository, "target/debug/balanse")
  const hours = Math.round(((await serverTime(baseURL)) - Date.now()) / 3_600_000)
  return (...args: string[]) =>
    hours === 0
      ? execFileSync(binary, args, { cwd: repository, encoding: "utf8" })
      : execFileSync("faketime", ["-f", `${hours > 0 ? "+" : ""}${hours}h`, binary, ...args], {
          cwd: repository,
          encoding: "utf8",
        })
}

/** The server's clock, from its Date header. */
async function serverTime(baseURL: string) {
  const served = (await fetch(baseURL)).headers.get("date")
  return served ? Date.parse(served) : Date.now()
}
