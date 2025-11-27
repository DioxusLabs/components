import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=textarea&", {
    timeout: 20 * 60 * 1000,
  }); // Increase timeout to 20 minutes

  await page.getByPlaceholder('Enter your description').first().fill('This is my description');
  await expect(page.locator('#textarea-message')).toContainText('Description here: This is my description');
});
