import { test } from "@playwright/test"
import { magicToken, sleep, smokeTest } from "../helpers"

let authenticatedPages = [
  `http://localhost:3000/dashboard/?magicToken=${magicToken}`,
  `http://localhost:3000/dashboard/strategy/d9c67f81-1b9a-4898-86c6-1d0bf399b131?magicToken=${magicToken}`,
  `http://localhost:3000/dashboard/editor/d9c67f81-1b9a-4898-86c6-1d0bf399b131?magicToken=${magicToken}`,
]

// loop through and smokeTest
test("Smoke test", async ({ page }) => {
  for (const url of authenticatedPages) {
    await page.goto(url)
    await sleep()
    await smokeTest(page)
  }
})
