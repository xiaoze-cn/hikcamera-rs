import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "./tests",
  timeout: 30_000,
  use: {
    baseURL: "http://localhost:4322",
    screenshot: "only-on-failure",
    trace: "on-first-retry",
  },
  webServer: {
    command:
      "pnpm run build && pnpm exec astro preview --host localhost --port 4322",
    url: "http://localhost:4322",
    reuseExistingServer: false,
    timeout: 60_000,
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],
});
