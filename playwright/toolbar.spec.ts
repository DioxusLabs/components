import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=toolbar&");
  let bold = page.getByRole("button", { name: "Bold" });
  let italic = page.getByRole("button", { name: "Italic" });
  let underline = page.getByRole("button", { name: "Underline" });
  let alignLeft = page.getByRole("button", { name: "Align Left" });
  let alignCenter = page.getByRole("button", { name: "Align Center" });
  let alignRight = page.getByRole("button", { name: "Align Right" });
  await page.getByRole("heading", { name: "Toolbar" }).click();
  // Tabbing to the first button should focus it
  await page.keyboard.press("Tab");
  await expect(bold).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(italic).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(underline).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(alignLeft).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(alignCenter).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(alignRight).toBeFocused();
});
