import { test } from "@playwright/test"
import { magicToken, sleep, smokeTest } from "../../helpers"

test.beforeEach(async ({ page }) => {
  await page.goto(
    `http://localhost:3000/dashboard/strategy/d9c67f81-1b9a-4898-86c6-1d0bf399b131?magicToken=${magicToken}`,
  )
  await sleep()
})

test("WIP", async ({ page }) => {
})
