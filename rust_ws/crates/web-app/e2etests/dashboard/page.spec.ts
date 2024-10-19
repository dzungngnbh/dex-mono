import { test } from "@playwright/test"
import { magicToken, sleep, smokeTest } from "../helpers"

test.beforeEach(async ({ page }) => {
  await page.goto(`http://localhost:3000/dashboard/?magicToken=${magicToken}`)
  await sleep()
})

test("WIP", async ({ page }) => {
})
