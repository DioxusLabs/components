import { test, expect, type Page } from "@playwright/test";

const BASE_URL = "http://127.0.0.1:8080";

async function gotoSidebarBlock(page: Page) {
  await page.goto(`${BASE_URL}/component/block/?name=sidebar&variant=main&`, {
    timeout: 20 * 60 * 1000,
  });

  await expect(page.locator(".sidebar-wrapper")).toBeVisible();
}

test("sidebar: preview page renders block", async ({ page }) => {
  await page.goto(`${BASE_URL}/component/?name=sidebar&`, {
    timeout: 20 * 60 * 1000,
  });
  const iframe = page.locator("#component-preview-frame iframe").first();
  await expect(iframe).toBeVisible();
  await expect(iframe).toHaveAttribute(
    "src",
    /component\/block\/\?name=sidebar&variant=main/,
  );

  // Ensure the iframe content actually loads.
  const iframeHandle = await iframe.elementHandle();
  if (!iframeHandle) {
    throw new Error("Sidebar preview iframe was not found");
  }
  const frame = await iframeHandle.contentFrame();
  if (!frame) {
    throw new Error("Sidebar preview iframe has no content frame");
  }

  await expect(frame.locator(".sidebar-wrapper")).toBeVisible();
});

test.describe("sidebar: block route", () => {
  test("desktop: toggles via button and Ctrl+B", async ({ page }) => {
    await gotoSidebarBlock(page);

    const sidebar = page.locator(".sidebar-desktop");
    await expect(sidebar).toHaveAttribute("data-state", "expanded");
    const trigger = page.locator('[data-slot="sidebar-trigger"]');
    await expect(trigger).toHaveAccessibleName("Toggle Sidebar");

    // Toggle via button.
    await trigger.click();
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");
    await trigger.click();
    await expect(sidebar).toHaveAttribute("data-state", "expanded");

    // Toggle via keyboard shortcut (⌘/Ctrl+B).
    await page.keyboard.press("Control+b");
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");
    await page.keyboard.press("Control+b");
    await expect(sidebar).toHaveAttribute("data-state", "expanded");
  });

  test("desktop: side switch updates data-side", async ({ page }) => {
    await gotoSidebarBlock(page);

    const sidebar = page.locator(".sidebar-desktop");
    await expect(sidebar).toHaveAttribute("data-side", "left");

    await page.getByRole("button", { name: "Right" }).click();
    await expect(sidebar).toHaveAttribute("data-side", "right");
    await page.getByRole("button", { name: "Left" }).click();
    await expect(sidebar).toHaveAttribute("data-side", "left");
  });

  test("desktop: icon collapse shows tooltip on focus and preserves accessible names", async ({
    page,
  }) => {
    await gotoSidebarBlock(page);

    const sidebar = page.locator(".sidebar-desktop");
    const trigger = page.locator('[data-slot="sidebar-trigger"]');

    await page.getByRole("button", { name: "Icon" }).click();
    await trigger.click();

    await expect(sidebar).toHaveAttribute("data-state", "collapsed");
    await expect(sidebar).toHaveAttribute("data-collapsible", "icon");

    // In icon-collapsed mode, tooltips should appear on keyboard focus.
    const playground = page
      .locator('[data-sidebar="menu-button"]')
      .filter({ hasText: "Playground" })
      .first();

    await playground.focus();

    const tooltip = page.getByRole("tooltip");
    await expect(tooltip).toBeVisible();
    await expect(tooltip).toContainText("Playground");

    // Even when labels are visually hidden in icon mode, the control should still have an accessible name.
    await expect(playground).toHaveAccessibleName("Playground");
  });

  test("mobile: opens as a sheet and closes with Escape (focus restored)", async ({
    page,
  }) => {
    await gotoSidebarBlock(page);

    const trigger = page.locator('[data-slot="sidebar-trigger"]');
    await trigger.tap();

    const sheet = page.locator(".sheet-root");
    await expect(sheet).toHaveAttribute("data-state", "open");
    await page.keyboard.press("Escape");
    await expect(sheet).toHaveCount(0);
    await expect(trigger).toBeFocused();
  });
});
