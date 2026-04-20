import { expect, test } from "@playwright/test";

test.describe("Item Component", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=item&");
  });

  test("should render variant styles correctly", async ({ page }) => {
    await expect(
      page.locator('.item[data-style="default"]').first(),
    ).toBeVisible();
    await expect(
      page.locator('.item[data-style="outline"]').first(),
    ).toBeVisible();
    await expect(
      page.locator('.item[data-style="muted"]').first(),
    ).toBeVisible();
  });

  test("should apply size variants and check text content", async ({
    page,
  }) => {
    const smItem = page.locator('.item[data-size="sm"]').first();
    await expect(smItem).toBeVisible();
    await expect(smItem).toContainText("Small Size");

    const xsItem = page.locator('.item[data-size="xs"]').first();
    await expect(xsItem).toBeVisible();
    await expect(xsItem).toContainText("Extra Small Size");
  });

  test("should render image media and composition elements", async ({
    page,
  }) => {
    // Select the item with the "Default Size" title which has the full composition
    const fullItem = page
      .locator(".item")
      .filter({ hasText: "Default Size" })
      .first();

    // Verify Media Image
    const image = fullItem.locator('.item-media[data-style="image"] img');
    await expect(image).toBeVisible();
    await expect(image).toHaveAttribute("src", /unsplash/);

    // Verify Header
    await expect(fullItem.locator(".item-header")).toContainText("Transaction");

    // Verify Actions content
    const actions = fullItem.locator(".item-actions");
    await expect(actions).toContainText("-$24.99");

    // Verify Footer and Button interactivity
    const dismissBtn = fullItem
      .locator(".item-footer button")
      .filter({ hasText: "Dismiss" });
    await expect(dismissBtn).toBeVisible();
    await dismissBtn.click();
  });
});
