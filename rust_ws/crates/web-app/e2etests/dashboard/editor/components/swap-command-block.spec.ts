import { expect, test } from "@playwright/test"
import { magicToken, sleep } from "../../../helpers"

test.beforeEach(async ({ page }) => {
  await page.goto(
    `http://localhost:3000/dashboard/editor/d9c67f81-1b9a-4898-86c6-1d0bf399b131?magicToken=${magicToken}`,
  )
})

test("SwapCommandBlock", async ({ page }) => {
  await page.waitForSelector("[data-test='command-block']")
  let commandBlockDetails = await page.locator("[data-test='command-block']")
    .all()
  await commandBlockDetails[1].click()

  await sleep(10 * 1000)
})
