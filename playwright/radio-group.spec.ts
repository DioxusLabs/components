import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=radio_group&", { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  await page.getByRole('radio', { name: 'Blue' }).click();
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('radio', { name: 'Red' })).toBeFocused();
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('radio', { name: 'Blue' })).toBeFocused();
});
