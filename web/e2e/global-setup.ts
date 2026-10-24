import { mkdirSync, writeFileSync } from "node:fs"

import { chromium, type FullConfig } from "@playwright/test"

import { balanse, loginLink } from "./support.ts"

/** A new Business for this run: requested, approved by the operator and
 *  logged in once through its welcome email (A1, A3). Every test starts
 *  from that session. */
export default async function globalSetup(config: FullConfig) {
  const baseURL = config.projects[0].use.baseURL ?? "http://127.0.0.1:3000"
  const email = `e2e-${Date.now()}@example.test`
  const requested = await fetch(`${baseURL}/api/signup`, {
    method: "POST",
    headers: { "Content-Type": "application/json", Origin: baseURL },
    body: JSON.stringify({
      business_name: "E2E Creative",
      email,
      accepted_terms: true,
      lends_money: false,
    }),
  })
  if (requested.status !== 202) throw new Error(`sign-up answered ${requested.status}`)
  const id = balanse("signups")
    .split("\n")
    .find((line) => line.includes(email))
    ?.split(/\s+/)[0]
  if (!id) throw new Error(`no sign-up request for ${email}`)
  balanse("approve-signup", id)
  balanse("outbox-drain")
  mkdirSync("e2e/.auth", { recursive: true })
  writeFileSync("e2e/.auth/business.json", JSON.stringify({ email }))

  const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_PATH })
  const page = await browser.newPage({ baseURL })
  await page.goto(await loginLink(email))
  await page.getByRole("button", { name: "Log in" }).click()
  await page.getByRole("heading", { name: "Debtors" }).waitFor()
  await page.context().storageState({ path: "e2e/.auth/state.json" })
  await browser.close()
}
