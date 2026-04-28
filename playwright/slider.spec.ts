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

test('drag survives pointercancel (iPad system gesture)', async ({ page }) => {
  // iPad can fire `pointercancel` (without a following `pointerup`) when an OS
  // gesture interrupts a drag. Regression: the slider didn't listen for
  // `pointercancel`, so its internal "active pointer" state stayed set and
  // every subsequent tap was ignored.
  await page.goto('http://127.0.0.1:8080/component/?name=slider&', { timeout: 20 * 60 * 1000 });
  const slider = page.locator('.dx-slider').first();
  const thumb = page.locator('.dx-slider-thumb').first();

  await expect(thumb).toHaveAttribute('aria-valuenow', '50');

  const tap = async (frac: number, pointerId: number) => {
    // Re-measure each time — focusing the thumb can scroll the page, which
    // shifts the slider's viewport coordinates between taps.
    const box = await slider.boundingBox();
    if (!box) throw new Error('slider has no bounding box');
    const x = box.x + box.width * frac;
    const y = box.y + box.height / 2;
    await slider.evaluate((el, { x, y, pointerId }) => {
      el.dispatchEvent(new PointerEvent('pointerdown', {
        pointerId,
        pointerType: 'touch',
        isPrimary: true,
        clientX: x,
        clientY: y,
        button: 0,
        buttons: 1,
        bubbles: true,
        cancelable: true,
      }));
    }, { x, y, pointerId });
  };

  // First tap at 30% sets the value normally.
  await tap(0.3, 1);
  await expect(thumb).toHaveAttribute('aria-valuenow', '30');

  // OS gesture: pointercancel fires *without* a matching pointerup. iPad
  // dispatches it on the captured target, not on window — bubble it from the
  // slider element so we exercise the on-element handler too.
  await slider.evaluate((el) => {
    el.dispatchEvent(new PointerEvent('pointercancel', {
      pointerId: 1,
      bubbles: true,
    }));
  });

  // New tap at 70% with a fresh pointer — should still update the slider.
  await tap(0.7, 2);
  await expect(thumb).toHaveAttribute('aria-valuenow', '70');
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

