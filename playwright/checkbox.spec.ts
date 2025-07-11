import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=checkbox&', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  let checkbox = page.getByRole('checkbox', { name: 'Demo Checkbox' });
  await expect(checkbox).toBeVisible();
  // The checkbox should not be checked initially
  await expect(checkbox).toHaveAttribute('data-state', 'unchecked');
  // Clicking the checkbox should check it
  await checkbox.click();
  await expect(checkbox).toHaveAttribute('data-state', 'checked');
  // Pressing space should also toggle the checkbox
  await page.keyboard.press('Space');
  await expect(checkbox).toHaveAttribute('data-state', 'unchecked');
});
