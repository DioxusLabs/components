import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=input&", {
    timeout: 20 * 60 * 1000,
  }); // Increase timeout to 20 minutes

  await page.getByRole('textbox', { name: 'Enter your name' }).fill('name');
  await expect(page.locator('#input-greeting')).toContainText('Hello, name!');
});
