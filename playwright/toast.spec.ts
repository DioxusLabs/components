import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=toast&');
  await page.getByRole('button', { name: 'Info (60s)' }).click();
  await page.getByRole('button', { name: 'Info (60s)' }).click();
  await page.getByRole('button', { name: 'close' }).first().click();
  await page.getByRole('button', { name: 'close' }).first().click();
  await expect(page.getByRole('button', { name: 'close' })).toHaveCount(0);
});
