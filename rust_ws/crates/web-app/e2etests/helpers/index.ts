import { expect, Page } from "@playwright/test"

export const magicToken = "thefirstunexpectedtoken"

export async function smokeTest(page: Page) {
  const bodytext = await page.locator("body").innerText()
  expect(bodytext).not.toContain("NaN")
  expect(bodytext).not.toContain("Internal Server Error")
}

export async function sleep(delay: number = 50) {
  return new Promise((resolve) => setTimeout(resolve, delay))
}
