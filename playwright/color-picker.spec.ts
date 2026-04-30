import { test, expect, type Page } from '@playwright/test';

const PAGE_URL = 'http://127.0.0.1:8080/component/?name=color_picker&';
const PAGE_TIMEOUT = 20 * 60 * 1000;

async function openPicker(page: Page) {
  await page.goto(PAGE_URL, { timeout: PAGE_TIMEOUT });
  // The color picker button's aria-label contains the current hex.
  const button = page.locator('.dx-color-picker-button').first();
  await expect(button).toBeVisible();
  await button.click();
  const popover = page.locator('.dx-color-picker-popover');
  await expect(popover).toHaveAttribute('data-state', 'open');
  return { button, popover };
}

test('opens popover and shows initial color', async ({ page }) => {
  const { button } = await openPicker(page);

  // Initial color is rgb(155, 128, 255) → #9B80FF
  await expect(button).toHaveAttribute('aria-label', /Color picker #9B80FF/i);

  // The hex field inside the popover mirrors the same hex.
  const hexField = page.locator('#color_field');
  await expect(hexField).toHaveValue('#9B80FF');

  // Both the trigger swatch and the popover swatch render the same color.
  const swatches = page.locator('.dx-color-swatch');
  await expect(swatches.first()).toHaveAttribute(
    'style',
    /--swatch-color:\s*#9B80FF/i,
  );
});

test('typing a new hex updates the color picker', async ({ page }) => {
  const { button } = await openPicker(page);
  const hexField = page.locator('#color_field');

  // Replace value with a fresh, well-known color.
  await hexField.click();
  await hexField.press('ControlOrMeta+A');
  await hexField.pressSequentially('#FF0000');

  await expect(hexField).toHaveValue('#FF0000');
  // The trigger button's aria-label echoes the new color.
  await expect(button).toHaveAttribute('aria-label', /Color picker #FF0000/i);

  // The hue slider snaps to red (0°). Read it from the thumb that lives in
  // the open dialog, not the (non-existent) one outside it.
  const hueThumb = page.locator('.dx-color-slider-thumb').first();
  // Hue can be reported as 0 or 360 depending on which way palette wraps —
  // both are red.
  await expect.poll(async () => {
    const v = await hueThumb.getAttribute('aria-valuenow');
    return Number(v) % 360;
  }).toBe(0);
});

test('hex field strips invalid characters and caps at 7 chars', async ({ page }) => {
  await openPicker(page);
  const hexField = page.locator('#color_field');

  await hexField.click();
  await hexField.press('ControlOrMeta+A');
  // "ZZZ" should be dropped, only hex digits keep, '#' auto-prepends, and the
  // total is truncated to 7 chars (#RRGGBB).
  await hexField.pressSequentially('ZZZff00aabbccdd');

  const value = await hexField.inputValue();
  expect(value.length).toBeLessThanOrEqual(7);
  expect(value).toMatch(/^#[0-9A-F]{0,6}$/);
});

test('hue slider keyboard navigation updates color', async ({ page }) => {
  await openPicker(page);
  // The hue slider lives inside the popover. Use the thumb directly.
  const hueThumb = page.locator('.dx-color-slider-thumb').first();
  await hueThumb.focus();

  const before = Number(await hueThumb.getAttribute('aria-valuenow'));
  await page.keyboard.press('ArrowRight');
  const after = Number(await hueThumb.getAttribute('aria-valuenow'));

  // ArrowRight should move hue by one step; default slider step is 1.
  expect(after).toBeGreaterThan(before);
  expect(after - before).toBeLessThan(5);

  // Shift+ArrowRight applies the 10× multiplier.
  await page.keyboard.press('Shift+ArrowRight');
  const shifted = Number(await hueThumb.getAttribute('aria-valuenow'));
  expect(shifted - after).toBeGreaterThan(5);
});

test('color area thumb keyboard navigation updates saturation/value', async ({ page }) => {
  await openPicker(page);
  const areaThumb = page.locator('.dx-color-area-thumb');
  await expect(areaThumb).toBeVisible();
  await areaThumb.focus();

  // Read the visually-hidden range inputs that mirror saturation (x) and value (y).
  const sInput = page.locator('input[aria-label="Saturation"]');
  const vInput = page.locator('input[aria-label="Value"]');

  const sBefore = Number(await sInput.inputValue());
  const vBefore = Number(await vInput.inputValue());

  // ArrowRight increases saturation, and focus hands off to the saturation
  // input so AT can announce the channel.
  await page.keyboard.press('ArrowRight');
  await expect(sInput).toBeFocused();

  // ArrowDown decreases value, and focus hands off to the value input.
  await page.keyboard.press('ArrowDown');
  await expect(vInput).toBeFocused();

  const sAfter = Number(await sInput.inputValue());
  const vAfter = Number(await vInput.inputValue());

  expect(sAfter).toBeGreaterThan(sBefore);
  expect(vAfter).toBeLessThan(vBefore);
});

test('escape closes the color picker popover', async ({ page }) => {
  const { popover } = await openPicker(page);
  await page.keyboard.press('Escape');
  await expect(popover).toHaveCount(0);
});
