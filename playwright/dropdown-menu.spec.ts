import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=dropdown_menu&');
  let menuElement = page.getByRole('button', { name: 'Open Menu' });
  // The menu should not be open initially
  await expect(menuElement).toHaveAttribute('data-state', 'closed');
  // Clicking the menu should open it
  await menuElement.click();
  await expect(menuElement).toHaveAttribute('data-state', 'open');
  // Pressing down should focus the first item
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('option', { name: 'Edit' })).toBeFocused();
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('option', { name: 'Undo' })).toBeFocused();
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('option', { name: 'Duplicate' })).toBeFocused();
  // The menu should close after selecting an item
  await page.keyboard.press('Enter');
  await expect(menuElement).toHaveAttribute('data-state', 'closed');

  // Reopen the menu
  await menuElement.click();
  await expect(menuElement).toHaveAttribute('data-state', 'open');
  // Pressing Escape should close the menu
  await page.keyboard.press('Escape');
  await expect(menuElement).toHaveAttribute('data-state', 'closed');

  // Reopen the menu
  await menuElement.click();
  await expect(menuElement).toHaveAttribute('data-state', 'open');
  // Pressing Tab should close the menu
  await page.keyboard.press('Tab');
  await expect(menuElement).toHaveAttribute('data-state', 'closed');

  // Reopen the menu
  await menuElement.click();
  await expect(menuElement).toHaveAttribute('data-state', 'open');
  // Clicking outside the menu should close it
  await page.click('body');
  await expect(menuElement).toHaveAttribute('data-state', 'closed');

   // Reopen the menu
  await menuElement.click();
  await expect(menuElement).toHaveAttribute('data-state', 'open');
  // Clicking an item should close the menu
  await page.getByRole('option', { name: 'Edit' }).click();
  await expect(menuElement).toHaveAttribute('data-state', 'closed');
});