import { test, expect } from "@playwright/test";

/**
 * Style Isolation Tests
 *
 * These tests verify that component styles are properly scoped with the `dx-` prefix
 * and that external CSS conflicts do not affect component appearance.
 */

test.describe("Style Isolation", () => {
  // Disable CSS transitions to prevent flaky color comparisons during animation
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      const style = document.createElement("style");
      style.id = "disable-transitions";
      style.textContent = "*, *::before, *::after { transition: none !important; }";
      if (document.head) {
        document.head.appendChild(style);
      }
      document.addEventListener("DOMContentLoaded", () => {
        if (!document.getElementById("disable-transitions")) {
          document.head.appendChild(style);
        }
      });
    });
  });

  test("button component resists external CSS conflicts", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=button&", {
      timeout: 20 * 60 * 1000,
    });

    // Wait for the component to load and styles to settle
    const button = page.locator(".dx-button").first();
    await expect(button).toBeVisible({ timeout: 30000 });
    await page.waitForTimeout(500);

    // Capture original computed styles
    const originalStyles = await button.evaluate((el) => {
      const styles = getComputedStyle(el);
      return {
        backgroundColor: styles.backgroundColor,
        color: styles.color,
        padding: styles.padding,
        borderRadius: styles.borderRadius,
      };
    });

    // Inject conflicting CSS targeting generic class names (without dx- prefix)
    await page.addStyleTag({
      content: `
        .button {
          background-color: red !important;
          color: yellow !important;
          padding: 100px !important;
          border-radius: 0 !important;
        }
      `,
    });

    // Wait a moment for styles to apply
    await page.waitForTimeout(100);

    // Verify the dx-prefixed component is unaffected
    const afterStyles = await button.evaluate((el) => {
      const styles = getComputedStyle(el);
      return {
        backgroundColor: styles.backgroundColor,
        color: styles.color,
        padding: styles.padding,
        borderRadius: styles.borderRadius,
      };
    });

    // The dx-button should not be affected by .button styles
    expect(afterStyles.backgroundColor).toBe(originalStyles.backgroundColor);
    expect(afterStyles.color).toBe(originalStyles.color);
    expect(afterStyles.padding).toBe(originalStyles.padding);
    expect(afterStyles.borderRadius).toBe(originalStyles.borderRadius);
  });

  test("checkbox component resists external CSS conflicts", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=checkbox&", {
      timeout: 20 * 60 * 1000,
    });

    // Wait for the component to load and styles to settle
    const checkbox = page.locator(".dx-checkbox").first();
    await expect(checkbox).toBeVisible({ timeout: 30000 });
    await page.waitForTimeout(500);

    // Capture original computed styles
    const originalStyles = await checkbox.evaluate((el) => {
      const styles = getComputedStyle(el);
      return {
        width: styles.width,
        height: styles.height,
        borderRadius: styles.borderRadius,
      };
    });

    // Inject conflicting CSS targeting generic class names
    await page.addStyleTag({
      content: `
        .checkbox {
          width: 500px !important;
          height: 500px !important;
          border-radius: 0 !important;
        }
      `,
    });

    // Wait a moment for styles to apply
    await page.waitForTimeout(100);

    // Verify the dx-prefixed component is unaffected
    const afterStyles = await checkbox.evaluate((el) => {
      const styles = getComputedStyle(el);
      return {
        width: styles.width,
        height: styles.height,
        borderRadius: styles.borderRadius,
      };
    });

    expect(afterStyles.width).toBe(originalStyles.width);
    expect(afterStyles.height).toBe(originalStyles.height);
    expect(afterStyles.borderRadius).toBe(originalStyles.borderRadius);
  });

  test("slider component resists external CSS conflicts", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=slider&", {
      timeout: 20 * 60 * 1000,
    });

    // Wait for the component to load and styles to settle
    const slider = page.locator(".dx-slider").first();
    await expect(slider).toBeVisible({ timeout: 30000 });
    await page.waitForTimeout(500);

    // Capture original computed styles
    const originalStyles = await slider.evaluate((el) => {
      const styles = getComputedStyle(el);
      return {
        height: styles.height,
        backgroundColor: styles.backgroundColor,
      };
    });

    // Inject conflicting CSS targeting generic class names
    await page.addStyleTag({
      content: `
        .slider {
          height: 500px !important;
          background-color: red !important;
        }
        .slider-track {
          height: 500px !important;
        }
        .slider-thumb {
          width: 500px !important;
          height: 500px !important;
        }
      `,
    });

    // Wait a moment for styles to apply
    await page.waitForTimeout(100);

    // Verify the dx-prefixed component is unaffected
    const afterStyles = await slider.evaluate((el) => {
      const styles = getComputedStyle(el);
      return {
        height: styles.height,
        backgroundColor: styles.backgroundColor,
      };
    });

    expect(afterStyles.height).toBe(originalStyles.height);
    expect(afterStyles.backgroundColor).toBe(originalStyles.backgroundColor);
  });

  test("dialog component resists external CSS conflicts", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=dialog&", {
      timeout: 20 * 60 * 1000,
    });

    // Open the dialog
    await page.getByRole("button", { name: "Show Dialog" }).click();

    // Wait for the dialog to appear
    const dialog = page.locator(".dx-dialog-backdrop");
    await expect(dialog).toHaveAttribute("data-state", "open");

    const dialogContent = page.locator(".dx-dialog").first();
    await expect(dialogContent).toBeVisible();

    // Capture original computed styles
    const originalStyles = await dialogContent.evaluate((el) => {
      const styles = getComputedStyle(el);
      return {
        backgroundColor: styles.backgroundColor,
        padding: styles.padding,
        borderRadius: styles.borderRadius,
      };
    });

    // Inject conflicting CSS targeting generic class names
    await page.addStyleTag({
      content: `
        .dialog {
          background-color: red !important;
          padding: 500px !important;
        }
        .dialog-content {
          background-color: green !important;
          padding: 500px !important;
          border-radius: 0 !important;
        }
        .dialog-backdrop {
          background-color: yellow !important;
        }
      `,
    });

    // Wait a moment for styles to apply
    await page.waitForTimeout(100);

    // Verify the dx-prefixed component is unaffected
    const afterStyles = await dialogContent.evaluate((el) => {
      const styles = getComputedStyle(el);
      return {
        backgroundColor: styles.backgroundColor,
        padding: styles.padding,
        borderRadius: styles.borderRadius,
      };
    });

    expect(afterStyles.backgroundColor).toBe(originalStyles.backgroundColor);
    expect(afterStyles.padding).toBe(originalStyles.padding);
    expect(afterStyles.borderRadius).toBe(originalStyles.borderRadius);
  });

  test("all component classes use dx- prefix", async ({ page }) => {
    // List of components to check
    const components = [
      "button",
      "checkbox",
      "slider",
      "dialog",
      "accordion",
      "tabs",
      "calendar",
      "select",
      "menubar",
      "tooltip",
    ];

    for (const component of components) {
      await page.goto(`http://127.0.0.1:8080/component/?name=${component}&`, {
        timeout: 20 * 60 * 1000,
      });

      // Wait for component to load
      await page.waitForTimeout(1000);

      // Find all elements with class attributes and verify dx- prefix
      const unprefixedClasses = await page.evaluate(() => {
        const elements = document.querySelectorAll("[class]");
        const problematicClasses: string[] = [];

        elements.forEach((el) => {
          const classAttr = el.getAttribute("class") || "";
          const classes = classAttr.split(" ");
          classes.forEach((cls) => {
            // Skip if it's:
            // - already prefixed with dx-
            // - doesn't look like a component class (no hyphens or too short)
            if (
              cls.startsWith("dx-") ||
              !cls.includes("-") ||
              cls.length < 3
            ) {
              return;
            }

            // Check if this looks like a component class (has hyphen, not a utility)
            // Common component patterns: button-primary, slider-thumb, dialog-content
            const componentPatterns = [
              /^(button|checkbox|slider|dialog|accordion|tabs|calendar|select|menubar|tooltip|avatar|badge|card|navbar|sheet|sidebar|popover|progress|radio|switch|textarea|toggle)-/,
            ];

            if (componentPatterns.some((pattern) => pattern.test(cls))) {
              problematicClasses.push(cls);
            }
          });
        });

        return [...new Set(problematicClasses)];
      });

      // No component classes should be found without dx- prefix
      expect(
        unprefixedClasses,
        `Component ${component} has unprefixed classes: ${unprefixedClasses.join(", ")}`
      ).toHaveLength(0);
    }
  });

  test("all CSS selectors targeting components use dx- prefix", async ({ page }) => {
    const components = [
      "button",
      "checkbox",
      "slider",
      "dialog",
      "accordion",
      "tabs",
      "calendar",
      "select",
      "menubar",
      "tooltip",
    ];

    // Component name patterns that should always be prefixed in CSS selectors
    const componentPattern =
      /\.(button|checkbox|slider|dialog|accordion|tabs|calendar|select|menubar|tooltip|avatar|badge|card|sheet|sidebar|popover|progress|radio|switch|textarea|toggle)(?:-[a-z][\w-]*)?(?=[^a-zA-Z\w-]|$)/;

    for (const component of components) {
      await page.goto(`http://127.0.0.1:8080/component/?name=${component}&`, {
        timeout: 20 * 60 * 1000,
      });

      await page.waitForTimeout(1000);

      const unprefixedSelectors = await page.evaluate((pattern) => {
        const re = new RegExp(pattern);
        const dxRe = /\.dx-/;
        const problematic: string[] = [];

        for (const sheet of Array.from(document.styleSheets)) {
          let rules: CSSRuleList;
          try {
            rules = sheet.cssRules;
          } catch {
            // Cross-origin stylesheet, skip
            continue;
          }

          const checkRule = (rule: CSSRule) => {
            if (rule instanceof CSSMediaRule || rule instanceof CSSSupportsRule) {
              for (const child of Array.from(rule.cssRules)) {
                checkRule(child);
              }
              return;
            }
            if (!(rule instanceof CSSStyleRule)) return;

            // Split compound selectors and check each part
            const selectors = rule.selectorText.split(",");
            for (const sel of selectors) {
              const trimmed = sel.trim();
              // Skip selectors that already use dx- prefix
              if (dxRe.test(trimmed)) continue;
              // Flag selectors that match bare component class names
              if (re.test(trimmed)) {
                problematic.push(rule.selectorText.trim());
              }
            }
          };

          for (const rule of Array.from(rules)) {
            checkRule(rule);
          }
        }

        return [...new Set(problematic)];
      }, componentPattern.source);

      expect(
        unprefixedSelectors,
        `Component page "${component}" has CSS selectors targeting component classes without dx- prefix:\n${unprefixedSelectors.join("\n")}`
      ).toHaveLength(0);
    }
  });

  test("accordion animations use dx- prefixed keyframes", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=accordion&", {
      timeout: 20 * 60 * 1000,
    });

    // Wait for component to load
    const accordionItem = page.locator(".dx-accordion-item").first();
    await expect(accordionItem).toBeVisible({ timeout: 30000 });

    // Inject conflicting keyframes with unprefixed names
    await page.addStyleTag({
      content: `
        @keyframes accordion-slide-down {
          from { height: 0; }
          to { height: 9999px; }
        }
        @keyframes accordion-slide-up {
          from { height: 9999px; }
          to { height: 0; }
        }
      `,
    });

    // Open accordion
    await accordionItem.locator("button").click();
    await expect(accordionItem).toHaveAttribute("data-open", "true");

    // The accordion content should animate properly despite conflicting keyframes
    const content = accordionItem.locator(".dx-accordion-content").first();
    await expect(content).toBeVisible();

    // Verify the height is reasonable (not 9999px from the injected keyframes)
    const height = await content.evaluate((el) => el.offsetHeight);
    expect(height).toBeLessThan(500);
    expect(height).toBeGreaterThan(0);
  });
});
