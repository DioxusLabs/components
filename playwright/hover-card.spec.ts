import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=hover_card&");
  let tooltip = page.getByRole("tooltip");
  // tabbing to the trigger element should show the tooltip
  await page.getByRole("heading", { name: "hover card" }).click();
  await page.keyboard.press("Tab");
  await expect(tooltip).toBeVisible();
  // tabbing out of the trigger element should hide the tooltip
  await page.keyboard.press("Tab");
  await expect(tooltip).toHaveCount(0);

  // hovering over the trigger element should show the tooltip
  await page.getByRole("button", { name: "Dioxus" }).hover();
  await expect(tooltip).toBeVisible();

  // moving the mouse away from the trigger element should hide the tooltip
  await page.mouse.move(0, 0);
  await expect(tooltip).toHaveCount(0);
});
