import { test, expect } from "@playwright/test";

test("recycle list virtualizes rows and updates on scroll", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=recycle_list&", { timeout: 20 * 60 * 1000 });

  const cards = page.locator(".recycle-list-card");
  await expect(cards.first()).toBeVisible({ timeout: 30000 });

  const initialCount = await cards.count();

  expect(initialCount).toBeGreaterThan(0);
  expect(initialCount).toBeLessThan(2000);

  // Retry scrolling + assertion: re-apply scroll on each retry since WASM
  // re-renders may reset scrollTop, especially on slower engines (WebKit).
  await expect(async () => {
    await page.evaluate(() => {
      document.querySelectorAll(".recycle-list-container").forEach((c) => {
        if (c.scrollHeight > c.clientHeight + 1) {
          c.scrollTop = 6000;
        }
      });
      window.scrollTo(0, 6000);
    });
    await page.waitForTimeout(300);
    const headings = await page.locator(".recycle-list-card h3").allTextContents();
    // After scrolling to offset 6000, at least some items with index > 30
    // should be visible, proving the virtual list responded to the scroll.
    const hasScrolledContent = headings.some((h) => {
      const match = h.match(/#(\d+)/);
      return match != null && parseInt(match[1]) > 30;
    });
    expect(hasScrolledContent).toBe(true);
  }).toPass({ timeout: 15000 });
});
