import { test, expect } from '@playwright/test';

test('pointer navigation', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=context_menu&', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  await page.getByRole('button', { name: 'right click here' }).click({
    button: 'right'
  });

  // Assert the context menu is visible
  const contextMenu = page.locator('.context-menu-content');
  await expect(contextMenu).toHaveAttribute('data-state', 'open');
  // Click on the "Edit" menu item
  await page.getByRole('menuitem', { name: 'Edit' }).click();
  // Assert the context menu is closed after clicking
  await expect(contextMenu).toHaveCount(0);
});

test('keyboard navigation', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=context_menu&', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  await page.getByRole('button', { name: 'right click here' }).click({
    button: 'right'
  });

  // Assert the context menu is visible
  const contextMenu = page.locator('.context-menu-content');
  await expect(contextMenu).toHaveAttribute('data-state', 'open');
  // Hit escape to close the context menu
  await page.keyboard.press('Escape');
  // Assert the context menu is closed after pressing escape
  await expect(contextMenu).toHaveCount(0);

  // Reopen the context menu
  await page.getByRole('button', { name: 'right click here' }).click({
    button: 'right'
  });
  await page.keyboard.press('ArrowDown');
  // Assert the "Edit" menu item is focused
  await expect(page.getByRole('menuitem', { name: 'Edit' })).toBeFocused();
  // Move down to the "Duplicate" menu item
  await page.keyboard.press('ArrowDown');
  await page.keyboard.press('ArrowDown');
  // Assert the "Duplicate" menu item is focused
  await expect(page.getByRole('menuitem', { name: 'Duplicate' })).toBeFocused();
  // Hit Enter to select the "Duplicate" menu item
  await page.keyboard.press('Enter');
  // Assert the context menu is closed after selection
  await expect(contextMenu).toHaveCount(0);
  // Assert the selected item is displayed
  await expect(page.getByText('Selected: Duplicate')).toBeVisible();
});