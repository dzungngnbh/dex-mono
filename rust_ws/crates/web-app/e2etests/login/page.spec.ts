import { test } from "@playwright/test"
import { magicToken, sleep, smokeTest } from "../helpers"

// cases
// 1. if user is connected, and user_not_found, then show need to register first, redirect to signup page.
// 2. if user is connected, and already_registered_wait_for_invite, then show already registered message.
// 3. if user is connected, and it redirect to the app

// TODO: How to manipulate the account address
test.beforeEach(async ({ page }) => {
  // await page.goto(`http://localhost:3000/login`)
  // await sleep()
})