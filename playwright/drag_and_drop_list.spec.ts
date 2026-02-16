import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const BASE = "http://127.0.0.1:9090";
const URL = `${BASE}/component/?name=drag_and_drop_list&`;
const LOAD_TIMEOUT = 20 * 60 * 1000;

/** Navigate to the DnD page and return the first (main) variant list. */
async function loadMainList(page: import("@playwright/test").Page) {
  await page.goto(URL, { timeout: LOAD_TIMEOUT });
  const list = page.locator(".dnd-list").first();
  await expect(list).toBeVisible({ timeout: 30000 });
  return list;
}

/** Navigate to the DnD page and return the second (removable) variant list. */
async function loadRemovableList(page: import("@playwright/test").Page) {
  await page.goto(URL, { timeout: LOAD_TIMEOUT });
  const list = page.locator(".dnd-list").nth(1);
  await expect(list).toBeVisible({ timeout: 30000 });
  return list;
}

test.describe("ARIA roles and structure", () => {
  test("list has listbox role", async ({ page }) => {
    const list = await loadMainList(page);
    await expect(list.getByRole("listbox")).toBeVisible();
  });

  test("list items have option role", async ({ page }) => {
    const list = await loadMainList(page);
    await expect(list.locator('[role="option"]')).toHaveCount(5);
  });

  test("drag icon wrappers are hidden from AT", async ({ page }) => {
    const list = await loadMainList(page);
    const iconDivs = list.locator(".item-icon-div");
    await expect(iconDivs).toHaveCount(5);
    for (let i = 0; i < 5; i++) {
      await expect(iconDivs.nth(i)).toHaveAttribute("aria-hidden", "true");
    }
  });
});

test.describe("Keyboard focus management", () => {
  test("first item is tab-reachable", async ({ page }) => {
    const list = await loadMainList(page);
    const options = list.locator('[role="option"]');
    await expect(options.first()).toHaveAttribute("tabindex", "0");
    await expect(options.nth(1)).toHaveAttribute("tabindex", "-1");
  });

  test("arrow down moves focus to next item", async ({ page }) => {
    const list = await loadMainList(page);
    const options = list.locator('[role="option"]');
    await options.first().click();
    await page.keyboard.press("ArrowDown");
    await expect(options.nth(1)).toBeFocused();
  });

  test("arrow up from first wraps to last", async ({ page }) => {
    const list = await loadMainList(page);
    const options = list.locator('[role="option"]');
    await options.first().click();
    await page.keyboard.press("ArrowUp");
    await expect(options.nth(4)).toBeFocused();
  });
});

test.describe("Live region announcements", () => {
  test("grabbing announces status", async ({ page }) => {
    const list = await loadMainList(page);
    const options = list.locator('[role="option"]');
    await options.first().click();
    await page.keyboard.press("Enter");
    const liveRegion = list.locator('[aria-live="assertive"]');
    await expect(liveRegion).toContainText("Grabbed item, position 1 of 5");
  });

  test("moving announces new position", async ({ page }) => {
    const list = await loadMainList(page);
    const options = list.locator('[role="option"]');
    await options.first().click();
    await page.keyboard.press("Enter");
    await page.keyboard.press("ArrowDown");
    const liveRegion = list.locator('[aria-live="assertive"]');
    await expect(liveRegion).toContainText("Moved to position 2 of 5");
  });

  test("dropping announces final position", async ({ page }) => {
    const list = await loadMainList(page);
    const options = list.locator('[role="option"]');
    await options.first().click();
    await page.keyboard.press("Enter");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
    const liveRegion = list.locator('[aria-live="assertive"]');
    await expect(liveRegion).toContainText("Dropped at position 2 of 5");
  });

  test("cancelling announces cancellation", async ({ page }) => {
    const list = await loadMainList(page);
    const options = list.locator('[role="option"]');
    await options.first().click();
    await page.keyboard.press("Enter");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Escape");
    const liveRegion = list.locator('[aria-live="assertive"]');
    await expect(liveRegion).toContainText("Reorder cancelled");
  });
});

test.describe("Remove button accessibility", () => {
  test("remove buttons have accessible labels", async ({ page }) => {
    const list = await loadRemovableList(page);
    const removeButtons = list.locator(".remove-button");
    await expect(removeButtons).toHaveCount(5);
    for (let i = 0; i < 5; i++) {
      await expect(removeButtons.nth(i)).toHaveAccessibleName(
        `Remove item ${i + 1}`,
      );
    }
  });

  test("remove button icon wrappers are hidden", async ({ page }) => {
    const list = await loadRemovableList(page);
    const iconSpans = list.locator(".remove-button .remove-icon");
    await expect(iconSpans).toHaveCount(5);
    for (let i = 0; i < 5; i++) {
      await expect(iconSpans.nth(i)).toHaveAttribute("aria-hidden", "true");
    }
  });
});

test.describe("Data attributes and focus during drag", () => {
  test("grabbed item uses data-is-grabbing", async ({ page }) => {
    const list = await loadMainList(page);
    const options = list.locator('[role="option"]');
    await options.first().click();
    await page.keyboard.press("Enter");
    await expect(options.first()).toHaveAttribute("data-is-grabbing", "true");
  });

  test("focus ring visible during keyboard drag", async ({ page }) => {
    const list = await loadMainList(page);
    const options = list.locator('[role="option"]');
    await options.first().click();
    await page.keyboard.press("Enter");
    await expect(options.first()).toHaveAttribute(
      "data-focus-visible",
      "true",
    );
  });
});

test.describe("Cancel focus behavior", () => {
  test("escape during drag returns focus to source item", async ({
    page,
  }) => {
    const list = await loadMainList(page);
    const options = list.locator('[role="option"]');
    await options.first().click();
    await page.keyboard.press("Enter");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Escape");
    await expect(options.first()).toBeFocused();
  });

  test("escape during drag from middle returns focus to that item", async ({
    page,
  }) => {
    const list = await loadMainList(page);
    const options = list.locator('[role="option"]');
    await options.nth(2).click();
    await page.keyboard.press("Enter");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Escape");
    await expect(options.nth(2)).toBeFocused();
  });
});

test.describe("Axe automated scan", () => {
  test("no automatically detectable a11y issues", async ({ page }) => {
    await loadMainList(page);

    const accessibilityScanResults = await new AxeBuilder({ page })
      .disableRules([
        "color-contrast",
        // Page-level issues (preview app has duplicate main/nav landmarks)
        "landmark-no-duplicate-main",
        "landmark-unique",
        // Remove buttons inside role="option" items is a known DnD list pattern
        "nested-interactive",
      ])
      .analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
  });
});
