import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=slider&', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  const slider = await page.locator('.slider');
  const thumb = await page.locator('.slider-thumb');
  // The initial aria-valuenow should be 50
  await expect(thumb).toHaveAttribute('aria-valuenow', '50');
  await thumb.focus();
  // The aria-valuenow should be 60 after pressing Shift+ArrowRight
  await page.keyboard.press('Shift+ArrowRight');
  await expect(thumb).toHaveAttribute('aria-valuenow', '60');
  await page.keyboard.press('Shift+ArrowRight');
  // The aria-valuenow should be 70 after pressing Shift+ArrowRight again
  await expect(thumb).toHaveAttribute('aria-valuenow', '70');
  // Pressing Shift+ArrowLeft should decrease the value by 10
  await page.keyboard.press('Shift+ArrowLeft');
  await expect(thumb).toHaveAttribute('aria-valuenow', '60');
  // Pressing ArrowLeft should decrease the value by 1
  await page.keyboard.press('ArrowLeft');
  await expect(thumb).toHaveAttribute('aria-valuenow', '59');
  // Pressing ArrowRight should increase the value by 1
  await page.keyboard.press('ArrowRight');
  await expect(thumb).toHaveAttribute('aria-valuenow', '60');
});