import { test, expect } from '@playwright/test';

test('hover navigation', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=navbar&');
  await page.getByRole('menuitem', { name: 'Inputs' }).hover();
  await page.getByRole('menuitem', { name: 'Calendar' }).click();
  // Assert the url changed to the calendar component
  await expect(page).toHaveURL(/.*name=calendar/);
});

test('mobile navigation', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=navbar&');
  await page.getByRole('menuitem', { name: 'Inputs' }).click();
  await page.getByRole('menuitem', { name: 'Calendar' }).click();
  // Assert the url changed to the calendar component
  await expect(page).toHaveURL(/.*name=calendar/);
});

test('keyboard navigation', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=navbar&');
  await page.getByRole('menuitem', { name: 'Inputs' }).focus();
  // Go right with the keyboard
  await page.keyboard.press('ArrowRight');
  // Assert the focus is on the information menu item
  await expect(page.getByRole('menuitem', { name: 'Information' })).toBeFocused();
  // Go left with the keyboard
  await page.keyboard.press('ArrowLeft');
  // Assert the focus is on the inputs menu item
  await expect(page.getByRole('menuitem', { name: 'Inputs' })).toBeFocused();
  await page.keyboard.press('ArrowDown');
  // Assert the focus is on the calendar menu item
  await expect(page.getByRole('menuitem', { name: 'Calendar' })).toBeFocused();
  // Click the focused menu item
  await page.keyboard.press('Enter');
  // Assert the url changed to the calendar component
  await expect(page).toHaveURL(/.*name=calendar/);
});