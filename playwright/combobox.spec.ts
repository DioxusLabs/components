import { test, expect, type Page } from "@playwright/test";

const SINGLE_URL = "http://127.0.0.1:8080/component/?name=combobox&";
const MULTI_URL = "http://127.0.0.1:8080/component/?name=combobox&variant=multi&";

const input = (page: Page) => page.locator(".dx-combobox-input").first();
const content = (page: Page) => page.locator(".dx-combobox-content").first();
const list = (page: Page) => page.locator(".dx-combobox-list").first();

test("opening, filtering, and selecting via keyboard", async ({ page }) => {
    await page.goto(SINGLE_URL, { timeout: 20 * 60 * 1000 });

    const trigger = input(page);
    await expect(trigger).toBeVisible();
    await trigger.click();
    await expect(content(page)).toHaveAttribute("data-state", "open");

    // Subsequence/substring filter ranks SvelteKit first when the user types "sv".
    await page.keyboard.type("sv");
    const svelte = list(page).getByRole("option", { name: "SvelteKit" });
    await page.keyboard.press("ArrowDown");
    await expect(svelte).toHaveAttribute("data-highlighted", "true");

    await page.keyboard.press("Enter");
    // Single-select closes the popup and the input shows the chosen text.
    await expect(content(page)).toHaveCount(0);
    await expect(trigger).toHaveValue("SvelteKit");

    // Reopening keeps the previous selection marked aria-selected.
    await trigger.click();
    await expect(content(page)).toHaveAttribute("data-state", "open");
    await expect(list(page).getByRole("option", { name: "SvelteKit" })).toHaveAttribute(
        "aria-selected",
        "true",
    );

    // Escape closes without changing the selection.
    await page.keyboard.press("Escape");
    await expect(content(page)).toHaveCount(0);
    await expect(trigger).toHaveValue("SvelteKit");
});

test("clicking an option commits and closes (single)", async ({ page }) => {
    await page.goto(SINGLE_URL, { timeout: 20 * 60 * 1000 });

    const trigger = input(page);
    await trigger.click();
    await expect(content(page)).toHaveAttribute("data-state", "open");

    await list(page).getByRole("option", { name: "Dioxus" }).click();
    await expect(content(page)).toHaveCount(0);
    await expect(trigger).toHaveValue("Dioxus");
});

test("multi: default values render comma-joined and listbox is multiselectable", async ({ page }) => {
    await page.goto(MULTI_URL, { timeout: 20 * 60 * 1000 });

    const trigger = input(page);
    // Default values from the demo: Next.js + Dioxus, in registration order.
    await expect(trigger).toHaveValue("Next.js, Dioxus");

    await trigger.click();
    await expect(content(page)).toHaveAttribute("data-state", "open");
    await expect(list(page)).toHaveAttribute("aria-multiselectable", "true");

    const next = list(page).getByRole("option", { name: "Next.js" });
    const dioxus = list(page).getByRole("option", { name: "Dioxus" });
    const svelte = list(page).getByRole("option", { name: "SvelteKit" });

    await expect(next).toHaveAttribute("aria-selected", "true");
    await expect(dioxus).toHaveAttribute("aria-selected", "true");
    await expect(svelte).toHaveAttribute("aria-selected", "false");
});

test("multi: clicking toggles options and keeps the popup open", async ({ page }) => {
    await page.goto(MULTI_URL, { timeout: 20 * 60 * 1000 });

    const trigger = input(page);
    await trigger.click();
    await expect(content(page)).toHaveAttribute("data-state", "open");

    const svelte = list(page).getByRole("option", { name: "SvelteKit" });
    const next = list(page).getByRole("option", { name: "Next.js" });

    // Toggle on an unselected option — popup stays open.
    await svelte.click();
    await expect(content(page)).toHaveAttribute("data-state", "open");
    await expect(svelte).toHaveAttribute("aria-selected", "true");

    // Toggle off an already-selected option — popup stays open.
    await next.click();
    await expect(content(page)).toHaveAttribute("data-state", "open");
    await expect(next).toHaveAttribute("aria-selected", "false");

    // Escape closes; the input reflects the latest selection.
    await page.keyboard.press("Escape");
    await expect(content(page)).toHaveCount(0);
    await expect(trigger).toHaveValue("Dioxus, SvelteKit");
});

test("multi: keyboard Enter toggles, popup stays open, query persists", async ({ page }) => {
    await page.goto(MULTI_URL, { timeout: 20 * 60 * 1000 });

    const trigger = input(page);
    await trigger.click();
    await expect(content(page)).toHaveAttribute("data-state", "open");

    // Type "sv" so SvelteKit ranks first; arrow + Enter toggles it on.
    await page.keyboard.type("sv");
    await page.keyboard.press("ArrowDown");
    const svelte = list(page).getByRole("option", { name: "SvelteKit" });
    await expect(svelte).toHaveAttribute("data-highlighted", "true");
    await page.keyboard.press("Enter");

    // Multi mode: popup stays open, query is preserved.
    await expect(content(page)).toHaveAttribute("data-state", "open");
    await expect(svelte).toHaveAttribute("aria-selected", "true");
    await expect(trigger).toHaveValue("sv");
});

test("mobile: multi tap on options keeps the popup open", async ({ page }) => {
    await page.goto(MULTI_URL, { timeout: 20 * 60 * 1000 });

    const trigger = input(page);
    await trigger.tap();
    await expect(content(page)).toHaveAttribute("data-state", "open");

    const svelte = list(page).getByRole("option", { name: "SvelteKit" });
    const remix = list(page).getByRole("option", { name: "Remix" });

    await svelte.tap();
    await expect(content(page)).toHaveAttribute("data-state", "open");
    await expect(svelte).toHaveAttribute("aria-selected", "true");

    await remix.tap();
    await expect(content(page)).toHaveAttribute("data-state", "open");
    await expect(remix).toHaveAttribute("aria-selected", "true");
});
