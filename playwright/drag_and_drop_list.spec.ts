import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const BASE = "http://127.0.0.1:8080";
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

/** Helper to get list items from a dnd-list container. */
function getItems(list: import("@playwright/test").Locator) {
  return list.locator(".dnd-list-item");
}

test.describe("ARIA roles and structure", () => {
  test("list has sortable list roledescription", async ({ page }) => {
    const list = await loadMainList(page);
    const ul = list.locator(".dnd-list-ul");
    await expect(ul).toHaveAttribute("aria-roledescription", "sortable list");
  });

  test("list items have sortable item roledescription", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    await expect(items).toHaveCount(5);
    for (let i = 0; i < 5; i++) {
      await expect(items.nth(i)).toHaveAttribute(
        "aria-roledescription",
        "sortable item",
      );
    }
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
    const items = getItems(list);
    await expect(items.first()).toHaveAttribute("tabindex", "0");
    await expect(items.nth(1)).toHaveAttribute("tabindex", "-1");
  });

  test("arrow up from first wraps to last", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    await items.first().click();
    await page.keyboard.press("ArrowUp");
    await expect(items.nth(4)).toBeFocused();
  });

  test("arrow down from last wraps to first", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    await items.nth(4).click();
    await page.keyboard.press("ArrowDown");
    await expect(items.first()).toBeFocused();
  });

  test("roving tabindex updates on arrow navigation", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    await items.first().click();
    await page.keyboard.press("ArrowDown");
    await expect(items.first()).toHaveAttribute("tabindex", "-1");
    await expect(items.nth(1)).toHaveAttribute("tabindex", "0");
  });
});

test.describe("Drag and drop lifecycle", () => {
  test("each arrow press moves one position with announcement", async ({
    page,
  }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    const liveRegion = list.locator('[aria-live="assertive"]');

    // Grab item 3 (index 2)
    await items.nth(2).click();
    await page.keyboard.press("Enter");
    await expect(liveRegion).toContainText(
      "You have lifted an item in position 3 of 5",
    );

    // First ArrowUp immediately moves to position 2
    await page.keyboard.press("ArrowUp");
    await expect(liveRegion).toContainText(
      "You have moved the item to position 2 of 5",
    );

    // Second ArrowUp moves to position 1
    await page.keyboard.press("ArrowUp");
    await expect(liveRegion).toContainText(
      "You have moved the item to position 1 of 5",
    );

    // ArrowDown back to position 2
    await page.keyboard.press("ArrowDown");
    await expect(liveRegion).toContainText(
      "You have moved the item to position 2 of 5",
    );

    // ArrowDown past original position to position 3
    await page.keyboard.press("ArrowDown");
    await expect(liveRegion).toContainText(
      "You have moved the item to position 3 of 5",
    );

    // ArrowDown to position 4
    await page.keyboard.press("ArrowDown");
    await expect(liveRegion).toContainText(
      "You have moved the item to position 4 of 5",
    );

    // Drop
    await page.keyboard.press("Enter");
    await expect(liveRegion).toContainText(
      "You have dropped the item. It has moved from position 3 to position 4",
    );
  });

  test("cancelling announces and returns focus to source", async ({
    page,
  }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    await items.first().click();
    await page.keyboard.press("Enter");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Escape");
    const liveRegion = list.locator('[aria-live="assertive"]');
    await expect(liveRegion).toContainText(
      "Movement cancelled. The item has returned to its starting position of 1",
    );
    await expect(items.first()).toBeFocused();
  });

  test("grabbed item has aria-grabbed true", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    await items.first().click();
    await page.keyboard.press("Enter");
    await expect(items.first()).toHaveAttribute("aria-grabbed", "true");
  });

  test("focus after successful drop lands on moved item", async ({
    page,
  }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    await items.first().click();
    await page.keyboard.press("Enter");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
    await expect(items.nth(1)).toBeFocused();
  });

  test("space key grabs and drops items", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    await items.first().click();
    await page.keyboard.press("Space");
    const liveRegion = list.locator('[aria-live="assertive"]');
    await expect(liveRegion).toContainText(
      "You have lifted an item in position 1 of 5",
    );
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Space");
    await expect(liveRegion).toContainText(
      "You have dropped the item. It has moved from position 1 to position 2",
    );
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

  test("focus moves to item at same index after removal", async ({
    page,
  }) => {
    const list = await loadRemovableList(page);
    const items = getItems(list);
    const initialCount = await items.count();
    const removeButtons = list.locator(".remove-button");
    await removeButtons.nth(2).click();
    await expect(items).toHaveCount(initialCount - 1);
    await expect(items.nth(2)).toBeFocused();
  });

  test("focus moves to new last item when removing last item", async ({
    page,
  }) => {
    const list = await loadRemovableList(page);
    const items = getItems(list);
    const initialCount = await items.count();
    const removeButtons = list.locator(".remove-button");
    await removeButtons.nth(initialCount - 1).click();
    await expect(items).toHaveCount(initialCount - 1);
    await expect(items.nth(initialCount - 2)).toBeFocused();
  });
});

test.describe("Axe automated scan", () => {
  test("no automatically detectable a11y issues", async ({ page }) => {
    await loadMainList(page);

    const accessibilityScanResults = await new AxeBuilder({ page })
      .include(".dnd-list")
      .disableRules(["color-contrast"])
      .analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
  });
});
