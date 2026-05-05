import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const BASE = "http://127.0.0.1:8080";
const URL = `${BASE}/component/?name=drag_and_drop_list&`;
const LOAD_TIMEOUT = 20 * 60 * 1000;

/** Navigate to the DnD page and return the first (main) variant list. */
async function loadMainList(page: import("@playwright/test").Page) {
  await page.goto(URL, { timeout: LOAD_TIMEOUT });
  const list = page.locator(".dx-dnd-list").first();
  await expect(list).toBeVisible({ timeout: 30000 });
  return list;
}

/** Navigate to the DnD page and return the second (removable) variant list. */
async function loadRemovableList(page: import("@playwright/test").Page) {
  await page.goto(URL, { timeout: LOAD_TIMEOUT });
  const list = page.locator(".dx-dnd-list").nth(1);
  await expect(list).toBeVisible({ timeout: 30000 });
  return list;
}

/** Helper to get list items from a dnd-list container. */
function getItems(list: import("@playwright/test").Locator) {
  return list.locator(".dx-dnd-list-item");
}

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
    const lastIndex = (await items.count()) - 1;
    await items.first().click();
    await page.keyboard.press("ArrowUp");
    await expect(items.nth(lastIndex)).toBeFocused();
  });

  test("arrow down from last wraps to first", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    const lastIndex = (await items.count()) - 1;
    await items.nth(lastIndex).click();
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
    const itemCount = await items.count();

    // Grab item 3 (index 2)
    await items.nth(2).click();
    await page.keyboard.press("Enter");
    await expect(liveRegion).toContainText(
      `You have lifted an item in position 3 of ${itemCount}`,
    );

    // First ArrowUp immediately moves to position 2
    await page.keyboard.press("ArrowUp");
    await expect(liveRegion).toContainText(
      `You have moved the item to position 2 of ${itemCount}`,
    );

    // Second ArrowUp moves to position 1
    await page.keyboard.press("ArrowUp");
    await expect(liveRegion).toContainText(
      `You have moved the item to position 1 of ${itemCount}`,
    );

    // ArrowDown back to position 2
    await page.keyboard.press("ArrowDown");
    await expect(liveRegion).toContainText(
      `You have moved the item to position 2 of ${itemCount}`,
    );

    // ArrowDown past original position to position 3
    await page.keyboard.press("ArrowDown");
    await expect(liveRegion).toContainText(
      `You have moved the item to position 3 of ${itemCount}`,
    );

    // ArrowDown to position 4
    await page.keyboard.press("ArrowDown");
    await expect(liveRegion).toContainText(
      `You have moved the item to position 4 of ${itemCount}`,
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
    const itemCount = await items.count();
    await items.first().click();
    await page.keyboard.press("Space");
    const liveRegion = list.locator('[aria-live="assertive"]');
    await expect(liveRegion).toContainText(
      `You have lifted an item in position 1 of ${itemCount}`,
    );
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Space");
    await expect(liveRegion).toContainText(
      "You have dropped the item. It has moved from position 1 to position 2",
    );
  });

  async function dropIndicatorOpacity(
    page: import("@playwright/test").Page,
  ): Promise<number> {
    return page.evaluate(() => {
      const beforeTarget = document.querySelector(
        '.dx-drop-indicator[data-position="before"] + .dx-dnd-list-item',
      );
      if (beforeTarget) {
        return Number.parseFloat(
          getComputedStyle(beforeTarget, "::before").opacity,
        );
      }
      const afterTarget = document.querySelector(
        '.dx-dnd-list-item:has(+ .dx-drop-indicator[data-position="after"])',
      );
      if (afterTarget) {
        return Number.parseFloat(
          getComputedStyle(afterTarget, "::after").opacity,
        );
      }
      return 0;
    });
  }

  test("mouse drag shows the drop-indicator line", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    const sourceBox = await items.nth(2).boundingBox();
    const targetBox = await items.nth(3).boundingBox();
    expect(sourceBox).not.toBeNull();
    expect(targetBox).not.toBeNull();

    await page.mouse.move(
      sourceBox!.x + sourceBox!.width / 2,
      sourceBox!.y + sourceBox!.height / 2,
    );
    await page.mouse.down();
    await page.mouse.move(
      targetBox!.x + targetBox!.width / 2,
      targetBox!.y + targetBox!.height * 0.8,
      { steps: 20 },
    );

    await expect(page.locator(".dx-drop-indicator")).toHaveCount(1);
    await expect.poll(() => dropIndicatorOpacity(page)).toBeGreaterThan(0.5);

    await page.mouse.up();
  });

  test("keyboard drag shows the drop-indicator line", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    // Use keyboard navigation so `:focus-visible` genuinely applies to
    // the item the primitive re-focuses during drag.
    await page.keyboard.press("Tab");
    for (let i = 0; i < 2; i++) {
      await page.keyboard.press("ArrowDown");
    }
    await expect(items.nth(2)).toBeFocused();
    await page.keyboard.press("Enter");
    await page.keyboard.press("ArrowDown");

    await expect.poll(() => dropIndicatorOpacity(page)).toBeGreaterThan(0.5);

    await page.keyboard.press("Escape");
  });

  test("mouse drop to the side commits without cancelling the native drop", async ({
    page,
  }) => {
    const runtimeErrors: string[] = [];
    page.on("console", (message) => {
      if (message.type() === "error") {
        runtimeErrors.push(message.text());
      }
    });
    page.on("pageerror", (error) => {
      runtimeErrors.push(error.message);
    });

    const list = await loadMainList(page);
    const items = getItems(list);
    const sourceTitle = await items.nth(2).locator(".dx-task-title").innerText();
    const sourceBox = await items.nth(2).boundingBox();
    const targetBox = await items.nth(3).boundingBox();
    expect(sourceBox).not.toBeNull();
    expect(targetBox).not.toBeNull();

    await page.evaluate(() => {
      (window as any).__dxLastDropEffect = null;
      document.addEventListener(
        "dragend",
        (event) => {
          (window as any).__dxLastDropEffect =
            event.dataTransfer?.dropEffect ?? null;
        },
        { capture: true, once: true },
      );
    });

    await page.mouse.move(
      sourceBox!.x + sourceBox!.width / 2,
      sourceBox!.y + sourceBox!.height / 2,
    );
    await page.mouse.down();
    await page.mouse.move(
      targetBox!.x + targetBox!.width / 2,
      targetBox!.y + targetBox!.height * 0.8,
      { steps: 20 },
    );
    await page.mouse.move(
      targetBox!.x + targetBox!.width + 120,
      targetBox!.y + targetBox!.height * 0.8,
      { steps: 10 },
    );
    await page.mouse.up();

    await expect
      .poll(() => page.evaluate(() => (window as any).__dxLastDropEffect))
      .toBe("move");
    await expect(items.nth(3).locator(".dx-task-title")).toHaveText(
      sourceTitle,
    );
    expect(runtimeErrors).toEqual([]);
  });

  test("cancelled mouse drag does not reorder the list", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    const sourceText = await items.nth(2).innerText();
    const targetText = await items.nth(3).innerText();
    const sourceBox = await items.nth(2).boundingBox();
    const targetBox = await items.nth(3).boundingBox();
    expect(sourceBox).not.toBeNull();
    expect(targetBox).not.toBeNull();

    await page.mouse.move(
      sourceBox!.x + sourceBox!.width / 2,
      sourceBox!.y + sourceBox!.height / 2,
    );
    await page.mouse.down();
    await page.mouse.move(
      targetBox!.x + targetBox!.width / 2,
      targetBox!.y + targetBox!.height * 0.8,
      { steps: 20 },
    );
    await page.keyboard.press("Escape");
    await page.mouse.up();

    await expect(items.nth(2)).toContainText(sourceText);
    await expect(items.nth(3)).toContainText(targetText);
  });
});

test.describe("Remove behavior", () => {
  test("focus moves to item at same index after removal", async ({
    page,
  }) => {
    const list = await loadRemovableList(page);
    const items = getItems(list);
    const initialCount = await items.count();
    const removeButtons = list.locator(".dx-remove-button");
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
    const removeButtons = list.locator(".dx-remove-button");
    await removeButtons.nth(initialCount - 1).click();
    await expect(items).toHaveCount(initialCount - 1);
    await expect(items.nth(initialCount - 2)).toBeFocused();
  });
});

test.describe("Axe automated scan", () => {
  test("no automatically detectable a11y issues", async ({ page }) => {
    await loadMainList(page);

    const accessibilityScanResults = await new AxeBuilder({ page })
      .include(".dx-dnd-list")
      .disableRules(["color-contrast"])
      .analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
  });
});
