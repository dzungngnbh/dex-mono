import { test } from "@playwright/test"

test("screenshots", async ({ page }) => {
  await page.goto("http://localhost:3000/dashboard/")

  await page.setViewportSize({
    width: 3456,
    height: 2234,
  })

  // sleep
  await page.waitForTimeout(5000)

  await page.screenshot({ path: "test-results/dashboard.png", fullPage: true })
})
