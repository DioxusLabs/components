import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=dialog&', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  await page.getByRole('button', { name: 'Show Dialog' }).click();
  // Assert the dialog is open
  const dialog = page.locator('.dialog-backdrop');
  await expect(dialog).toHaveAttribute('data-state', 'open');
  // Assert the close button is focused
  const closeButton = dialog.getByRole('button');
  await expect(closeButton).toBeFocused();
  // Hitting tab should keep focus on the close button
  await page.keyboard.press('Tab');
  await expect(closeButton).toBeFocused();
  // Hitting escape should close the dialog
  await page.keyboard.press('Escape');
  // Assert the dialog can no longer be found
  await expect(dialog).toHaveCount(0);

  // Reopen the dialog
  await page.getByRole('button', { name: 'Show Dialog' }).click();
  // Assert the dialog is open again
  await expect(dialog).toHaveAttribute('data-state', 'open');
  // Click the close button
  await closeButton.click();
  // Assert the dialog is closed after clicking close
  await expect(dialog).toHaveCount(0);
});
