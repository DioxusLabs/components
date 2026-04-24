import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=collapsible&", { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  // Get the .dx-collapsible-content
  const collapsibleContent = page.locator(".dx-collapsible-content");
  // Click on the .dx-collapsible-trigger
  const firstCollapsibleTrigger = page.locator(".dx-collapsible-trigger");
  await firstCollapsibleTrigger.click();
  // Verify that the first .dx-collapsible-content is expanded (data-open="true")
  await expect(collapsibleContent.first()).toHaveAttribute("data-open", "true");
});
