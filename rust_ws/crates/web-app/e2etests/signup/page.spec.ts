import { test } from "@playwright/test"
import { magicToken, sleep, smokeTest } from "../helpers"

// cases
// 1. if user is connected, and is_disabled is false in the system, then ask them to login
// 2. if user is connected, and is_disabled is true then showing waiting for the invitation.
// 3. if user is connected, but not in the system, then show sign message.

// TODO: How to manipulate the account address
test.beforeEach(async ({ page }) => {
  // await page.goto(`http://localhost:3000/prototypes/signup/index.html`)
  // await sleep()
})