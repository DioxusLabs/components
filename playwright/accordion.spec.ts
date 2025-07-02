import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=accordion&");
  // Get the first .accordion-item
  const accordionItem = page.locator(".accordion-item");
  // Click on the first .accordion-item
  const firstAccordionItem = accordionItem.first();
  await firstAccordionItem.locator("button").click();
  // Verify that the first .accordion-item is expanded (data-open="true")
  await expect(firstAccordionItem).toHaveAttribute("data-open", "true");

  // Click on the second .accordion-item
  const secondAccordionItem = accordionItem.nth(1);
  await secondAccordionItem.locator("button").click();
  // Verify that the second .accordion-item is expanded (data-open="true")
  await expect(secondAccordionItem).toHaveAttribute("data-open", "true");
  // Verify the first .accordion-item is collapsed (data-open="false")
  await expect(firstAccordionItem).toHaveAttribute("data-open", "false");
});
