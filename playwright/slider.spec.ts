import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=slider&', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  const slider = await page.locator('.dx-slider').first();
  const thumb = await page.locator('.dx-slider-thumb').first();
  // The initial aria-valuenow should be 50
  await expect(thumb).toHaveAttribute('aria-valuenow', '50');
  await thumb.focus();
  // The aria-valuenow should be 60 after pressing Shift+ArrowRight
  await page.keyboard.press('Shift+ArrowRight');
  await expect(thumb).toHaveAttribute('aria-valuenow', '60');
  await page.keyboard.press('Shift+ArrowRight');
  // The aria-valuenow should be 70 after pressing Shift+ArrowRight again
  await expect(thumb).toHaveAttribute('aria-valuenow', '70');
  // Pressing Shift+ArrowLeft should decrease the value by 10
  await page.keyboard.press('Shift+ArrowLeft');
  await expect(thumb).toHaveAttribute('aria-valuenow', '60');
  // Pressing ArrowLeft should decrease the value by 1
  await page.keyboard.press('ArrowLeft');
  await expect(thumb).toHaveAttribute('aria-valuenow', '59');
  // Pressing ArrowRight should increase the value by 1
  await page.keyboard.press('ArrowRight');
  await expect(thumb).toHaveAttribute('aria-valuenow', '60');
});

test('dynamic min/max', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/block?name=slider&variant=dynamic_range&', { timeout: 20 * 60 * 1000 });
  const slider = page.locator('.dx-slider');
  const thumb = slider.locator('.dx-slider-thumb');

  // Initial state: percentage mode (0-100)
  await expect(thumb).toHaveAttribute('aria-valuemin', '0');
  await expect(thumb).toHaveAttribute('aria-valuemax', '100');

  // Switch to absolute number mode
  await page.getByRole('switch', { name: 'Percentage' }).click();

  // Should now be absolute mode (0-1000)
  await expect(thumb).toHaveAttribute('aria-valuemin', '0');
  await expect(thumb).toHaveAttribute('aria-valuemax', '1000');

  // Click back to percentage mode
  await page.getByRole('switch', { name: 'Percentage' }).click();

  // Should be back to percentage mode (0-100)
  await expect(thumb).toHaveAttribute('aria-valuemin', '0');
  await expect(thumb).toHaveAttribute('aria-valuemax', '100');
});

test('range two thumbs', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/block?name=slider&variant=range&', { timeout: 20 * 60 * 1000 });
  const thumbs = page.locator('.dx-slider-thumb');
  await expect(thumbs).toHaveCount(2);
  const t0 = thumbs.nth(0);
  const t1 = thumbs.nth(1);

  // Initial values
  await expect(t0).toHaveAttribute('aria-valuenow', '20');
  await expect(t1).toHaveAttribute('aria-valuenow', '80');

  // Per-thumb ARIA bounds reflect the live neighbor constraint
  await expect(t0).toHaveAttribute('aria-valuemax', '80');
  await expect(t1).toHaveAttribute('aria-valuemin', '20');

  // Keyboard nudges thumb 0
  await t0.focus();
  await page.keyboard.press('ArrowRight');
  await expect(t0).toHaveAttribute('aria-valuenow', '21');

  // Spam right past thumb 1's value — thumb 0 must clamp at 80, not cross
  for (let i = 0; i < 200; i++) await page.keyboard.press('ArrowRight');
  await expect(t0).toHaveAttribute('aria-valuenow', '80');
  await expect(t1).toHaveAttribute('aria-valuenow', '80');
  // After clamping, thumb 1's lower bound moves up to thumb 0's value
  await expect(t1).toHaveAttribute('aria-valuemin', '80');
});

