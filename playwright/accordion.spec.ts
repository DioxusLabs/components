import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=accordion&", { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  const accordionItems = page.locator("[data-open]").filter({ has: page.getByRole("button") });
  const buttons = accordionItems.getByRole("button");
  const firstAccordionItem = accordionItems.first();
  await buttons.first().click();
  await expect(firstAccordionItem).toHaveAttribute("data-open", "true");

  const secondAccordionItem = accordionItems.nth(1);
  await buttons.nth(1).click();
  await expect(secondAccordionItem).toHaveAttribute("data-open", "true");
  await expect(firstAccordionItem).toHaveAttribute("data-open", "false");
});

test("keyboard navigation skips disabled items", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=accordion&", { timeout: 20 * 60 * 1000 });
  const accordionItems = page.locator("[data-open]").filter({ has: page.getByRole("button") });
  const buttons = accordionItems.getByRole("button");

  await expect(accordionItems.nth(2)).toHaveAttribute("data-disabled", "true");
  await expect(buttons.nth(2)).toBeDisabled();

  await buttons.nth(1).focus();
  await page.keyboard.press("ArrowDown");
  await expect(buttons.nth(3)).toBeFocused();

  await page.keyboard.press("ArrowUp");
  await expect(buttons.nth(1)).toBeFocused();
});
