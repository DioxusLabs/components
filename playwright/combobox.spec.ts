import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const URL = "http://127.0.0.1:8080/component/?name=combobox&";

const input = (page: Page) =>
    page.getByRole("combobox", { name: "Select framework" });

const content = (page: Page) =>
    page.locator(".dx-combobox-content[data-state='open']");

const list = (page: Page) =>
    page.locator(".dx-combobox-content[data-state='open'] .dx-combobox-list");

test("has no automatically detectable accessibility issues", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const trigger = input(page);
    await expect(trigger).toBeVisible();
    await trigger.focus();
    await expect(trigger).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(content(page)).toBeVisible();

    const accessibilityScanResults = await new AxeBuilder({ page })
        .disableRules("color-contrast")
        .analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
});

test("filters and selects with the keyboard", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });

    const trigger = input(page);
    await trigger.click();
    await expect(content(page)).toBeVisible();

    await page.keyboard.type("sve");
    const svelte = list(page).getByRole("option", { name: "SvelteKit" });
    await expect(svelte).toBeVisible();

    await page.keyboard.press("ArrowDown");
    await expect(svelte).toHaveAttribute("data-highlighted", "true");

    await page.keyboard.press("Enter");
    await expect(content(page)).toHaveCount(0);
    await expect(trigger).toHaveValue("SvelteKit");

    await trigger.click();
    await expect(svelte).toHaveAttribute("aria-selected", "true");

    await page.keyboard.press("Escape");
    await expect(content(page)).toHaveCount(0);
    await expect(trigger).toHaveValue("SvelteKit");
});

test("arrow keys stay on visible filtered options", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });

    const trigger = input(page);
    await trigger.click();
    await page.keyboard.type("sve");
    await expect(trigger).toBeFocused();

    const svelte = list(page).getByRole("option", { name: "SvelteKit" });
    await expect(svelte).toBeVisible();

    await page.keyboard.press("ArrowDown");
    await expect(svelte).toHaveAttribute("data-highlighted", "true");

    await page.keyboard.press("ArrowDown");
    await expect(svelte).toHaveAttribute("data-highlighted", "true");

    await page.keyboard.press("ArrowUp");
    await expect(svelte).toHaveAttribute("data-highlighted", "true");
});

test("keeps filtered options in source order", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });

    const trigger = input(page);
    await trigger.click();
    await page.keyboard.type("s");

    const next = list(page).getByRole("option", { name: "Next.js" });
    const svelte = list(page).getByRole("option", { name: "SvelteKit" });
    const solid = list(page).getByRole("option", { name: "SolidStart" });

    await expect(next).toBeVisible();
    await expect(svelte).toBeVisible();
    await expect(solid).toBeVisible();

    const nextBox = await next.boundingBox();
    const svelteBox = await svelte.boundingBox();
    expect(nextBox).not.toBeNull();
    expect(svelteBox).not.toBeNull();
    expect(nextBox!.y).toBeLessThan(svelteBox!.y);

    await page.keyboard.press("ArrowDown");
    await expect(next).toHaveAttribute("data-highlighted", "true");

    await page.keyboard.press("ArrowDown");
    await expect(svelte).toHaveAttribute("data-highlighted", "true");

    await page.keyboard.press("ArrowUp");
    await expect(next).toHaveAttribute("data-highlighted", "true");
});

test("keeps filtered options during keyboard close animation", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });

    const trigger = input(page);
    await trigger.click();
    await page.keyboard.type("sve");

    const svelte = list(page).getByRole("option", { name: "SvelteKit" });
    await page.keyboard.press("ArrowDown");
    await expect(svelte).toHaveAttribute("data-highlighted", "true");

    await page.keyboard.press("Enter");

    const closingContent = page.locator(".dx-combobox-content[data-state='closed']");
    await expect(closingContent).toBeVisible();
    await expect(closingContent.getByRole("option", { name: "SvelteKit" })).toBeVisible();
    await expect(closingContent.getByRole("option")).toHaveCount(1);
    await expect(content(page)).toHaveCount(0);
});

test("clicking an option commits and closes", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });

    const trigger = input(page);
    await trigger.click();
    await list(page).getByRole("option", { name: "Dioxus" }).click();

    await expect(content(page)).toHaveCount(0);
    await expect(trigger).toHaveValue("Dioxus");
});
