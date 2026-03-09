import { test, expect } from "@playwright/test";

test("recycle list virtualizes rows and updates on scroll", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=recycle_list&", { timeout: 20 * 60 * 1000 });

  const cards = page.locator(".recycle-list-card");
  await expect(cards.first()).toBeVisible({ timeout: 30000 });

  const beforeFirstHeading = (await page.locator(".recycle-list-card h3").first().textContent()) ?? "";
  const initialCount = await cards.count();

  expect(initialCount).toBeGreaterThan(0);
  expect(initialCount).toBeLessThan(2000);

  // Retry scrolling + assertion: re-apply scroll on each retry since WASM
  // re-renders may reset scrollTop, especially on slower engines (WebKit).
  await expect(async () => {
    await page.evaluate(() => {
      const container = document.querySelector(".recycle-list-container");
      if (container) {
        container.scrollTop = 6000;
      }
    });
    await page.waitForTimeout(200);
    const afterFirstHeading = (await page.locator(".recycle-list-card h3").first().textContent()) ?? "";
    expect(afterFirstHeading).not.toEqual(beforeFirstHeading);
  }).toPass({ timeout: 15000 });
});
