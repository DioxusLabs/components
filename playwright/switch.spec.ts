import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=switch&', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  let switchElement = page.getByRole('switch', { name: 'Switch Demo' });
  await expect(switchElement).toBeVisible();
  // The switch should not be checked initially
  await expect(switchElement).toHaveAttribute('data-state', 'unchecked');
  // Clicking the switch should check it
  await switchElement.click();
  await expect(switchElement).toHaveAttribute('data-state', 'checked');
  // Pressing space should also toggle the switch
  await page.keyboard.press('Space');
  await expect(switchElement).toHaveAttribute('data-state', 'unchecked');
});
