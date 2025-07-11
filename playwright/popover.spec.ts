import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=popover&');
  const popoverButton = page.getByRole('button', { name: 'Show Popover' });
  await expect(popoverButton).toBeVisible();
  await popoverButton.click();
  // pressing the first input should be focused
  const confirm = page.getByRole('button', { name: 'Confirm' });
  const cancel = page.getByRole('button', { name: 'Cancel' });
  await expect(confirm).toBeFocused();
  // pressing tab again should focus the cancel button
  await page.keyboard.press('Tab');
  await expect(cancel).toBeFocused();
  // pressing tab again should focus the confirm button again
  await page.keyboard.press('Tab');
  await expect(confirm).toBeFocused();
  // pressing enter should close the popover
  await page.keyboard.press('Enter');

  // Open the popover again
  await popoverButton.click();
  // pressing escape should close the popover
  await page.keyboard.press('Escape');
});