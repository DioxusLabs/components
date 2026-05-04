import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=accordion&", { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  // Get the first .dx-accordion-item
  const accordionItem = page.locator(".dx-accordion-item");
  // Click on the first .dx-accordion-item
  const firstAccordionItem = accordionItem.first();
  await firstAccordionItem.locator("button").click();
  // Verify that the first .dx-accordion-item is expanded (data-open="true")
  await expect(firstAccordionItem).toHaveAttribute("data-open", "true");

  // Click on the second .dx-accordion-item
  const secondAccordionItem = accordionItem.nth(1);
  await secondAccordionItem.locator("button").click();
  // Verify that the second .dx-accordion-item is expanded (data-open="true")
  await expect(secondAccordionItem).toHaveAttribute("data-open", "true");
  // Verify the first .dx-accordion-item is collapsed (data-open="false")
  await expect(firstAccordionItem).toHaveAttribute("data-open", "false");
});

test("keyboard navigation skips disabled items", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=accordion&", { timeout: 20 * 60 * 1000 });
  const accordionItems = page.locator(".dx-accordion-item");
  const buttons = accordionItems.locator("button");

  await expect(accordionItems.nth(2)).toHaveAttribute("data-disabled", "true");
  await expect(buttons.nth(2)).toBeDisabled();

  await buttons.nth(1).focus();
  await page.keyboard.press("ArrowDown");
  await expect(buttons.nth(3)).toBeFocused();

  await page.keyboard.press("ArrowUp");
  await expect(buttons.nth(1)).toBeFocused();
});
