import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=toast&');
  // Create a toast
  await page.getByRole('button', { name: 'Info (60s)' }).click();
  // Create another toast
  await page.getByRole('button', { name: 'Info (60s)' }).click();
  const toast_close_buttons = page.getByRole('button', { name: 'close' });
  // Hover and close the first toast
  await toast_close_buttons.first().hover();
  await toast_close_buttons.first().click();
  await expect(toast_close_buttons).toHaveCount(1);

  // Hover and close the second toast
  await toast_close_buttons.first().hover();
  await toast_close_buttons.first().click();
  await expect(toast_close_buttons).toHaveCount(0);
});
