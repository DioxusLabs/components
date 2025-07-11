import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=toggle&', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  let toggleElement = page.getByRole('button', { name: 'B', exact: true });
  await expect(toggleElement).toBeVisible();
  // The toggle should not be checked initially
  await expect(toggleElement).toHaveAttribute('data-state', 'off');
  // // Clicking the toggle should check it
  await toggleElement.click();
  await expect(toggleElement).toHaveAttribute('data-state', 'on');
  // // Pressing space should also toggle the toggle
  await toggleElement.press('Space');
  await expect(toggleElement).toHaveAttribute('data-state', 'off');
});
