import { test, expect } from "@playwright/test";

test("card component structure", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=card&", {
    timeout: 20 * 60 * 1000,
  });

  // Find the card element
  const card = page.locator(".card").nth(0);

  // Assert the card is visible with correct data-slot
  await expect(card).toBeVisible();
  await expect(card).toHaveAttribute("data-slot", "card");

  // Assert card-header structure
  const cardHeader = card.locator(".card-header");
  await expect(cardHeader).toBeVisible();
  await expect(cardHeader).toHaveAttribute("data-slot", "card-header");

  // Assert card-title structure
  const cardTitle = card.locator(".card-title");
  await expect(cardTitle).toBeVisible();
  await expect(cardTitle).toHaveAttribute("data-slot", "card-title");

  // Assert card-description structure
  const cardDescription = card.locator(".card-description");
  await expect(cardDescription).toBeVisible();
  await expect(cardDescription).toHaveAttribute("data-slot", "card-description");

  // Assert card-action structure
  const cardAction = card.locator(".card-action");
  await expect(cardAction).toBeVisible();
  await expect(cardAction).toHaveAttribute("data-slot", "card-action");

  // Assert card-content structure
  const cardContent = card.locator(".card-content");
  await expect(cardContent).toBeVisible();
  await expect(cardContent).toHaveAttribute("data-slot", "card-content");

  // Assert card-footer structure
  const cardFooter = card.locator(".card-footer");
  await expect(cardFooter).toBeVisible();
  await expect(cardFooter).toHaveAttribute("data-slot", "card-footer");
});
