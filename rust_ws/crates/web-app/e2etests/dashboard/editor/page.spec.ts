import { expect, test } from "@playwright/test"
import { magicToken, smokeTest } from "../../helpers"

test.beforeEach(async ({ page }) => {
  await page.goto(
    `http://localhost:3000/dashboard/editor/d9c67f81-1b9a-4898-86c6-1d0bf399b131?magicToken=${magicToken}`,
  )
})

// TODO: make sure we run server with test db to test
test("WIP", async ({ page }) => {
  await page.waitForSelector("[data-test='command-title']")

  // visibility
  await expect(page.getByText("1. tradingview_webhook: Webhook")).toBeVisible()
  await expect(
    page.getByText("2. uniswap_v3_single_hop_swap: Swap ( single hop )"),
  ).toBeVisible()
})
