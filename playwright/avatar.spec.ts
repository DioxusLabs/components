import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=avatar&", { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  // Get the avatar element
  const avatar = page.locator(".avatar-item").nth(0);
  // Verify the avatar has a loaded image
  let image = avatar.locator("img");
  await expect(image).toHaveAttribute("src", "https://avatars.githubusercontent.com/u/66571940?s=96&v=4");

  // Get the third avatar element (Error State - has invalid image URL, shows fallback)
  const errorAvatar = page.locator(".avatar-item").nth(2);
  // Verify the error state avatar has fallback text
  await expect(errorAvatar).toContainText("JK");
});
