import { defineConfig } from "@playwright/test"

import { SESSION } from "./support.ts"

// Screenshots and the walkthrough video of a demo Business, written to
// docs/showcase/. Not tests: run them on purpose (`pnpm showcase`) against a
// running server; see docs/showcase/README.md.
export default defineConfig({
  testDir: ".",
  testMatch: "*.showcase.ts",
  globalSetup: "./global-setup.ts",
  fullyParallel: false,
  workers: 1,
  retries: 0,
  timeout: 5 * 60_000,
  reporter: "list",
  use: {
    baseURL: process.env.E2E_BASE_URL ?? "http://127.0.0.1:3000",
    storageState: SESSION,
    launchOptions: { executablePath: process.env.CHROMIUM_PATH },
  },
})
