import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=avatar&");
  // Get the avatar element
  const avatar = page.locator(".avatar-item").nth(0);
  // Verify the avatar has a loaded image
  let image = avatar.locator("img");
  await expect(image).toHaveAttribute("src", "https://avatars.githubusercontent.com/u/66571940?s=96&v=4");

  // Get the second avatar element
  const secondAvatar = page.locator(".avatar-item").nth(1);
  // Verify the second avatar has fallback text
  await expect(secondAvatar).toContainText("JK");
});
