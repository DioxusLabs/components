import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=select&', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  // Find Select a fruit...
  let selectTrigger = page.locator(".select-trigger");
  await selectTrigger.click();
  // Assert the select menu is open
  const selectMenu = page.locator('.select-list');
  await expect(selectMenu).toHaveAttribute('data-state', 'open');

  // Assert the menu is focused
  await expect(selectMenu).toBeFocused();
  await page.keyboard.press('ArrowDown');
  const firstOption = selectMenu.getByRole('option', { name: 'apple' });
  await expect(firstOption).toBeFocused();

  // Assert moving down with arrow keys moves focus to the next option
  await page.keyboard.press('ArrowDown');
  const secondOption = selectMenu.getByRole('option', { name: 'banana' });
  await expect(secondOption).toBeFocused();

  // Assert moving up with arrow keys moves focus back to the previous option
  await page.keyboard.press('ArrowUp');
  await expect(firstOption).toBeFocused();

  // Assert pressing Enter selects the focused option
  await page.keyboard.press('Enter');
  // Assert the select menu is closed after selection
  await expect(selectMenu).toHaveCount(0);

  // Assert the selected value is displayed in the button
  await expect(selectTrigger).toHaveText('apple');

  // Reopen the select menu
  await selectTrigger.click();

  // Assert typeahead functionality works
  await page.keyboard.type('Ban');
  // Assert the second option is focused after typing 'Ban'
  await expect(secondOption).toBeFocused();

  // Assert pressing Escape closes the select menu
  await page.keyboard.press('Escape');
  // Assert the select menu is closed
  await expect(selectMenu).toHaveCount(0);

  // Reopen the select menu
  await selectTrigger.click();
  // Assert the select menu is open again
  await expect(selectMenu).toHaveAttribute('data-state', 'open');

  // Click the second option to select it
  let bananaOption = selectMenu.getByRole('option', { name: 'banana' });
  await bananaOption.click();
  // Assert the select menu is closed after clicking an option
  await expect(selectMenu).toHaveCount(0);
  // Assert the selected value is now 'banana'
  await expect(selectTrigger).toHaveText('banana');
});
