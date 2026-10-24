import { defineConfig, devices } from "@playwright/test"

// Browser tests of the main journeys, against a running Balanse (the Rust
// server with the built app, Postgres and Mailpit). CI starts them; locally:
// `pnpm build`, `cargo run -- serve`, then `pnpm e2e`.
export default defineConfig({
  testDir: "e2e",
  globalSetup: "./e2e/global-setup.ts",
  fullyParallel: false,
  workers: 1,
  retries: 0,
  reporter: process.env.CI ? "github" : "list",
  use: {
    baseURL: process.env.E2E_BASE_URL ?? "http://127.0.0.1:3000",
    storageState: "e2e/.auth/state.json",
    trace: "retain-on-failure",
  },
  projects: [
    {
      name: "desktop",
      use: {
        ...devices["Desktop Chrome"],
        launchOptions: { executablePath: process.env.CHROMIUM_PATH },
      },
    },
  ],
})
